#[macro_use] extern crate error_chain;
extern crate toml;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate byteorder;
#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;
extern crate nfd;

#[macro_use] mod error;
mod tas;
mod config;
#[cfg(windows)] mod inject;

use std::env;
use std::path::{Path, PathBuf};

use nfd::Response;

use tas::Tas;
use config::Config;

// link against shell32.dll
#[link = "shell32.dll"]
extern "C" {}

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
        if !Path::new("tas.lua").is_file() {
            let cur_dir = std::env::current_dir().unwrap();
            let cur_dir = cur_dir.to_str().unwrap();
            let result = nfd::open_file_dialog(Some("lua"), Some(cur_dir)).unwrap();
            match result {
                Response::Okay(file_path) => PathBuf::from(file_path),
                Response::OkayMultiple(_) => unreachable!("Multiple files selected."),
                Response::Cancel => panic!("Cancelled file selection.")
            }
        } else {
            PathBuf::from("tas.lua")
        }
    } else {
        PathBuf::from(env::args().nth(1).unwrap())
    };
    println!("Executing Script {} ...", script_file.display());
    tas.execute(script_file, &config);
    println!("Script Executed.");
    println!("Finished");

}