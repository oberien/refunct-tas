#[macro_use] extern crate error_chain;
extern crate toml;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate byteorder;
#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;

#[macro_use] mod error;
mod tas;
mod config;
#[cfg(windows)] mod inject;

use std::env;
use std::path::{Path, PathBuf};

use tas::Tas;
use config::Config;

fn main() {
    println!("Read config...");
    let config = Config::load("Config.toml");
    println!("Config loaded successfully.");

    //let std = ::std::io::stdin();
    //let lock = std.lock();
    //println!("Start parsing...");
    //let frames = tas::parse_lines(lock.lines(), &config.infile);
    //println!("Parsing finished successfully.");
    

    let mut tas;
    #[cfg(windows)]
    {
        // inject dll
        println!("Testing if DLL is already injected");
        match Tas::new() {
            Ok(val) => {
                tas = val;
                println!("DLL already injected.");
            },
            Err(_) => {
                println!("DLL has not been injected yet, injecting...");
                inject::inject();
                println!("DLL Injected");
                println!("Create tas...");
                tas = Tas::new().unwrap();
                println!("TAS created successfully.");
            }
        }
    }
    #[cfg(unix)]
    {
        println!("Create tas...");
        tas = Tas::new().unwrap();
        println!("TAS created successfully.");
    }
    let script_file = if env::args().len() == 1 {
        if !Path::new("main.re").is_file() {
            panic!("No tas file specified. Usage: refunct-tas <file.lua>");
        } else {
            PathBuf::from("main.re")
        }
    } else {
        PathBuf::from(env::args().nth(1).unwrap())
    };
    println!("Executing Script {} ...", script_file.display());
    tas.execute(script_file, &config);
    println!("Script Executed.");
    println!("Finished");
}

