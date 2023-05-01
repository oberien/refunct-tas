use std::sync::atomic::{AtomicU32, Ordering};
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PlayerId(u32);

// first 1000 ids are for anyone's internal hackery use
static PLAYER_ID_SERIAL: AtomicU32 = AtomicU32::new(1000);

impl PlayerId {
    pub fn next() -> PlayerId {
        PlayerId(PLAYER_ID_SERIAL.fetch_add(1, Ordering::SeqCst))
    }

    pub fn id(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Request {
    GetServerTime,
    /// room-name, player-name, x, y, z, pitch, yaw, roll
    JoinRoom(String, String, f32, f32, f32, f32, f32, f32),
    /// x, y, z, pitch, yaw, roll
    MoveSelf(f32, f32, f32, f32, f32, f32),
    /// platform-id
    PressPlatform(u8),
    /// button-id
    PressButton(u8),
    NewGamePressed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Response {
    /// unix timestamp
    ServerTime(u64),
    /// id, name, x, y, z, pitch, yaw, roll
    PlayerJoinedRoom(PlayerId, String, f32, f32, f32, f32, f32, f32),
    PlayerLeftRoom(PlayerId),
    /// id, x, y, z, pitch, yaw, roll
    MoveOther(PlayerId, f32, f32, f32, f32, f32, f32),
    /// platform-id
    PressPlatform(u8),
    /// button-id
    PressButton(u8),
    NewGamePressed(PlayerId),
    /// unix timestamp
    StartNewGameAt(u64),
    RoomNameTooLong,
}
