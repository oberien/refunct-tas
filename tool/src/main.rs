#[macro_use]
extern crate error_chain;
extern crate toml;
extern crate rustc_serialize;
extern crate byteorder;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;

#[macro_use]
mod error;
mod tas;
mod config;
#[cfg(windows)]
mod inject;

use std::io::BufRead;

use tas::Tas;
use config::Config;

fn main() {
    println!("Read config...");
    let config = Config::load("Config.toml");
    println!("Config loaded successfully.");

    let std = ::std::io::stdin();
    let lock = std.lock();
    println!("Start parsing...");
    let frames = tas::parse_lines(lock.lines(), &config.infile);
    println!("Parsing finished successfully.");

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
                println!("TaS created successfully.");
            }
        }
    }
    #[cfg(unix)]
    {
        println!("Create tas...");
        tas = Tas::new().unwrap();
        println!("TaS created successfully.");
    }

    //handle_err!(tas.test_loop())
    println!("Wait for click on 'New Game'...");
    handle_err!(tas.wait_for_new_game());
    println!("New Game detected. Starting TaS execution");
    handle_err!(tas.play(&frames, &config.ingame));
}

