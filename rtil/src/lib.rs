#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
#![feature(mpsc_select)]
#![feature(abi_thiscall)]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate lazy_static;
extern crate byteorder;
extern crate memmap;
extern crate lua;
extern crate backtrace;

#[cfg(unix)] extern crate libc;
#[cfg(unix)] extern crate object;
#[cfg(unix)] extern crate cpp_demangle;
#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;

use std::sync::{Once, ONCE_INIT};
use std::thread;
use std::panic;

mod error;
#[macro_use] mod statics;
mod native;
mod threads;

#[cfg(unix)] pub use native::INITIALIZE_CTOR;
#[cfg(windows)] pub use native::DllMain;

static INIT: Once = ONCE_INIT;

pub extern "C" fn initialize() {
    INIT.call_once(|| {
        panic::set_hook(Box::new(|_| {
            log!("{:?}", backtrace::Backtrace::new());
        }));
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
                // start threads
                threads::start();
                // hook stuff
                native::init();
            });
        }
    });
}
