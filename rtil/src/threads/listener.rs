use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};
use std::net::TcpListener;
use std::io::Write;
use std::thread;

use threads::{StreamToListener, StreamToLua, LuaToStream};
use error::*;

pub fn run(stream_listener_rx: Receiver<StreamToListener>, stream_lua_tx: Sender<StreamToLua>,
           lua_stream_rx: Receiver<LuaToStream>) -> Result<()> {
    let listener = TcpListener::bind("localhost:21337")?;

    thread::spawn(move || {
        let mut stream_handle = None;
        while let Ok((mut stream, _)) = listener.accept() {
            match stream_listener_rx.try_recv() {
                Ok(StreamToListener::ImDead) => {
                    let _ = stream.write_all(&[255, 1]);
                    continue;
                }
                Err(TryRecvError::Empty) => {},
                Err(e) => {
                    log!("Error receiving stream_listener: {:?}", e);
                    panic!();
                }
            }

            let (stream_lua_tx, stream_lua_rx) = mpsc::channel();
            let (lua_stream_tx, lua_stream_rx) = mpsc::channel();

            let (stream_lua_tx, lua_stream_rx) = stream_handle.map(|handle| handle.join())
                .unwrap_or((stream_lua_tx, lua_stream_rx));


        }
    });
    Ok(())
}
