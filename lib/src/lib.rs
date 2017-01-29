#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]

#[macro_use]
extern crate lazy_static;
#[cfg(unix)]
extern crate libc;

use std::fs::{OpenOptions, File};
use std::io::Write;
use std::sync::{Mutex, Once, ONCE_INIT};
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::Result;

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

mod consts;
#[cfg(unix)]
mod linux;
#[cfg(windows)]
mod windows;
#[cfg(unix)]
use linux::*;
#[cfg(windows)]
use windows::*;

lazy_static! {
    static ref LOGFILE: Mutex<File> = Mutex::new(OpenOptions::new()
        .create(true).write(true)
        .open("/tmp/refunct-tas.log").unwrap());
    static ref SLATEAPP: Mutex<usize> = Mutex::new(0);
    static ref INPUTS: Mutex<Vec<Event>> = Mutex::new(Vec::new());
}

enum Event {
    Press(u8),
    Release(u8),
    Mouse(i32, i32),
}

static INIT: Once = ONCE_INIT;

extern fn initialize() {
    INIT.call_once(|| {
        let exe = ::std::env::current_exe().unwrap();
        let file = exe.file_name().unwrap();
        if file == "Refunct-Linux-Shipping" {
            init();
            thread::spawn(|| {
                match main_loop() {
                    Ok(_) => log!("Main Loop finished successful, exiting..."),
                    Err(err) => log!("Main Loop experienced an error: {:?}", err)
                }
            });
        }
    });
}

fn main_loop() -> Result<()> {
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
        let buf = [0; 1];
    }
}

fn init() -> Result<()> {
    hook_slateapp();
    Ok(())
}
