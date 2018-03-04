use native::slateapp::{SLATEAPP, FSlateApplication};

use libc::{uintptr_t, int32_t, uint32_t};

use native::{
    FSLATEAPPLICATION_ONKEYDOWN,
    FSLATEAPPLICATION_ONKEYUP,
    FSLATEAPPLICATION_ONRAWMOUSEMOVE,
};

impl FSlateApplication {
    pub(in native) fn on_key_down(key_code: i32, character_code: u32, is_repeat: bool) {
        let fun: extern "C" fn(this: uintptr_t, key_code: int32_t, character_code: uint32_t, is_repeat: uint32_t) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONKEYDOWN) };
        fun(*SLATEAPP.get() as uintptr_t, key_code, character_code, is_repeat as u32)
    }
    pub(in native) fn on_key_up(key_code: i32, character_code: u32, is_repeat: bool) {
        let fun: extern "C" fn(this: uintptr_t, key_code: int32_t, character_code: uint32_t, is_repeat: uint32_t) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONKEYUP) };
        fun(*SLATEAPP.get() as uintptr_t, key_code, character_code, is_repeat as u32)
    }

    pub(in native) fn on_raw_mouse_move(x: i32, y: i32) {
        let fun: extern "C" fn(this: uintptr_t, x: int32_t, y: int32_t) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONRAWMOUSEMOVE) };
        fun(*SLATEAPP.get() as uintptr_t, x, y)
    }
}

#[inline(never)]
pub(in native) extern "C" fn save(this: usize) {
    SLATEAPP.set(this);
    log!("Got FSlateApplication: {:#x}", this);
}

pub(in native) extern "C" fn key_down(_this: usize, key_code: i32, character_code: u32, is_repeat: bool) {
    // on Linux UE applies a (1<<30) mask to mod keys
    ::threads::ue::key_down(key_code & !(1<<30), character_code, is_repeat);
}

pub(in native) extern "C" fn key_up(_this: usize, key_code: i32, character_code: u32, is_repeat: bool) {
    // on Linux UE applies a (1<<30) mask to mod keys
    ::threads::ue::key_up(key_code & !(1<<30), character_code, is_repeat);
}
