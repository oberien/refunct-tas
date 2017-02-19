#[macro_use]
mod macros;
mod slateapp;
mod newgame;
mod tick;
mod controller;

use libc::{self, c_void, PROT_READ, PROT_WRITE, PROT_EXEC};

pub use self::slateapp::{hook_slateapp, FSlateApplication};
pub use self::newgame::hook_newgame;
pub use self::tick::hook_tick;
pub use self::controller::{hook_controller, AController};

// Shoutout to https://github.com/geofft/redhook/blob/master/src/ld_preload.rs#L18
// Rust doesn't directly expose __attribute__((constructor)), but this
// is how GNU implements it.
#[link_section=".init_array"]
pub static INITIALIZE_CTOR: extern fn() = ::initialize;

pub fn make_rw(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    unsafe { libc::mprotect(page, 0x1000, PROT_READ | PROT_WRITE); }
}

pub fn make_rx(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    unsafe { libc::mprotect(page, 0x1000, PROT_READ | PROT_EXEC); }
}
