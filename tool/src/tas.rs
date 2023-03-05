use std::net::TcpStream;
use std::io::{Read, Write};
use std::path::Path;
use std::fs::File;
use std::env;
use std::{thread, time};
use discord_rich_presence::{
    activity::{self, Activity},
    DiscordIpc, DiscordIpcClient,
};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::time::Duration;

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

use crate::error::{Error, Result};

pub struct Tas {
    con: TcpStream,
}

impl Tas {
    pub fn new() -> Result<Tas> {
        let con = TcpStream::connect("localhost:21337")
            .map_err(|_ | Error::CantConnectToRtil)?;
        Ok(Tas {
            con,
        })
    }

    pub fn execute<P: AsRef<Path>>(&mut self, path: P) {
        let path = path.as_ref();
        let mut file = File::open(path).expect(&format!("Couldn't open TAS file {:?}", path));
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();

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
        self.con.write_all(current_dir.as_bytes()).unwrap();

        println!("Sending code");
        let path = path.display().to_string();
        self.con.write_u8(0).unwrap();
        self.con.write_u32::<LittleEndian>(path.len() as u32).unwrap();
        self.con.write_all(path.as_bytes()).unwrap();
        self.con.write_u32::<LittleEndian>(code.len() as u32).unwrap();
        self.con.write_all(code.as_bytes()).unwrap();
        println!("Tas Execution started");
        let mut client = DiscordIpcClient::new(&"985548868659867698").expect("failed to create client");
        println!("Created client.");
        let activity = Activity::new();
        println!("New activity.");

        client
            .connect()
            .expect("failed to connect to Discord, please try again or relaunch Discord app");
        println!("Client connected.");

        let mut client = DiscordIpcClient::new(&"985548868659867698").expect("failed to create client");
        println!("Created client.");
        let activity = Activity::new();
        println!("New activity.");

        client
            .connect()
            .expect("failed to connect to Discord, please try again or relaunch Discord app");
        println!("Client connected.");

        let mut activity: Activity = activity.details(&"BING");
        println!("Activity details set.");
        activity = activity.state(&"YIPPEE!");
        println!("Activity state set.");
        let mut assets = activity::Assets::new();
        println!("New assets.");
        assets = assets.large_image(&"drp_refunct");
        println!("Assets large image set.");
        assets = assets.large_text(&"BAZ");
        println!("Assets large text set.");
        activity = activity.assets(assets);
        println!("Assets set.");

        let time_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        println!("Timestamp set.");

        activity = activity.timestamps(activity::Timestamps::new().start(time_unix));
        client.set_activity(activity).expect("client set activity");

        let mut activity: Activity = activity.details(&"BING");
        println!("Activity details set.");
        activity = activity.state(&"YIPPEE!");
        println!("Activity state set.");
        let mut assets = activity::Assets::new();
        println!("New assets.");
        assets = assets.large_image(&"drp_refunct");
        println!("Assets large image set.");
        assets = assets.large_text(&"BAZ");
        println!("Assets large text set.");
        activity = activity.assets(assets);
        println!("Assets set.");

        let time_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        println!("Timestamp set.");

        activity = activity.timestamps(activity::Timestamps::new().start(time_unix));
        client.set_activity(activity).expect("client set activity");

        // Works perfectly but can't currently get things like current level etc.

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
