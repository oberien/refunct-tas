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

use std::sync::{Once, ONCE_INIT};
use std::thread;

mod error;
#[macro_use]
mod statics;
mod consts;
mod loops;
mod native;

pub use native::tick_intercept;
#[cfg(unix)]
pub use native::INITIALIZE_CTOR;

static INIT: Once = ONCE_INIT;

pub extern fn initialize() {
    INIT.call_once(|| {
        let exe = ::std::env::current_exe().unwrap();
        let file = exe.file_name().unwrap();
        if file == "Refunct-Linux-Shipping" {
            thread::spawn(|| {
                ::std::thread::sleep(::std::time::Duration::from_secs(5));
                // hook stuff
                native::init();
                thread::spawn(|| {
                    match loops::main_loop() {
                        Ok(_) => log!("Main Loop finished successful, exiting..."),
                        Err(err) => log!("Main Loop experienced an error: {:?}", err)
                    }
                });
            });
        }
    });
}
