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
        let mut split = l.split("|");
        let keys = split.next().expect("empty line???");
        if let Some(mouse) = split.next() {
            let mut split = mouse.split(|x| x == ' ' || x == ':');
            let x = split.next().map(|x| x.parse().expect(&format!("cannot convert {} to number", x))).unwrap_or(0);
            let y = split.next().map(|y| y.parse().expect(&format!("cannot convert {} to number", y))).unwrap_or(0);
            if x != 0 || y != 0 {
                handle_err!(tas.move_mouse(x, y));
            }
        }
        for c in keys.chars() {
            if !last.contains(c) {
                handle_err!(tas.press_key(c));
            }
        }
        for c in last.chars() {
            if !keys.contains(c) {
                handle_err!(tas.release_key(c));
            }
        }
        last = keys.to_string();
        handle_err!(tas.step());
    }
}
