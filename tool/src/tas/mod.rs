use std::net::TcpStream;
use std::io::{Read, BufRead, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use hlua;

use error::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Response {
    Stopped(PlayerStats),
    NewGame,
}

pub struct Tas {
    buf: Vec<u8>,
    con: TcpStream,
}

impl Tas {
    pub fn new() -> Result<Tas> {
        let con = TcpStream::connect("localhost:21337")?;
        Ok(Tas {
            buf: Vec::new(),
            con: con,
        })
    }

    #[allow(unused)]
    pub fn test_loop(&mut self) -> Result<()> {
        let mut con2 = self.con.try_clone().unwrap();
        ::std::thread::spawn(move || {
            let stdin = ::std::io::stdin();
            for line in stdin.lock().lines() {
                let byte: u8 = line.unwrap().parse().unwrap();
                con2.write_all(&[byte]).unwrap();
            }
        });
        loop {
            let mut buf = [0u8; 1];
            self.con.read_exact(&mut buf)?;
            println!("received {:?}", buf);
        }
    }

    pub fn stop(&mut self) -> Result<PlayerStats> {
        self.con.write_u8(0)?;
        self.read_stats()
    }

    pub fn step(&mut self) -> Result<Response> {
        self.buf.write_u8(1)?;
        self.con.write_all(&self.buf)?;
        self.con.flush()?;
        self.buf.clear();
        self.read_response()
    }

    pub fn cont(&mut self) -> Result<()> {
        self.con.write_u8(2)?;
        Ok(())
    }

    pub fn press_key(&mut self, key: i32) -> Result<()> {
        self.buf.write_u8(3)?;
        self.buf.write_i32::<LittleEndian>(key)?;
        Ok(())
    }

    pub fn release_key(&mut self, key: i32) -> Result<()> {
        self.buf.write_u8(4)?;
        self.buf.write_i32::<LittleEndian>(key)?;
        Ok(())
    }

    pub fn move_mouse(&mut self, x: i32, y: i32) -> Result<()> {
        self.buf.write_u8(5)?;
        self.buf.write_i32::<LittleEndian>(x)?;
        self.buf.write_i32::<LittleEndian>(y)?;
        Ok(())
    }

    pub fn set_delta(&mut self, delta: f64) -> Result<()> {
        self.buf.write_u8(6)?;
        self.buf.write_f64::<LittleEndian>(delta)?;
        Ok(())
    }

    pub fn set_location(&mut self, x: f32, y: f32, z: f32) -> Result<()> {
        self.buf.write_u8(8)?;
        self.buf.write_f32::<LittleEndian>(x)?;
        self.buf.write_f32::<LittleEndian>(y)?;
        self.buf.write_f32::<LittleEndian>(z)?;
        Ok(())
    }

    pub fn set_rotation(&mut self, pitch: f32, yaw: f32, roll: f32) -> Result<()> {
        self.buf.write_u8(7)?;
        self.buf.write_f32::<LittleEndian>(pitch)?;
        self.buf.write_f32::<LittleEndian>(yaw)?;
        self.buf.write_f32::<LittleEndian>(roll)?;
        Ok(())
    }

    pub fn set_velocity(&mut self, x: f32, y: f32, z: f32) -> Result<()> {
        self.buf.write_u8(9)?;
        self.buf.write_f32::<LittleEndian>(x)?;
        self.buf.write_f32::<LittleEndian>(y)?;
        self.buf.write_f32::<LittleEndian>(z)?;
        Ok(())
    }

    pub fn wait_for_new_game(&mut self) -> Result<PlayerStats> {
        while self.step()? != Response::NewGame {}
        self.read_stats()
    }

    fn read_stats(&mut self) -> Result<PlayerStats> {
        match self.read_response()? {
            Response::NewGame => unreachable!(),
            Response::Stopped(stats) => Ok(stats),
        }
    }

    fn read_response(&mut self) -> Result<Response> {
        let code = self.con.read_u8()?;
        match code {
            0 => {
                let x = self.con.read_f32::<LittleEndian>()?;
                let y = self.con.read_f32::<LittleEndian>()?;
                let z = self.con.read_f32::<LittleEndian>()?;
                let pitch = self.con.read_f32::<LittleEndian>()?;
                let yaw = self.con.read_f32::<LittleEndian>()?;
                let roll = self.con.read_f32::<LittleEndian>()?;
                let velx = self.con.read_f32::<LittleEndian>()?;
                let vely = self.con.read_f32::<LittleEndian>()?;
                let velz = self.con.read_f32::<LittleEndian>()?;
                let accx = self.con.read_f32::<LittleEndian>()?;
                let accy = self.con.read_f32::<LittleEndian>()?;
                Ok(Response::Stopped(PlayerStats {
                    x, y, z,
                    pitch, yaw, roll,
                    velx, vely, velz,
                    accx, accy
                }))
            },
            1 => Ok(Response::NewGame),
            255 => {
                let code = self.con.read_u8()?;
                match code {
                    0 => Err(ErrorKind::UnknownCommand.into()),
                    _ => panic!("not yet implemented: Code: {}", code)
                }
            }
            _ => panic!("not yet implemented: Code: {}", code)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerStats {
    x: f32,
    y: f32,
    z: f32,
    pitch: f32,
    yaw: f32,
    roll: f32,
    velx: f32,
    vely: f32,
    velz: f32,
    accx: f32,
    accy: f32,
}

impl<'lua, L> hlua::Push<L> for PlayerStats where L: hlua::AsMutLua<'lua> {
    type Err = hlua::Void;

    fn push_to_lua(self, lua: L) -> ::std::result::Result<hlua::PushGuard<L>, (Self::Err, L)> {
        let stats = self.clone();
        Ok(hlua::push_userdata(self, lua, move |mut metatable| {
            metatable.set("x", stats.x);
            metatable.set("y", stats.y);
            metatable.set("z", stats.z);
            metatable.set("pitch", stats.pitch);
            metatable.set("yaw", stats.yaw);
            metatable.set("roll", stats.roll);
            metatable.set("velx", stats.velx);
            metatable.set("vely", stats.vely);
            metatable.set("velz", stats.velz);
            metatable.set("accx", stats.accx);
            metatable.set("accy", stats.accy);
        }))
    }
}
