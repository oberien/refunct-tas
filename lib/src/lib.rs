#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
#![feature(mpsc_select)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate byteorder;

#[cfg(unix)]
extern crate libc;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;

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
#[cfg(windows)]
pub use native::DllMain;

static INIT: Once = ONCE_INIT;

pub extern fn initialize() {
    INIT.call_once(|| {
        log!("initialize");
        let exe = ::std::env::current_exe().unwrap();
        let file = exe.file_name().unwrap();
        if cfg!(unix) && file == "Refunct-Linux-Shipping" || cfg!(windows) && file == "Refunct-Win32-Shipping.exe" {
            thread::spawn(|| {
                log!("Starting initialize");
                // on Linux we need to wait for the packer to finish
                if cfg!(unix) {
                    ::std::thread::sleep(::std::time::Duration::from_secs(5));
                }
                // start main loop, which internally spawns a new thread
                loops::main_loop();
                // hook stuff
                native::init();
            });
        }
    });
}
