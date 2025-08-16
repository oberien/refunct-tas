use crate::native::Hooks;

mod listener;
mod stream_read;
mod stream_write;
pub mod ue;

pub fn start(hooks: Hooks) {
    let (stream_rebo_tx, stream_rebo_rx) = crossbeam_channel::unbounded();
    let (rebo_stream_tx, rebo_stream_rx) = crossbeam_channel::unbounded();
    listener::run(stream_rebo_tx, rebo_stream_rx).unwrap();
    ue::run(stream_rebo_rx, rebo_stream_tx, hooks);
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
    // filenmae, code
    Start(String, String),
    Stop,
    WorkingDir(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReboToStream {
    Print(String),
    MiDone,
}
