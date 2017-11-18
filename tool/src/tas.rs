use std::net::TcpStream;
use std::io::{Read, Write};
use std::path::Path;
use std::fs::File;
use std::env;

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
            con,
        })
    }

    pub fn execute<P: AsRef<Path>>(&mut self, path: P, config: &Config) {
        let path = path.as_ref();
        let mut file = File::open(path).expect(&format!("Couldn't open TAS file {:?}", path));
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        println!("Setting Config");
        self.con.write_u8(2).unwrap();
        self.con.write_i32::<LittleEndian>(config.forward.into()).unwrap();
        self.con.write_i32::<LittleEndian>(config.backward.into()).unwrap();
        self.con.write_i32::<LittleEndian>(config.left.into()).unwrap();
        self.con.write_i32::<LittleEndian>(config.right.into()).unwrap();
        self.con.write_i32::<LittleEndian>(config.jump.into()).unwrap();
        self.con.write_i32::<LittleEndian>(config.crouch.into()).unwrap();
        self.con.write_i32::<LittleEndian>(config.menu.into()).unwrap();

        println!("Setting Environment");
        let current_dir = env::current_dir().unwrap();
        let current_dir = current_dir.canonicalize().unwrap();
        let mut current_dir = current_dir.to_str().unwrap();
        if current_dir.starts_with("\\\\?\\") {
            current_dir = &current_dir[4..];
        }
        println!("Current dir: {}", current_dir);
        self.con.write_u8(3).unwrap();
        self.con.write_u32::<LittleEndian>(current_dir.len() as u32).unwrap();
        self.con.write_all(&current_dir.as_bytes()).unwrap();

        println!("Sending code");
        self.con.write_u8(0).unwrap();
        self.con.write_u32::<LittleEndian>(s.len() as u32).unwrap();
        self.con.write_all(s.as_bytes()).unwrap();
        println!("Tas Execution started");

        loop {
            match self.con.read_u8().unwrap() {
                0 => {
                    let len = self.con.read_u32::<LittleEndian>().unwrap();
                    let mut buf = vec![0u8; len as usize];
                    self.con.read_exact(&mut buf).unwrap();
                    let s = String::from_utf8(buf).unwrap();
                    println!("{}", s);
                }
                1 => {
                    println!("Execution Finished");
                    break;
                }
                255 => match self.con.read_u8().unwrap() {
                    0 => println!("Error: Unknown Command."),
                    1 => println!("Error: There is already a connection to the game. Please close that one first or restart the game."),
                    2 => println!("Error: Invalid data received."),
                    n => println!("Error: Got unknown error number: {}", n),
                }
                n => println!("Error: Got unknown number: {}", n),
            }
        }
    }
}
