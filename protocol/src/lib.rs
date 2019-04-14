use std::io::{Read, Write, Result, Error, ErrorKind};

use byteorder::{WriteBytesExt, ReadBytesExt, NetworkEndian};

pub type PlayerId = u32;

#[derive(Clone, Debug)]
pub enum Message {
    /// name, x, y, z
    JoinRoom(String, f32, f32, f32),
    /// id, x, y, z
    PlayerJoinedRoom(PlayerId, f32, f32, f32),
    PlayerLeftRoom(PlayerId),
    /// x, y, z
    MoveSelf(f32, f32, f32),
    /// id, x, y, z
    MoveOther(PlayerId, f32, f32, f32),
}

impl Message {
    pub fn serialize<W: Write>(self, mut w: W) -> Result<()> {
        match self {
            Message::JoinRoom(room, x, y, z) => {
                w.write_u8(0)?;
                w.write_u64::<NetworkEndian>(room.len() as u64)?;
                w.write_all(room.as_bytes())?;
                w.write_f32::<NetworkEndian>(x)?;
                w.write_f32::<NetworkEndian>(y)?;
                w.write_f32::<NetworkEndian>(z)?;
            },
            Message::PlayerJoinedRoom(id, x, y, z) => {
                w.write_u8(1)?;
                w.write_u32::<NetworkEndian>(id)?;
                w.write_f32::<NetworkEndian>(x)?;
                w.write_f32::<NetworkEndian>(y)?;
                w.write_f32::<NetworkEndian>(z)?;
            },
            Message::PlayerLeftRoom(id) => {
                w.write_u8(2)?;
                w.write_u32::<NetworkEndian>(id)?;
            },
            Message::MoveSelf(x, y, z) => {
                w.write_u8(3)?;
                w.write_f32::<NetworkEndian>(x)?;
                w.write_f32::<NetworkEndian>(y)?;
                w.write_f32::<NetworkEndian>(z)?;
            }
            Message::MoveOther(id, x, y, z) => {
                w.write_u8(4)?;
                w.write_u32::<NetworkEndian>(id)?;
                w.write_f32::<NetworkEndian>(x)?;
                w.write_f32::<NetworkEndian>(y)?;
                w.write_f32::<NetworkEndian>(z)?;
            }
        }
        Ok(())
    }

    pub fn deserialize<R: Read>(mut r: R) -> Result<Self> {
        let variant = r.read_u8()?;
        let msg = match variant {
            0 => {
                let len = r.read_u64::<NetworkEndian>()? as usize;
                let mut vec = vec![0; len];
                r.read_exact(&mut vec)?;
                let room = String::from_utf8(vec)
                    .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
                let x = r.read_f32::<NetworkEndian>()?;
                let y = r.read_f32::<NetworkEndian>()?;
                let z = r.read_f32::<NetworkEndian>()?;
                Message::JoinRoom(room, x, y, z)
            },
            1 => {
                let id = r.read_u32::<NetworkEndian>()?;
                let x = r.read_f32::<NetworkEndian>()?;
                let y = r.read_f32::<NetworkEndian>()?;
                let z = r.read_f32::<NetworkEndian>()?;
                Message::PlayerJoinedRoom(id, x, y, z)
            },
            2 => {
                let id = r.read_u32::<NetworkEndian>()?;
                Message::PlayerLeftRoom(id)
            },
            3 => {
                let x = r.read_f32::<NetworkEndian>()?;
                let y = r.read_f32::<NetworkEndian>()?;
                let z = r.read_f32::<NetworkEndian>()?;
                Message::MoveSelf(x, y, z)
            },
            4 => {
                let id = r.read_u32::<NetworkEndian>()?;
                let x = r.read_f32::<NetworkEndian>()?;
                let y = r.read_f32::<NetworkEndian>()?;
                let z = r.read_f32::<NetworkEndian>()?;
                Message::MoveOther(id, x, y, z)
            },
            _ => return Err(Error::new(ErrorKind::InvalidData, "unknown message variant")),
        };
        Ok(msg)
    }
}
