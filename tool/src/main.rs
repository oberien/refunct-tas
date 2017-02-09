#[macro_use]
extern crate error_chain;
extern crate toml;
extern crate rustc_serialize;
extern crate byteorder;

#[macro_use]
mod error;
mod tas;
mod config;
mod pidof;

use std::io::BufRead;

use tas::Tas;
use config::Config;

fn main() {
    // set gdb path
    if cfg!(windows) {
        ::std::env::set_var("GDB_BINARY", "./gdb.exe");
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

