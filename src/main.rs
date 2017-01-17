use std::process::Command;

extern crate gdb;
#[macro_use]
extern crate error_chain;

#[macro_use]
mod error;
mod consts;
mod tas;

use std::io::BufRead;

use tas::Tas;

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

    let mut tas = Tas::new(pid).unwrap();
    handle_err!(tas.init());

    let std = ::std::io::stdin();
    let input = std.lock();
    let mut last = String::new();
    for l in input.lines() {
        let l = l.unwrap();
        for c in l.chars() {
            if !last.contains(c) {
                handle_err!(tas.press_key(c));
            }
        }
        for c in last.chars() {
            if !l.contains(c) {
                handle_err!(tas.release_key(c));
            }
        }
        last = l;
        handle_err!(tas.step());
    }
}
