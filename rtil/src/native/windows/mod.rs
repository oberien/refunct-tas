pub mod consts;

use std::ptr;

use winapi::ctypes::c_void;
use winapi::um::winnt::{PAGE_READWRITE, PAGE_EXECUTE_READ};
use kernel32::{VirtualProtect, GetModuleHandleA};

// https://www.unknowncheats.me/forum/general-programming-and-reversing/123333-demo-pure-rust-internal-coding.html
// Entry Point
#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern "stdcall" fn DllMain(module: u32, reason: u32, reserved: *mut c_void) {
    match reason {
        1 => ::initialize(),
        _ => ()
    }
}

pub fn base_address() -> usize {
    unsafe { GetModuleHandleA(ptr::null()) as usize }
}

macro_rules! find {
    ($($name:ident,)*) => {
        $(
            pub(in native) static mut $name: usize = 0;
        )*
        pub(in native) fn init() {
            let base = base_address();
            log!("Got Base address: {:#x}", base);
            unsafe {
                $(
                    $name = base + self::consts::$name;
                )*
            }
        }
    }
}

find! {
    FSLATEAPPLICATION_TICK,
    AMYCHARACTER_TICK,
    AMYCHARACTER_FORCEDUNCROUCH,
    UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE,
    FAPP_DELTATIME,
    FSLATEAPPLICATION_ONKEYDOWN,
    FSLATEAPPLICATION_ONKEYUP,
    FSLATEAPPLICATION_ONRAWMOUSEMOVE,
    FMEMORY_MALLOC,
    FMEMORY_FREE,
    FNAME_FNAME,
    AMYHUD_DRAWHUD,
    AHUD_DRAWLINE,
    AHUD_DRAWTEXT,
    AHUD_PROJECT,
    GWORLD,
    UWORLD_SPAWNACTOR,
    UWORLD_DESTROYACTOR,
    AMYCHARACTER_STATICCLASS,
    APAWN_SPAWNDEFAULTCONTROLLER,
}

pub(in native) fn make_rw(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut std::ffi::c_void;
    let mut out = 0;
    unsafe { VirtualProtect(page, 0x1000, PAGE_READWRITE, &mut out); }
}

pub(in native) fn make_rx(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut std::ffi::c_void;
    let mut out = 0;
    unsafe { VirtualProtect(page, 0x1000, PAGE_EXECUTE_READ, &mut out); }
}
