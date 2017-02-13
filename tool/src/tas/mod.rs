mod parser;

pub use self::parser::Frame;
//pub use self::parser::parse_lines;

use std::net::TcpStream;
use std::io::{Read, BufRead, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

use error::*;
use config::Config;

pub struct Tas {
    con: TcpStream,
}

impl Tas {
    pub fn new() -> Result<Tas> {
        let con = TcpStream::connect("localhost:21337")?;
        Ok(Tas {
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

    pub fn stop(&mut self) -> Result<()> {
        self.con.write_u8(0)?;
        self.read_result_until_success().chain_err(|| "Error executing stop")?;
        Ok(())
    }

    pub fn step(&mut self) -> Result<u8> {
        self.con.write_u8(1)?;
        self.read_result()
    }

    pub fn cont(&mut self) -> Result<()> {
        self.con.write_u8(2)?;
        self.read_result_until_success().chain_err(|| "Error executing cont")?;
        Ok(())
    }

    pub fn press_key(&mut self, key: char) -> Result<()> {
        self.con.write_u8(3)?;
        self.con.write_i32::<LittleEndian>(key as i32)?;
        self.read_result_until_success().chain_err(|| "Error executing press_key")?;
        Ok(())
    }

    pub fn release_key(&mut self, key: char) -> Result<()> {
        self.con.write_u8(4)?;
        self.con.write_i32::<LittleEndian>(key as i32)?;
        self.read_result_until_success().chain_err(|| "Error executing release_key")?;
        Ok(())
    }

    pub fn move_mouse(&mut self, x: i32, y: i32) -> Result<()> {
        self.con.write_u8(5)?;
        self.con.write_i32::<LittleEndian>(x)?;
        self.con.write_i32::<LittleEndian>(y)?;
        self.read_result_until_success().chain_err(|| "Error executing move_mouse")?;
        Ok(())
    }

    pub fn set_delta(&mut self, delta: f64) -> Result<()> {
        self.con.write_u8(6)?;
        self.con.write_f64::<LittleEndian>(delta)?;
        self.read_result_until_success().chain_err(|| "Error executing set_delta")?;
        Ok(())
    }

    pub fn wait_for_new_game(&mut self) -> Result<()> {
        self.stop()?;
        loop {
            let res = self.step()?;
            if res == 1 {
                break;
            }
        }
        Ok(())
    }

    fn read_result(&mut self) -> Result<u8> {
        let res = self.con.read_u8()?;
        // no error → early return
        if res != 255 {
            return Ok(res);
        }
        let code = self.con.read_u8()?;
        match code {
            0 => Err(ErrorKind::UnknownCommand.into()),
            _ => unimplemented!()
        }
    }

    fn read_result_until_success(&mut self) -> Result<()> {
        while self.read_result()? != 0 {}
        Ok(())
    }

    pub fn play(&mut self, frames: &Vec<Frame>, inputs: &Config) -> Result<()> {
        self.stop()?;
        self.set_delta(1.0/60.0)?;
        let mut last = Frame::default();
        for frame in frames {
            // new inputs
            if frame.forward && !last.forward {
                self.press_key(inputs.forward)?;
            }
            if frame.backward && !last.backward {
                self.press_key(inputs.backward)?;
            }
            if frame.left && !last.left {
                self.press_key(inputs.left)?;
            }
            if frame.right && !last.right {
                self.press_key(inputs.right)?;
            }
            if frame.jump && !last.jump {
                self.press_key(inputs.jump)?;
            }

            // old inputs
            if last.forward && !frame.forward {
                self.release_key(inputs.forward)?;
            }
            if last.backward && !frame.backward {
                self.release_key(inputs.backward)?;
            }
            if last.left && !frame.left {
                self.release_key(inputs.left)?;
            }
            if last.right && !frame.right {
                self.release_key(inputs.right)?;
            }
            if last.jump && !frame.jump {
                self.release_key(inputs.jump)?;
            }

            last = frame.clone();

            // press ESC
            if frame.esc {
                 self.press_key(0x1b as char)?;
            }

            // mouse movements
            if frame.mouse_x != 0 || frame.mouse_y != 0 {
                 self.move_mouse(frame.mouse_x, frame.mouse_y)?;
            }

            self.step()?;
        }
        self.set_delta(0.0)?;
        self.cont()?;
        Ok(())
    }
}

