use archipelago_rs::protocol::{ClientMessage, ServerMessage};

mod listener;
mod stream_read;
mod stream_write;
mod archipelago;
pub mod ue;

pub fn start() {
    let (stream_rebo_tx, stream_rebo_rx) = crossbeam_channel::unbounded();
    let (rebo_stream_tx, rebo_stream_rx) = crossbeam_channel::unbounded();
    let (archipelago_rebo_tx, archipelago_rebo_rx) = crossbeam_channel::unbounded();
    let (rebo_achipelago_tx, rebo_archipelago_rx) = tokio::sync::mpsc::unbounded_channel();
    listener::run(stream_rebo_tx, rebo_stream_rx).unwrap();
    archipelago::run(archipelago_rebo_tx, rebo_archipelago_rx);
    ue::run(stream_rebo_rx, rebo_stream_tx, archipelago_rebo_rx, rebo_achipelago_tx);
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListenerToStream {
    KillYourself,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StreamToListener {
    ImDead,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StreamToRebo {
    // filename, code
    Start(String, String),
    Stop,
    WorkingDir(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReboToStream {
    Print(String),
    MiDone,
}

#[derive(Debug)]
pub enum ArchipelagoToRebo {
    ServerMessage(ServerMessage),
    ConnectionAborted,
}
#[derive(Debug)]
pub enum ReboToArchipelago {
    Connect {
        server_and_port: String,
        game: String,
        slot: String,
        password: Option<String>,
        items_handling: Option<i32>,
        tags: Vec<String>,
    },
    ClientMessage(ClientMessage),
    Disconnect,
}
