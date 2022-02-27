use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub forward: Key,
    pub backward: Key,
    pub left: Key,
    pub right: Key,
    pub jump: Key,
    pub crouch: Key,
    pub menu: Key,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(untagged)]
pub enum Key {
    Char(char),
    Code(u32),
}

impl From<Key> for i32 {
    fn from(key: Key) -> i32 {
        match key {
            Key::Char(c) => c as i32,
            Key::Code(c) => c as i32,
        }
    }
}


impl Config {
    #[allow(unused)]
    pub fn save<P: AsRef<Path>>(&self, path: P) {
        let path = path.as_ref();
        let contents = toml::to_string(self).unwrap();
        let mut file = OpenOptions::new().create(true).write(true)
            .truncate(true).open(path).expect("Failed to open config file");
        writeln!(file, "{}", contents).expect("Failed to write config file");
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Config {
        let path = path.as_ref();
        if !path.exists() {
            panic!("Config file doesn't exist");
        }

        let mut config = String::new();
        {
            let mut config_file = File::open(path).expect("Failed to open config file");
            config_file.read_to_string(&mut config).expect("Failed to read config file");
        }

        toml::from_str(&config).expect("Failed to decode config")
    }
}
