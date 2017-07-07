#[macro_use]
mod macros;
mod slateapp;
mod newgame;
mod tick;
mod controller;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::slice;

use libc::{self, c_void, PROT_READ, PROT_WRITE, PROT_EXEC};
use memmem::{TwoWaySearcher, Searcher};

use consts;
pub use self::slateapp::{hook_slateapp, FSlateApplication};
pub use self::newgame::hook_newgame;
pub use self::tick::hook_tick;
pub use self::controller::{hook_controller, AController};

// Shoutout to https://github.com/geofft/redhook/blob/master/src/ld_preload.rs#L18
// Rust doesn't directly expose __attribute__((constructor)), but this
// is how GNU implements it.
#[link_section=".init_array"]
pub static INITIALIZE_CTOR: extern fn() = ::initialize;

pub static mut AMYCHARACTER_EXECFORCEDUNCROUCH: usize = 0;
pub static mut FSLATEAPPLICATION_TICK: usize = 0;
pub static mut FSLATEAPPLICATION_ONKEYDOWN: usize = 0;
pub static mut FSLATEAPPLICATION_ONKEYUP: usize = 0;
pub static mut FSLATEAPPLICATION_ONRAWMOUSEMOVE: usize = 0;
pub static mut ACONTROLLER_GETCONTROLROTATION: usize = 0;
pub static mut UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE: usize = 0;

pub fn init() {
    let pages = refunct_pages();
    log!("pages: {:?}", pages);
    unsafe {
        AMYCHARACTER_EXECFORCEDUNCROUCH =
            find_signature(&pages, &consts::AMYCHARACTER_EXECFORCEDUNCROUCH)
            - consts::AMYCHARACTER_EXECFORCEDUNCROUCH_OFFSET;
        log!("found AMyCharacter::execForcedUnCrouch: {:#x}", AMYCHARACTER_EXECFORCEDUNCROUCH);
        FSLATEAPPLICATION_TICK =
            find_signature(&pages, &consts::FSLATEAPPLICATION_TICK)
                - consts::FSLATEAPPLICATION_TICK_OFFSET;
        log!("found FSlateApplication::Tick: {:#x}", FSLATEAPPLICATION_TICK);
        // onkeyup and onkeydown are pretty much equal
        FSLATEAPPLICATION_ONKEYDOWN =
            find_nth_signature(&pages, &consts::FSLATEAPPLICATION_ONKEYDOWN, 0)
                - consts::FSLATEAPPLICATION_ONKEYDOWN_OFFSET;
        log!("found FSlateApplication::OnKeyDown: {:#x}", FSLATEAPPLICATION_ONKEYDOWN);
        FSLATEAPPLICATION_ONKEYUP =
            find_nth_signature(&pages, &consts::FSLATEAPPLICATION_ONKEYDOWN, 1)
                - consts::FSLATEAPPLICATION_ONKEYDOWN_OFFSET;
        log!("found FSlateApplication::OnKeyUp: {:#x}", FSLATEAPPLICATION_ONKEYUP);
        FSLATEAPPLICATION_ONRAWMOUSEMOVE =
            find_signature(&pages, &consts::FSLATEAPPLICATION_ONRAWMOUSEMOVE)
                - consts::FSLATEAPPLICATION_ONRAWMOUSEMOVE_OFFSET;
        log!("found FSlateApplication::OnRawMouseMove: {:#x}", FSLATEAPPLICATION_ONRAWMOUSEMOVE);
        ACONTROLLER_GETCONTROLROTATION =
            find_signature(&pages, &consts::ACONTROLLER_GETCONTROLROTATION)
                - consts::ACONTROLLER_GETCONTROLROTATION_OFFSET;
        log!("found AController::GetControlRotation: {:#x}", ACONTROLLER_GETCONTROLROTATION);
        UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE =
            find_signature(&pages, &consts::UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE)
                - consts::UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE_OFFSET;
        log!("found UEngine::UpdateTimeAndHandleMaxTickRate: {:#x}", UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE);
    }
}

fn find_signature(pages: &[(usize, usize)], signature: &[u8]) -> usize {
    if let Some(addr) = find_nth_signature_opt(pages, signature, 0) {
        if let Some(duplicate) = find_nth_signature_opt(pages, signature, 1) {
            log!("duplicate entries found for signature {:?}: {:#x} and {:#x}", signature, addr, duplicate);
            ::std::process::exit(1);
        }
        return addr;
    }
    log!("signature {:?} not found", signature);
    ::std::process::exit(1)
}

fn find_nth_signature(pages: &[(usize, usize)], signature: &[u8], n: usize) -> usize {
    if let Some(addr) = find_nth_signature_opt(pages, signature, n) {
        return addr;
    }
    log!("signature number {} ({:?}) not found", n, signature);
    ::std::process::exit(1)
}

fn find_nth_signature_opt(pages: &[(usize, usize)], signature: &[u8], n: usize) -> Option<usize> {
    let mut i = 0;
    for &(start, end) in pages {
        let mut inpage = start;
        while let Some(addr) = search_page(inpage, end, signature) {
            let addr = inpage + addr;
            if i == n {
                return Some(addr);
            }
            // +1 because the signature is at addr
            inpage = addr + 1;
            i += 1;
        }
    }
    None
}

fn search_page(start: usize, end: usize, signature: &[u8]) -> Option<usize> {
    let ram = unsafe { slice::from_raw_parts(start as *const u8, end - start) };
    let searcher = TwoWaySearcher::new(signature);
    searcher.search_in(ram)
}

fn refunct_pages() -> Vec<(usize, usize)> {
    let file = File::open("/proc/self/maps").unwrap();
    let file = BufReader::new(file);
    let mut ranges = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();
        if line.ends_with("Refunct-Linux-Shipping") {
            let mut blocks = line.split(" ");
            let mut addr = blocks.next().unwrap().split("-");
            let perms = blocks.next().unwrap();
            if perms == "r-xp" {
                let start = usize::from_str_radix(addr.next().unwrap(), 16).unwrap();
                let end = usize::from_str_radix(addr.next().unwrap(), 16).unwrap();
                ranges.push((start, end))
            }
        }
    }
    ranges
}

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
