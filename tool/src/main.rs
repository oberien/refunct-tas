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
    // inject dll
    if cfg!(windows) {
        inject::inject();
        println!("DLL Injected");
        // TODO: remove
        ::std::thread::sleep(::std::time::Duration::from_secs(5));
    }
    println!("Read config...");
    let config = Config::load("Config.toml");
    println!("Config loaded successfully.");

    let std = ::std::io::stdin();
    let lock = std.lock();
    println!("Start parsing...");
    let frames = tas::parse_lines(lock.lines(), &config.infile);
    println!("Parsing finished successfully.");

    //println!("Get PID of Refunct...");
    //let pid = pidof::pidof();
    //println!("Got PID: {}", pid);

    println!("Create tas...");
    let mut tas = Tas::new().unwrap();
    println!("TaS created successfully.");
    //handle_err!(tas.test_loop())
    println!("Wait for click on 'New Game'...");
    handle_err!(tas.wait_for_new_game());
    println!("New Game detected. Starting TaS execution");
    handle_err!(tas.play(&frames, &config.ingame));
}

