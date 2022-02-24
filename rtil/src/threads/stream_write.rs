use std::net::TcpStream;
use std::io::Write;
use std::thread::{self, JoinHandle};

use byteorder::{WriteBytesExt, LittleEndian};
use crossbeam_channel::{select, Receiver};

use threads::{ListenerToStream, ReboToStream};
use error::*;

struct StreamWrite {
    con: TcpStream,
    listener_stream_rx: Receiver<ListenerToStream>,
    rebo_stream_rx: Receiver<ReboToStream>,
}

pub fn run(con: TcpStream, listener_stream_rx: Receiver<ListenerToStream>,
           rebo_stream_rx: Receiver<ReboToStream>) -> JoinHandle<Receiver<ReboToStream>> {
    let mut stream = StreamWrite {
        con,
        listener_stream_rx,
        rebo_stream_rx,
    };
    thread::spawn(move || {
        loop {
            match stream.recv_and_write() {
                Ok(()) => {},
                Err(e) => {
                    log!("Got error during `recv_and_write`: {:?}", e);
                    return stream.die();
                }
            }
        }
    })
}

impl StreamWrite {
    fn recv_and_write(&mut self) -> Result<()> {
        let rebo_stream_rx = &self.rebo_stream_rx;
        let listener_stream_rx = &self.listener_stream_rx;
        select! {
            recv(rebo_stream_rx) -> res => match res.unwrap() {
                ReboToStream::Print(s) => {
                    self.con.write_u8(0)?;
                    self.con.write_u32::<LittleEndian>(s.len() as u32)?;
                    self.con.write_all(s.as_bytes())?;
                    self.con.flush()?;
                }
                ReboToStream::MiDone => {
                    log!("Writing done to socket.");
                    self.con.write_u8(1)?;
                }
            },
            recv(listener_stream_rx) -> res => match res.unwrap() {
                ListenerToStream::KillYourself => return Err("We should kill ourselves".into())
            }
        }
        Ok(())
    }

    fn die(self) -> Receiver<ReboToStream> {
        self.rebo_stream_rx
    }
}
