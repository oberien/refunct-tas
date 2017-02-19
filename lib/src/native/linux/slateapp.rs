use native::SLATEAPP;
use std::slice;

use libc::{uintptr_t, int32_t, uint32_t};
use byteorder::{WriteBytesExt, LittleEndian};

use consts;
use statics::Static;


lazy_static! {
    static ref START: Static<[u8; 12]> = Static::new();
}

pub struct FSlateApplication;

impl FSlateApplication {
    pub unsafe fn on_key_down(key_code: i32, character_code: u32, is_repeat: bool) {
        let fun: unsafe extern fn(this: uintptr_t, key_code: int32_t, character_code: uint32_t, is_repeat: uint32_t) =
                ::std::mem::transmute(consts::FSLATEAPPLICATION_ONKEYDOWN);
        fun(*SLATEAPP.get() as uintptr_t, key_code, character_code, is_repeat as u32)
    }
    pub unsafe fn on_key_up(key_code: i32, character_code: u32, is_repeat: bool) {
        let fun: unsafe extern fn(this: uintptr_t, key_code: int32_t, character_code: uint32_t, is_repeat: uint32_t) =
                ::std::mem::transmute(consts::FSLATEAPPLICATION_ONKEYUP);
        fun(*SLATEAPP.get() as uintptr_t, key_code, character_code, is_repeat as u32)
    }

    pub unsafe fn on_raw_mouse_move(x: i32, y: i32) {
        let fun: unsafe extern fn(this: uintptr_t, x: int32_t, y: int32_t) =
                ::std::mem::transmute(consts::FSLATEAPPLICATION_ONRAWMOUSEMOVE);
        fun(*SLATEAPP.get() as uintptr_t, x, y)
    }
}

pub fn hook_slateapp() {
    log!("Hooking FSlateApplication::Tick");
    super::make_rw(consts::FSLATEAPPLICATION_TICK);
    let hook_fn = get_slateapp as *const () as usize;
    let mut tick = unsafe { slice::from_raw_parts_mut(consts::FSLATEAPPLICATION_TICK as *mut u8, 12) }; 
    let mut saved = [0u8; 12];
    saved[..].copy_from_slice(tick);
    START.set(saved);
    log!("orig tick: {:?}", tick);
    // mov rax, addr
    tick[..2].copy_from_slice(&[0x48, 0xb8]);
    (&mut tick[2..10]).write_u64::<LittleEndian>(hook_fn as u64).unwrap();
    // jmp rax
    tick[10..].copy_from_slice(&[0xff, 0xe0]);
    log!("Injected Code: {:?}", tick);
    super::make_rx(consts::FSLATEAPPLICATION_TICK);
    log!("FSlateApplication::Tick successfully hooked");
}

#[naked]
unsafe extern fn get_slateapp() -> ! {
    // push argument
    asm!("push rdi" :::: "intel");
    alignstack_pre!();
    // call interceptor
    asm!("call rax" :: "{rax}"(save_slateapp as usize) :: "intel");
    alignstack_post!();
    // restore everything and jump to original function
    asm!(r"
        pop rdi
        jmp rax
    ":: "{rax}"(consts::FSLATEAPPLICATION_TICK) :: "intel");
    ::std::intrinsics::unreachable()
}

#[inline(never)]
extern fn save_slateapp(this: usize) {
    super::make_rw(consts::FSLATEAPPLICATION_TICK);
    SLATEAPP.set(this);
    let mut tick = unsafe { slice::from_raw_parts_mut(consts::FSLATEAPPLICATION_TICK as *mut _, 12) }; 
    tick.copy_from_slice(&*START.get());
    super::make_rx(consts::FSLATEAPPLICATION_TICK);
    log!("Got FSlateApplication: {:#x}", this);
}

