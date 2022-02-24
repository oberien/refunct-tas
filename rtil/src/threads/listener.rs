use std::net::TcpListener;
use std::io::Write;
use std::thread::{self, JoinHandle};

use crossbeam_channel::{Sender, Receiver, TryRecvError};

use threads::{stream_read, stream_write, StreamToListener, StreamToRebo, ReboToStream, ListenerToStream};
use error::*;

pub fn run(stream_rebo_tx: Sender<StreamToRebo>, rebo_stream_rx: Receiver<ReboToStream>) -> Result<()> {
    log!("starting listener thread");
    let listener = TcpListener::bind("localhost:21337")?;
    let mut stream_rebo_tx = Some(stream_rebo_tx);
    let mut rebo_stream_rx = Some(rebo_stream_rx);
    let (mut listener_stream_tx, listener_stream_rx) = crossbeam_channel::unbounded();
    let mut listener_stream_rx = Some(listener_stream_rx);
    let (stream_listener_tx, mut stream_listener_rx) = crossbeam_channel::unbounded();
    let mut stream_listener_tx = Some(stream_listener_tx);

    thread::spawn(move || {
        let mut stream_read_thread: Option<JoinHandle<Sender<StreamToRebo>>> = None;
        let mut stream_write_thread: Option<JoinHandle<Receiver<ReboToStream>>> = None;

        // make first iteration work
        stream_listener_tx.as_ref().unwrap().send(StreamToListener::ImDead).unwrap();

        while let Ok((mut con, _)) = listener.accept() {
            log!("Got new connection from {:?}", con.peer_addr());
            match stream_listener_rx.try_recv() {
                Ok(StreamToListener::ImDead) => {}
                Err(TryRecvError::Empty) => {
                    log!("There is already an open connection.");
                    let _ = con.write_all(&[255, 1]);
                    continue;
                },
                Err(e) => {
                    log!("Error receiving stream_listener: {:?}", e);
                    panic!();
                }
            }

            // recover channels from threads and create new ones
            if stream_rebo_tx.is_none() {
                stream_rebo_tx = Some(stream_read_thread.unwrap().join().unwrap());
                // Stream_write could have tried to write to TcpStream and failed, thus already died.
                let _ = listener_stream_tx.send(ListenerToStream::KillYourself);
                rebo_stream_rx = Some(stream_write_thread.unwrap().join().unwrap());
                let (tx, rx) = crossbeam_channel::unbounded();
                listener_stream_tx = tx;
                listener_stream_rx = Some(rx);
                let (tx, rx) = crossbeam_channel::unbounded();
                stream_listener_tx = Some(tx);
                stream_listener_rx = rx;
            }

            // clear old data from streams
            while let Ok(_) = rebo_stream_rx.as_ref().unwrap().try_recv() {}

            log!("Starting stream threads.");
            stream_read_thread = Some(stream_read::run(con.try_clone().unwrap(), stream_listener_tx.take().unwrap(), stream_rebo_tx.take().unwrap()));
            stream_write_thread = Some(stream_write::run(con, listener_stream_rx.take().unwrap(), rebo_stream_rx.take().unwrap()));
        }
    });
    Ok(())
}
