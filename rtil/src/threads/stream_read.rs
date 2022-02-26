use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread::{self, JoinHandle};

use byteorder::{ReadBytesExt, LittleEndian};
use crossbeam_channel::Sender;

use crate::threads::{StreamToListener, StreamToRebo, Config};
use crate::error::Result;

struct StreamRead {
    con: TcpStream,
    stream_listener_tx: Sender<StreamToListener>,
    stream_rebo_tx: Sender<StreamToRebo>,
}

pub fn run(con: TcpStream, stream_listener_tx: Sender<StreamToListener>, stream_rebo_tx: Sender<StreamToRebo>)
           -> JoinHandle<Sender<StreamToRebo>> {
    let mut stream = StreamRead {
        con,
        stream_listener_tx,
        stream_rebo_tx,
    };
    thread::spawn(move || {
        loop {
            match stream.handle_cmd() {
                Ok(()) => {},
                Err(e) => {
                    log!("Got error during `handle_cmd`: {:?}", e);
                    return stream.die();
                }
            }
        }
    })
}

impl StreamRead {
    fn handle_cmd(&mut self) -> Result<()> {
        match self.con.read_u8()? {
            0 => {
                log!("Reading code");
                let code = self.read_string()?;
                log!("Got code");
                self.stream_rebo_tx.send(StreamToRebo::Start(code)).unwrap();
            }
            1 => {
                log!("Got stop");
                self.stream_rebo_tx.send(StreamToRebo::Stop).unwrap()
            }
            2 => {
                log!("Reading Config...");
                let config = Config {
                    forward: self.con.read_i32::<LittleEndian>()?,
                    backward: self.con.read_i32::<LittleEndian>()?,
                    left: self.con.read_i32::<LittleEndian>()?,
                    right: self.con.read_i32::<LittleEndian>()?,
                    jump: self.con.read_i32::<LittleEndian>()?,
                    crouch: self.con.read_i32::<LittleEndian>()?,
                    menu: self.con.read_i32::<LittleEndian>()?,
                };
                log!("Got Config: {:?}", config);
                self.stream_rebo_tx.send(StreamToRebo::Config(config)).unwrap();
            }
            3 => {
                log!("Reading working dir");
                let path = self.read_string()?;
                self.stream_rebo_tx.send(StreamToRebo::WorkingDir(path)).unwrap();
            }
            255 => log!("Got Error code from client: {}", self.con.read_u8()?),
            cmd => {
                log!("Client sent invalid command: {}", cmd);
                self.con.write_all(&[255, 0])?;
            }
        }
        Ok(())
    }

    fn read_string(&mut self) -> Result<String> {
        let len = self.con.read_u32::<LittleEndian>()?;
        let mut buf = vec![0u8; len as usize];
        self.con.read_exact(&mut buf)?;
        let string = match String::from_utf8(buf) {
            Ok(code) => code,
            Err(e) => {
                let _ = self.con.write_all(&[255, 2]);
                return Err(e.into());
            }
        };
        Ok(string)
    }

    fn die(self) -> Sender<StreamToRebo> {
        self.stream_rebo_tx.send(StreamToRebo::Stop).unwrap();
        self.stream_listener_tx.send(StreamToListener::ImDead).unwrap();
        self.stream_rebo_tx
    }
}