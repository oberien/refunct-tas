extern crate dynsym;

use std::env;

fn main() {
    for (name, addr) in dynsym::iter(env::args().nth(1).unwrap()) {
        println!("{:#10x}    {}", addr, name);
    }
}
