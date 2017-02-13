use std::path::Path;
use std::fs::File;
use std::io::Read;

use toml::{Parser, Decoder, Value};
use rustc_serialize::Decodable;

#[derive(Debug, RustcDecodable, Clone)]
pub struct Config {
    pub forward: char,
    pub backward: char,
    pub left: char,
    pub right: char,
    pub jump: char,
    pub crouch: u32,
    pub menu: u32,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Config {
        let mut config = String::new();
        {
            let mut config_file = File::open(path).expect("Failed to open config file");
            config_file.read_to_string(&mut config).expect("Failed to read config file");
        }

        let mut parser = Parser::new(&config);

        let parsed = match parser.parse() {
            Some(x) => x,
            None => {
                for e in parser.errors {
                    println!("{}", e);
                }
                panic!("Failed to parse config");
            }
        };

        match Decodable::decode(&mut Decoder::new(Value::Table(parsed))) {
            Ok(x) => x,
            Err(e) => panic!("Failed to decode config: {}", e),
        }
    }
}
