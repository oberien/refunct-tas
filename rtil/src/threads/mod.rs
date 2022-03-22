mod listener;
mod stream_read;
mod stream_write;
mod rebo;
pub mod ue;

use crate::native::AMyCharacter;

pub fn start() {
    let (stream_rebo_tx, stream_rebo_rx) = crossbeam_channel::unbounded();
    let (rebo_stream_tx, rebo_stream_rx) = crossbeam_channel::unbounded();
    let (rebo_ue_tx, rebo_ue_rx) = crossbeam_channel::unbounded();
    let (ue_rebo_tx, ue_rebo_rx) = crossbeam_channel::unbounded();
    listener::run(stream_rebo_tx, rebo_stream_rx).unwrap();
    rebo::run(stream_rebo_rx, rebo_stream_tx, rebo_ue_tx, ue_rebo_rx);
    ue::run(rebo_ue_rx, ue_rebo_tx);
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
    Start(String),
    Stop,
    WorkingDir(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReboToStream {
    Print(String),
    MiDone,
}

#[derive(Debug, PartialEq)]
pub enum ReboToUe {
    Stop,
    AdvanceFrame,
    // we need to execute events on the main loop, because possible played audio
    // needs to access thread-local storage
    PressKey(i32, u32, bool),
    ReleaseKey(i32, u32, bool),
    MoveMouse(i32, i32),
    DrawLine(f32, f32, f32, f32, (f32, f32, f32, f32), f32),
    DrawText(String, (f32, f32, f32, f32), f32, f32, f32, bool),
    Resume,
    /// x, y, z, pitch, yaw, roll
    SpawnAMyCharacter(f32, f32, f32, f32, f32, f32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum UeToRebo {
    Tick,
    NewGame,
    KeyDown(i32, u32, bool),
    KeyUp(i32, u32, bool),
    DrawHud,
    AMyCharacterSpawned(AMyCharacter),
}
