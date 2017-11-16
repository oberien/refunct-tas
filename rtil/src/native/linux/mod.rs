#[macro_use] mod macros;
pub(in native) mod slateapp;
pub(in native) mod tick;
pub(in native) mod controller;
pub(in native) mod character;

use std::fs::File;
use std::env;
use std::collections::HashMap;

use libc::{self, c_void, PROT_READ, PROT_WRITE, PROT_EXEC};
use memmap::Mmap;
use object::{ElfFile, Object};
use cpp_demangle::{Symbol, DemangleOptions};

pub use self::slateapp::{hook_slateapp, FSlateApplication};
pub use self::tick::hook_tick;

// Shoutout to https://github.com/geofft/redhook/blob/master/src/ld_preload.rs#L18
// Rust doesn't directly expose __attribute__((constructor)), but this
// is how GNU implements it.
#[link_section=".init_array"]
pub static INITIALIZE_CTOR: extern fn() = ::initialize;

pub static mut AMYCHARACTER_FORCEDUNCROUCH: usize = 0;
pub static mut FSLATEAPPLICATION_TICK: usize = 0;
pub static mut FSLATEAPPLICATION_ONKEYDOWN: usize = 0;
pub static mut FSLATEAPPLICATION_ONKEYUP: usize = 0;
pub static mut FSLATEAPPLICATION_ONRAWMOUSEMOVE: usize = 0;
pub static mut ACONTROLLER_GETCONTROLROTATION: usize = 0;
pub static mut UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE: usize = 0;
pub static mut AMYCHARACTER_TICK: usize = 0;
pub static mut FAPP_DELTATIME: usize = 0;

const NAMES: [&str; 9] = [
    "AMyCharacter::ForcedUnCrouch",
    "FSlateApplication::Tick",
    "FSlateApplication::OnKeyDown",
    "FSlateApplication::OnKeyUp",
    "FSlateApplication::OnRawMouseMove",
    "AController::GetControlRotation",
    "UEngine::UpdateTimeAndHandleMaxTickRate",
    "AMyCharacter::Tick",
    "FApp::DeltaTime",
];

pub fn init() {
    let file = File::open(env::current_exe().unwrap()).unwrap();
    let file = unsafe { Mmap::map(&file) }.unwrap();
    let file = ElfFile::parse(&*file).unwrap();
    let options = DemangleOptions { no_params: true };
    let addrs: HashMap<_, _> = file.dynamic_symbols().into_iter()
        .flat_map(|sym| sym.name()
            .and_then(|name| Symbol::new(name).ok())
            .and_then(|symbol| symbol.demangle(&options).ok())
            .map(|name| name.split(' ').next().unwrap().to_string())
            .map(|name| (name, sym.address() as usize)))
        .filter(|&(ref name, _)| NAMES.contains(&name.as_str()))
        .collect();
    unsafe {
        AMYCHARACTER_FORCEDUNCROUCH = *addrs.get(NAMES[0]).unwrap();
        log!("found AMyCharacter::execForcedUnCrouch: {:#x}", AMYCHARACTER_FORCEDUNCROUCH);
        FSLATEAPPLICATION_TICK = *addrs.get(NAMES[1]).unwrap();
        log!("found FSlateApplication::Tick: {:#x}", FSLATEAPPLICATION_TICK);
        FSLATEAPPLICATION_ONKEYDOWN = *addrs.get(NAMES[2]).unwrap();
        log!("found FSlateApplication::OnKeyDown: {:#x}", FSLATEAPPLICATION_ONKEYDOWN);
        FSLATEAPPLICATION_ONKEYUP = *addrs.get(NAMES[3]).unwrap();
        log!("found FSlateApplication::OnKeyUp: {:#x}", FSLATEAPPLICATION_ONKEYUP);
        FSLATEAPPLICATION_ONRAWMOUSEMOVE = *addrs.get(NAMES[4]).unwrap();
        log!("found FSlateApplication::OnRawMouseMove: {:#x}", FSLATEAPPLICATION_ONRAWMOUSEMOVE);
        ACONTROLLER_GETCONTROLROTATION = *addrs.get(NAMES[5]).unwrap();
        log!("found AController::GetControlRotation: {:#x}", ACONTROLLER_GETCONTROLROTATION);
        UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE = *addrs.get(NAMES[6]).unwrap();
        log!("found UEngine::UpdateTimeAndHandleMaxTickRate: {:#x}", UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE);
        AMYCHARACTER_TICK = *addrs.get(NAMES[7]).unwrap();
        log!("found AMyCharacter::Tick: {:#x}", AMYCHARACTER_TICK);
        FAPP_DELTATIME = *addrs.get(NAMES[8]).unwrap();
        log!("found FApp::DeltaTime: {:#x}", FAPP_DELTATIME);
    }
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
