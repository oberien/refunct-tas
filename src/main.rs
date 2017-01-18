use std::process::Command;

extern crate gdb;
#[macro_use]
extern crate error_chain;
extern crate toml;
extern crate rustc_serialize;

#[macro_use]
mod error;
mod consts;
mod tas;
mod config;

use std::io::BufRead;

use tas::Tas;
use config::Inputs;

fn main() {
    let mut s;
    loop {
        let output = Command::new("pidof")
            .arg("Refunct-Linux-Shipping")
            .output()
            .expect("Cannot get pid of Refunct");
        s = String::from_utf8(output.stdout).expect("Output of pidof is not utf8");
        if s.pop() == Some('\n') {
            break;
        }
    }
    let pid: u32 = s.parse().expect("Pidof returned non-number");
    println!("pid: {}", pid);

    let inputs = Inputs::load("Inputs.toml");

    let std = ::std::io::stdin();
    let lock = std.lock();
    let frames = tas::parse_lines(lock.lines(), &inputs);

    let mut tas = Tas::new(pid).unwrap();
    handle_err!(tas.init());
    handle_err!(tas.play(&frames, &inputs));
}
