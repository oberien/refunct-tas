#[macro_use] mod macros;
pub(in native) mod slateapp;
pub(in native) mod controller;
pub(in native) mod character;
pub(in native) mod consts;
pub(in native) mod app;

use std::ptr::null;

use winapi::c_void;
use winapi::winnt::{PAGE_READWRITE, PAGE_EXECUTE_READ};
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

macro_rules! find {
    ($($name:ident,)*) => {
        $(
            pub(in native) static mut $name: usize = 0;
        )*
        pub(in native) fn init() {
            let base = unsafe { GetModuleHandleA(null()) as usize };
            log!("Got Base address: {:#x}", base);
            $(
                $name = base + consts::$name;
            )*
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
    ACONTROLLER_GETCONTROLROTATION,
    FMEMORY_MALLOC,
    FMEMORY_FREE,
    FNAME_FNAME,
}

pub(in native) fn make_rw(addr: usize) {
//    log!("make_rw: {:#x}", addr);
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    let mut out = 0;
    unsafe { VirtualProtect(page, 0x1000, PAGE_READWRITE, &mut out); }
}

pub(in native) fn make_rx(addr: usize) {
//    log!("make_rx: {:#x}", addr);
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    let mut out = 0;
    unsafe { VirtualProtect(page, 0x1000, PAGE_EXECUTE_READ, &mut out); }
}
