use std::slice;

use winapi::minwindef::BOOL;
use byteorder::{WriteBytesExt, LittleEndian};

use statics::Static;
use native::SLATEAPP;
use super::{
    FSLATEAPPLICATION,
    FSLATEAPPLICATION_TICK,
    FSLATEAPPLICATION_ONKEYDOWN,
    FSLATEAPPLICATION_ONKEYUP,
    FSLATEAPPLICATION_ONRAWMOUSEMOVE,
    make_rw,
    make_rx,
};

lazy_static! {
    static ref START: Static<[u8; 7]> = Static::new();
}

pub struct FSlateApplication;

impl FSlateApplication {
    pub unsafe fn on_key_down(key_code: i32, character_code: u32, is_repeat: bool) {
        // set arguments
        asm!("push ecx" :: "{ecx}"(is_repeat as BOOL) :: "intel");
        asm!("push ecx" :: "{ecx}"(character_code) :: "intel");
        asm!("push ecx" :: "{ecx}"(key_code) :: "intel");
        // call function with thiscall
        asm!("call eax" :: "{ecx}"(FSLATEAPPLICATION), "{eax}"(FSLATEAPPLICATION_ONKEYDOWN as usize) :: "intel");
    }
    pub unsafe fn on_key_up(key_code: i32, character_code: u32, is_repeat: bool) {
        // set arguments
        asm!("push ecx" :: "{ecx}"(is_repeat as BOOL) :: "intel");
        asm!("push ecx" :: "{ecx}"(character_code) :: "intel");
        asm!("push ecx" :: "{ecx}"(key_code) :: "intel");
        // call function with thiscall
        asm!("call eax" :: "{ecx}"(FSLATEAPPLICATION), "{eax}"(FSLATEAPPLICATION_ONKEYUP as usize) :: "intel");
    }

    pub unsafe fn on_raw_mouse_move(x: i32, y: i32) {
        // set arguments
        asm!("push ecx" :: "{ecx}"(y) :: "intel");
        asm!("push ecx" :: "{ecx}"(x) :: "intel");
        // call function with thiscall
        asm!("call eax" :: "{ecx}"(FSLATEAPPLICATION), "{eax}"(FSLATEAPPLICATION_ONRAWMOUSEMOVE as usize) :: "intel");
    }
}

pub fn hook_slateapp() {
    log!("Hooking FSlateApplication::Tick");
    let addr = unsafe { FSLATEAPPLICATION_TICK };
    make_rw(addr);
    let hook_fn = get_slateapp as *const () as usize;
    let mut tick = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 7) }; 
    let mut saved = [0u8; 7];
    saved[..].copy_from_slice(tick);
    START.set(saved);
    log!("orig tick: {:?}", tick);
    // mov eax, addr
    tick[0] = 0xb8;
    (&mut tick[1..5]).write_u32::<LittleEndian>(hook_fn as u32).unwrap();
    // jmp eax
    tick[5..].copy_from_slice(&[0xff, 0xe0]);
    log!("Injected Code: {:?}", tick);
    make_rx(addr);
    log!("FSlateApplication::Tick successfully hooked");
}

#[inline(never)]
#[naked]
unsafe extern fn get_slateapp() -> ! {
    // push argument
    asm!("push ecx" :::: "intel");
    // call interceptor
    asm!("call $0
    " :: "i"(save_slateapp as usize) :: "intel");
    // restore everything and jump to original function
    asm!(r"
        pop ecx
        jmp eax
    ":: "{eax}"(FSLATEAPPLICATION_TICK) :: "intel");
    ::std::intrinsics::unreachable()
}

#[inline(never)]
extern fn save_slateapp(this: usize) {
    log!("save_slateapp");
    let addr = unsafe { FSLATEAPPLICATION_TICK };
    make_rw(addr);
    SLATEAPP.set(this + 0x3c);
    unsafe { FSLATEAPPLICATION = this + 0x3c };
    let mut tick = unsafe { slice::from_raw_parts_mut(addr as *mut _, 7) }; 
    tick.copy_from_slice(&*START.get());
    make_rx(addr);
    log!("Got FSlateApplication: {:#x}", this);
}
