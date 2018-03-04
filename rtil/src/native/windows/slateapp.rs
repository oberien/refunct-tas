use native::slateapp::{SLATEAPP, FSlateApplication};

use winapi::minwindef::BOOL;
use winapi::basetsd::{UINT_PTR, INT32, UINT32};

use native::{
    FSLATEAPPLICATION_ONKEYDOWN,
    FSLATEAPPLICATION_ONKEYUP,
    FSLATEAPPLICATION_ONRAWMOUSEMOVE,
};

impl FSlateApplication {
    pub(in native) fn on_key_down(key_code: i32, character_code: u32, is_repeat: bool) {
        let fun: extern "thiscall" fn(this: UINT_PTR, key_code: INT32, character_code: UINT32, is_repeat: BOOL) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONKEYDOWN) };
        fun(*SLATEAPP.get() as UINT_PTR, key_code, character_code, is_repeat as BOOL)
    }
    pub(in native) fn on_key_up(key_code: i32, character_code: u32, is_repeat: bool) {
        let fun: extern "thiscall" fn(this: UINT_PTR, key_code: INT32, character_code: UINT32, is_repeat: BOOL) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONKEYUP) };
        fun(*SLATEAPP.get() as UINT_PTR, key_code, character_code, is_repeat as BOOL)
    }

    pub(in native) fn on_raw_mouse_move(x: i32, y: i32) {
        let fun: extern "thiscall" fn(this: UINT_PTR, x: INT32, y: INT32) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONRAWMOUSEMOVE) };
        fun(*SLATEAPP.get() as UINT_PTR, x, y)
    }
}

#[inline(never)]
pub(in native) extern "thiscall" fn save(this: usize) {
    SLATEAPP.set(this + 0x3c);
    log!("Got FSlateApplication: {:#x}", this);
}

pub(in native) extern "thiscall" fn key_down(_this: usize, key_code: i32, character_code: u32, is_repeat: bool) {
    ::threads::ue::key_down(key_code, character_code, is_repeat);
}

pub(in native) extern "thiscall" fn key_up(_this: usize, key_code: i32, character_code: u32, is_repeat: bool) {
    ::threads::ue::key_up(key_code, character_code, is_repeat);
}
