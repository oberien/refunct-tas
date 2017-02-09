use std::io::Write;
use std::sync::mpsc::{self, Sender, Receiver};
use std::net::{TcpListener, TcpStream};

use byteorder::{ReadBytesExt, LittleEndian};

use error::*;
use statics::{SENDER, RECEIVER};

pub enum Event {
    Stop,
    Step,
    Continue,
    Press(i32),
    Release(i32),
    Mouse(i32, i32),
    SetDelta(f64),
}

pub fn main_loop() -> Result<()> {
    let listener = match TcpListener::bind("localhost:21337") {
        Ok(l) => l,
        Err(err) => {
            log!("Cannot bind TcpListener: {:?}", err);
            return Ok(());
        }
    };
    loop {
        // setup channels
        let (tx, rx) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        RECEIVER.set(rx);
        SENDER.set(tx2);
        let con = match listener.accept() {
            Ok((con, addr)) => {
                log!("Got connection from {}", addr);
                con
            },
            Err(err) => {
                log!("Cannot accept connection: {:?}", err);
                return Ok(());
            }
        };
        handler_loop(con, tx, rx2)?;
    }
}

pub fn handler_loop(mut con: TcpStream, tx: Sender<Event>, rx: Receiver<()>) -> Result<()> {
    loop {
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
                return Ok(());
            }
        }
        con.write_all(&[0])?;
    }
}
