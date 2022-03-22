use std::sync::atomic::{AtomicU32, Ordering};
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PlayerId(u32);

static PLAYER_ID_SERIAL: AtomicU32 = AtomicU32::new(0);

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
    /// room-name, player-name, x, y, z
    JoinRoom(String, String, f32, f32, f32),
    /// x, y, z
    MoveSelf(f32, f32, f32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Response {
    /// id, name, x, y, z
    PlayerJoinedRoom(PlayerId, String, f32, f32, f32),
    PlayerLeftRoom(PlayerId),
    /// id, x, y, z
    MoveOther(PlayerId, f32, f32, f32),
}
