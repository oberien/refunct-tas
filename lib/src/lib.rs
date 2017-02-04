#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate byteorder;

#[cfg(unix)]
extern crate libc;

use std::fs::{OpenOptions, File};
use std::sync::{Mutex, Once, ONCE_INIT};
use std::thread;

macro_rules! log {
    () => {{
        use ::std::io::Write;
        writeln!(&mut ::LOGFILE.lock().unwrap(), "").unwrap();
    }};
    ($fmt:expr) => {{
        use ::std::io::Write;
        writeln!(&mut ::LOGFILE.lock().unwrap(), $fmt).unwrap()
    }};
    ($fmt:expr, $($vars:tt)*) => {{
        use ::std::io::Write;
        writeln!(&mut ::LOGFILE.lock().unwrap(), $fmt, $($vars)*).unwrap()
    }};
}

mod error;
mod consts;
mod loops;
mod native;

lazy_static! {
    static ref LOGFILE: Mutex<File> = Mutex::new(OpenOptions::new()
        .create(true).write(true)
        .open("/tmp/refunct-tas.log").unwrap());
}

static INIT: Once = ONCE_INIT;

pub extern fn initialize() {
    INIT.call_once(|| {
        let exe = ::std::env::current_exe().unwrap();
        let file = exe.file_name().unwrap();
        if file == "Refunct-Linux-Shipping" {
            // hook stuff
            native::init();
            thread::spawn(|| {
                match loops::main_loop() {
                    Ok(_) => log!("Main Loop finished successful, exiting..."),
                    Err(err) => log!("Main Loop experienced an error: {:?}", err)
                }
            });
        }
    });
}
