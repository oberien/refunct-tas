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

use nfd::Response;
use std::env;
use std::path::Path;

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
    let mut script_file = String::new();
    if env::args().collect::<Vec<String>>().len() == 1 {
        let cur_dir = std::env::current_dir().unwrap();
        let cur_dir = cur_dir.to_str().unwrap();
        let result = nfd::open_file_dialog(Some("lua"), Some(cur_dir)).unwrap_or_else(|e| { panic!(e) });
        match result {
            Response::Okay(file_path) => script_file = Path::new(&file_path).file_name().unwrap().to_str().unwrap().to_string(),
            Response::OkayMultiple(_) => panic!("Multiple files selected."),
            Response::Cancel => println!("Cancelled file selection.")
        };
    } else {
        script_file = env::args().skip(1).next().unwrap();
    }
    println!("Executing Script {} ...", script_file);
    tas.execute(script_file, &config);
    println!("Script Executed.");

    println!("Finished");
}

/*fn get_script_file() -> &str {
    if env::args().collect::<Vec<String>>().len() == 1 {
        let cur_dir = std::env::current_dir().unwrap();
        let cur_dir = cur_dir.to_str().unwrap();
        let result = nfd::open_file_dialog(Some("lua"), Some(cur_dir)).unwrap_or_else(|e| { panic!(e) });
        let mut ret = "";
        match result {
            Response::Okay(file_path) => ret = file_path.as_str(),
            Response::OkayMultiple(_) => panic!("Multiple files selected."),
            Response::Cancel => println!("Cancelled file selection.")
        };
        return ret;
    } else {
        return env::args().skip(1).next().unwrap();
    }
}*/