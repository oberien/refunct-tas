#![feature(llvm_asm)]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
#![feature(abi_thiscall)]
#![feature(stmt_expr_attributes)]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate lazy_static;
extern crate byteorder;
extern crate lua;
extern crate backtrace;
extern crate failure;
extern crate protocol;
extern crate crossbeam_channel;

#[cfg(unix)] extern crate libc;
#[cfg(unix)] extern crate dynsym;
#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;
extern crate object;

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
        panic::set_hook(Box::new(|info| {
            let msg = match info.payload().downcast_ref::<&'static str>() {
                Some(s) => *s,
                None => match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            };
            let thread = thread::current();
            let name = thread.name().unwrap_or("<unnamed>");
            log!("thread '{}' panicked at '{}'\nBacktrace: {:?}", name, msg, backtrace::Backtrace::new());
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
