#[macro_use] mod macros;
pub(in native) mod slateapp;
pub(in native) mod controller;
pub(in native) mod character;
pub(in native) mod app;

use std::env;
use std::collections::HashMap;

use libc::{self, c_void, PROT_READ, PROT_WRITE, PROT_EXEC};
use dynsym;

// Shoutout to https://github.com/geofft/redhook/blob/master/src/ld_preload.rs#L18
// Rust doesn't directly expose __attribute__((constructor)), but this
// is how GNU implements it.
#[link_section=".init_array"]
pub static INITIALIZE_CTOR: extern "C" fn() = ::initialize;

macro_rules! find {
    ($($idx:expr, $name:ident, $symbol:expr,)*) => {
        $(
            pub(in native) static mut $name:usize = 0;
        )*
        const NAMES: &[&str] = &[
            $(
                $symbol,
            )*
        ];

        pub(in native) fn init() {
            let addrs: HashMap<_, _> = dynsym::iter(env::current_exe().unwrap()).into_iter()
                .filter_map(|(name, addr)| NAMES.iter()
                    .find(|&&pattern| {
                        if pattern.starts_with('^') {
                            name.starts_with(&pattern[1..])
                        } else {
                            name.contains(pattern)
                        }
                    })
                    .map(|&name| (name, addr)))
                .collect();
            log!("{:?}", addrs);
            unsafe {
                $(
                    $name = *addrs.get(NAMES[$idx]).unwrap();
                    log!("found {}: {:#x}", NAMES[$idx], $name);
                )*
            }
        }
    }
}

find! {
    0, AMYCHARACTER_FORCEDUNCROUCH, "^AMyCharacter::ForcedUnCrouch()",
    1, FSLATEAPPLICATION_TICK, "^FSlateApplication::Tick()",
    2, FSLATEAPPLICATION_ONKEYDOWN, "^FSlateApplication::OnKeyDown(int, unsigned int, bool)",
    3, FSLATEAPPLICATION_ONKEYUP, "^FSlateApplication::OnKeyUp(int, unsigned int, bool)",
    4, FSLATEAPPLICATION_ONRAWMOUSEMOVE, "^FSlateApplication::OnRawMouseMove(int, int)",
    5, ACONTROLLER_GETCONTROLROTATION, "^AController::GetControlRotation()",
    6, UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE, "^UEngine::UpdateTimeAndHandleMaxTickRate()",
    7, AMYCHARACTER_TICK, "^AMyCharacter::Tick(float)",
    8, FAPP_DELTATIME, "^FApp::DeltaTime",
    9, FMEMORY_MALLOC, "^FMemory::Malloc(unsigned long, unsigned int)",
    10, FMEMORY_FREE, "^FMemory::Free(void*)",
    11, FNAME_FNAME, "^FName::complete object constructor(wchar_t const*, EFindName)",
}

pub(in native) fn make_rw(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    unsafe { libc::mprotect(page, 0x1000, PROT_READ | PROT_WRITE); }
}

pub(in native) fn make_rx(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    unsafe { libc::mprotect(page, 0x1000, PROT_READ | PROT_EXEC); }
}
