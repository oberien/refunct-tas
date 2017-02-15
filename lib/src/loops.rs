use std::thread;
use std::collections::HashSet;
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
        log!("Setting up channels");
        // channel to send Events to the main thread
        let (mainsender_tx, mainsender_rx) = mpsc::channel();
        // channel to receive Responses from the main thread
        let (mainreceiver_tx, mut mainreceiver_rx) = mpsc::channel();

        RECEIVER.set(mainsender_rx);
        SENDER.set(mainreceiver_tx);
        log!("Channels set up");

        loop {
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
            let mut con2 = con.try_clone().unwrap();

            // clear all events that happened before the client connected
            while let Ok(_) = mainreceiver_rx.try_recv() {}

            // channel to receive the "Stopped" event from the channel thread
            let (channel_tx, channel_rx) = mpsc::channel();
            // channel to inform the channel thread to stop
            let (stop_tx, stop_rx) = mpsc::channel();
            // We can't read from the TcpStream and main-thread channel at the
            // same time, so we create a thread to handle the main-thread channel.
            let channel_thread = thread::spawn(move || {
                {
                    let rx = &mainreceiver_rx;
                    loop {
                        select! {
                            res = rx.recv() => match res.unwrap() {
                                Response::NewGame => con2.write_all(&[1]).unwrap(),
                                Response::Stopped => channel_tx.send(()).unwrap(),
                            },
                            _ = stop_rx.recv() => break
                        }
                    }
                }
                mainreceiver_rx
            });

            // setup HashSet containing all currently pressed keys to clean up in the end
            let mut keys = HashSet::new();
            match handler_loop(con, &mainsender_tx, &channel_rx, &mut keys) {
                Ok(_) => log!("Handler Loop finished successful"),
                Err(err) => log!("Handler Loop experienced an error: {:?}", err)
            }

            // inform the channel thread to stop and wait for it
            stop_tx.send(()).unwrap();
            mainreceiver_rx = channel_thread.join().unwrap();

            // If an error happened or it's finished, clean up
            mainsender_tx.send(Event::SetDelta(0.0)).unwrap();
            for key in keys.iter().cloned() {
                mainsender_tx.send(Event::Release(key)).unwrap();
            }
            mainsender_tx.send(Event::Continue).unwrap();
        }
    });
    Ok(())
}

pub fn handler_loop(mut con: TcpStream, tx: &Sender<Event>, rx: &Receiver<()>, keys: &mut HashSet<i32>) -> Result<()> {
    let mut stopping = false;
    loop {
        // if we are stopping, wait for the next tick
        if stopping {
            rx.recv()?;
        }

        let cmd = con.read_u8()?;
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
                keys.insert(key);
                tx.send(Event::Press(key)).chain_err(|| "error during send")?;
            },
            4 => {
                let key = con.read_i32::<LittleEndian>()?;
                keys.remove(&key);
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
