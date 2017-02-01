#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate byteorder;

#[cfg(unix)]
extern crate libc;

use std::fs::{OpenOptions, File};
use std::io::Write;
use std::sync::{Mutex, Once, ONCE_INIT};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::net::{TcpListener, TcpStream};

use byteorder::{ReadBytesExt, LittleEndian};

macro_rules! log {
    () => {{
        use ::std::io::Write;
        writeln!(&mut ::LOGFILE.lock().unwrap(), "").unwrap();
    }};
    ($fmt:expr) => {{
        use ::std::io::Write;
        writeln!(&mut ::LOGFILE.lock().unwrap(), $fmt).unwrap()
    }};
    ($fmt:expr, $($vars:tt)*) => {{
        use ::std::io::Write;
        writeln!(&mut ::LOGFILE.lock().unwrap(), $fmt, $($vars)*).unwrap()
    }};
}

mod error;
mod consts;
#[cfg(unix)]
mod linux;
#[cfg(windows)]
mod windows;
#[cfg(unix)]
use linux::*;
#[cfg(windows)]
use windows::*;
use error::*;

lazy_static! {
    static ref LOGFILE: Mutex<File> = Mutex::new(OpenOptions::new()
        .create(true).write(true)
        .open("/tmp/refunct-tas.log").unwrap());
    static ref SLATEAPP: Mutex<usize> = Mutex::new(0);
    static ref RECEIVER: Mutex<Option<Receiver<Event>>> = Mutex::new(None);
    static ref SENDER: Mutex<Option<Sender<()>>> = Mutex::new(None);
}

enum Event {
    Stop,
    Step,
    Continue,
    Press(i32),
    Release(i32),
    Mouse(i32, i32),
    SetDelta(f64),
}

static INIT: Once = ONCE_INIT;

pub extern fn initialize() {
    INIT.call_once(|| {
        let exe = ::std::env::current_exe().unwrap();
        let file = exe.file_name().unwrap();
        if file == "Refunct-Linux-Shipping" {
            // hook stuff
            hook_slateapp();
            // setup channels
            let (tx, rx) = mpsc::channel();
            let (tx2, rx2) = mpsc::channel();
            *RECEIVER.lock().unwrap() = Some(rx);
            *SENDER.lock().unwrap() = Some(tx2);
            thread::spawn(|| {
                match main_loop(tx, rx2) {
                    Ok(_) => log!("Main Loop finished successful, exiting..."),
                    Err(err) => log!("Main Loop experienced an error: {:?}", err)
                }
            });
        }
    });
}

fn main_loop(tx: Sender<Event>, rx: Receiver<()>) -> Result<()> {
    let listener = match TcpListener::bind("localhost:21337") {
        Ok(l) => l,
        Err(err) => {
            log!("Cannot bind TcpListener: {:?}", err);
            return Ok(());
        }
    };
    loop {
        let mut con = match listener.accept() {
            Ok((con, addr)) => {
                log!("Got connection from {}", addr);
                con
            },
            Err(err) => {
                log!("Cannot accept connection: {:?}", err);
                return Ok(());
            }
        };
        writeln!(&mut con, "slateapp: {:#x}", *SLATEAPP.lock().unwrap()).unwrap();
        let cmd = con.read_u8()?;
        match cmd {
            0 => {
                tx.send(Event::Stop).chain_err(|| "error during send")?;
            },
            1 => {
                tx.send(Event::Step).chain_err(|| "error during send")?;
            },
            2 => {
                tx.send(Event::Continue).chain_err(|| "error during send")?;
            },
            3 => {
                let key = con.read_i32::<LittleEndian>()?;
                tx.send(Event::Press(key)).chain_err(|| "error during send")?;
            },
            4 => {
                let key = con.read_i32::<LittleEndian>()?;
                tx.send(Event::Release(key)).chain_err(|| "error during send")?;
            },
            5 => {
                let x = con.read_i32::<LittleEndian>()?;
                let y = con.read_i32::<LittleEndian>()?;
                tx.send(Event::Mouse(x, y)).chain_err(|| "error during send")?;
            },
            6 => {
                let delta = con.read_f64::<LittleEndian>()?;
                tx.send(Event::SetDelta(delta)).chain_err(|| "error during send")?;
            },
            _ => {
                con.write_all(&[255])?;
            }
        }
        con.write_all(&[0])?;
    }
}
