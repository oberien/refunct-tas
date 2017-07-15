#[macro_use]
extern crate error_chain;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate byteorder;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;
extern crate hlua;

#[macro_use]
mod error;
mod tas;
mod config;
mod lua;
#[cfg(windows)]
mod inject;

use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;
use std::env;

use hlua::{Lua, LuaFunction};

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
    

    let tas;
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
    let tas = Rc::new(RefCell::new(tas));

    println!("Setting up lua environment...");
    let mut lua = Lua::new();
    lua.openlibs();
    lua::init_tas(&mut lua, tas.clone(), config);
    println!("Lua environment set up successfully");

    let script_file = env::args().skip(1).next().unwrap_or("tas.lua".to_string());
    println!("Parsing script {}...", script_file);
    let mut function = LuaFunction::load_from_reader(lua, File::open(&script_file).unwrap()).unwrap();
    println!("Script successfully parsed");

    //handle_err!(tas.test_loop())
    println!("Executing TAS...");
    function.call::<()>().unwrap();
    println!("TAS successfully finished");
    println!("Cleaning up...");
    handle_err!(tas.borrow_mut().set_delta(0.0));
    handle_err!(tas.borrow_mut().cont());
    println!("Finished");
    //handle_err!(tas.play(&frames, &config.ingame));
}

