extern crate lua;

use std::io::{self, Read};
use std::cell::RefCell;
use std::rc::Rc;

use lua::stub::Stub;
use lua::Lua;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = Vec::new();
    let mut lua = Lua::new(Rc::new(RefCell::new(Stub::new())));
    while let Ok(_) = stdin.read_to_end(&mut buf) {
        {
            let input = String::from_utf8_lossy(&buf);
            if !input.ends_with('\n') {
                println!();
            }
            lua.execute(&input);
        }
        buf.clear();
    }
}