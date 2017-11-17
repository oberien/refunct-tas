use std::sync::mpsc::Receiver;
use std::net::TcpStream;
use std::io::Write;
use std::thread::{self, JoinHandle};

use byteorder::{WriteBytesExt, LittleEndian};

use threads::{ListenerToStream, LuaToStream};
use error::*;

struct StreamWrite {
    con: TcpStream,
    listener_stream_rx: Receiver<ListenerToStream>,
    lua_stream_rx: Receiver<LuaToStream>,
}

pub fn run(con: TcpStream, listener_stream_rx: Receiver<ListenerToStream>,
           lua_stream_rx: Receiver<LuaToStream>) -> JoinHandle<Receiver<LuaToStream>> {
    let mut stream = StreamWrite {
        con,
        listener_stream_rx,
        lua_stream_rx,
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
        let lua_stream_rx = self.lua_stream_rx;
        let listener_stream_rx = self.listener_stream_rx;
        select! {
            res = lua_stream_rx.recv() => match res.unwrap() {
                LuaToStream::Print(s) => {
                    self.con.write_u8(0)?;
                    self.con.write_u32::<LittleEndian>(s.len() as u32)?;
                    self.con.write_all(s.as_bytes())?;
                }
                LuaToStream::ImDone => self.con.write_u8(1)?,
            },
            res = listener_stream_rx.recv() => match res.unwrap() {
                ListenerToStream::KillYourself => return Err("We should kill ourselves".into())
            }
        }
        Ok(())
    }

    fn die(self) -> Receiver<LuaToStream> {
        self.lua_stream_rx
    }
}
