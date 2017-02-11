use std::thread;
use std::io::Write;
use std::sync::mpsc::{self, Sender, Receiver};
use std::net::{TcpListener, TcpStream};

use byteorder::{ReadBytesExt, LittleEndian};

use error::*;
use statics::{SENDER, RECEIVER};

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Stop,
    Step,
    Continue,
    Press(i32),
    Release(i32),
    Mouse(i32, i32),
    SetDelta(f64),
}

pub enum Response {
    Stopped,
    NewGame,
}

pub fn main_loop() -> Result<()> {
    log!("Starting TCPListener");
    let listener = match TcpListener::bind("localhost:21337") {
        Ok(l) => l,
        Err(err) => {
            return Err(err).chain_err(|| format!("Cannot bind TcpListener"));
        }
    };
    log!("TCPListener is listening");
    thread::spawn(move || {
        loop {
            log!("Setting up channels");
            // setup channels
            let (tx, rx) = mpsc::channel();
            let (tx2, rx2) = mpsc::channel();
            RECEIVER.set(rx);
            SENDER.set(tx2);
            log!("Channels set up");
            let con = match listener.accept() {
                Ok((con, addr)) => {
                    log!("Got connection from {}", addr);
                    con
                },
                Err(err) => {
                    log!("Cannot accept connection: {:?}", err);
                    return;
                }
            };
            // clear all events that happened before the client connected
            while let Ok(_) = rx2.try_recv() {}
            match handler_loop(con, tx, rx2) {
                Ok(_) => log!("Handler Loop finished successful"),
                Err(err) => log!("Handler Loop experienced an error: {:?}", err)
            }
        }
        unreachable!();
    });
    Ok(())
}

pub fn handler_loop(mut con: TcpStream, tx: Sender<Event>, rx: Receiver<Response>) -> Result<()> {
    let mut stopping = false;
    let (contx, conrx) = mpsc::channel();
    let (stoptx, stoprx) = mpsc::channel();
    let mut con2 = con.try_clone().unwrap();
    // channel-thread
    thread::spawn(move || {
        loop {
            select! {
                res = rx.recv() => match res.unwrap() {
                    Response::NewGame => con2.write_all(&[1]).unwrap(),
                    Response::Stopped => contx.send(()).unwrap(),
                },
                _ = stoprx.recv() => return
            }
        }
    });
    loop {
        // if we are stopping, wait for the next tick
        if stopping {
            conrx.recv()?;
        }
        // if the TcpStream has an error, inform the channel-thread
        let cmd = match con.read_u8() {
            Ok(val) => val,
            Err(err) => {
                stoptx.send(()).unwrap();
                return Err(err.into());
            }
        };
        match cmd {
            0 => {
                tx.send(Event::Stop).chain_err(|| "error during send")?;
                stopping = true;
            },
            1 => {
                tx.send(Event::Step).chain_err(|| "error during send")?;
            },
            2 => {
                tx.send(Event::Continue).chain_err(|| "error during send")?;
                stopping = false;
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
                con.write_all(&[255, 0])?;
                return Ok(());
            }
        }
        con.write_all(&[0])?;
    }
}
