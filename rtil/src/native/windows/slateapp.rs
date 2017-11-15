use winapi::minwindef::BOOL;
use winapi::basetsd::{UINT_PTR, INT32, UINT32};

use native::SLATEAPP;
use super::{
    FSLATEAPPLICATION,
    FSLATEAPPLICATION_TICK,
    FSLATEAPPLICATION_ONKEYDOWN,
    FSLATEAPPLICATION_ONKEYUP,
    FSLATEAPPLICATION_ONRAWMOUSEMOVE,
};

pub struct FSlateApplication;

impl FSlateApplication {
    pub unsafe fn on_key_down(key_code: i32, character_code: u32, is_repeat: bool) {
        let fun: unsafe extern "thiscall" fn(this: UINT_PTR, key_code: INT32, character_code: UINT32, is_repeat: BOOL) =
            ::std::mem::transmute(FSLATEAPPLICATION_ONKEYDOWN);
        fun(FSLATEAPPLICATION as UINT_PTR, key_code, character_code, is_repeat as BOOL)
    }
    pub unsafe fn on_key_up(key_code: i32, character_code: u32, is_repeat: bool) {
        let fun: unsafe extern "thiscall" fn(this: UINT_PTR, key_code: INT32, character_code: UINT32, is_repeat: BOOL) =
            ::std::mem::transmute(FSLATEAPPLICATION_ONKEYUP);
        fun(FSLATEAPPLICATION as UINT_PTR, key_code, character_code, is_repeat as BOOL)
    }

    pub unsafe fn on_raw_mouse_move(x: i32, y: i32) {
        let fun: unsafe extern "thiscall" fn(this: UINT_PTR, x: INT32, y: INT32) =
            ::std::mem::transmute(FSLATEAPPLICATION_ONRAWMOUSEMOVE);
        fun(FSLATEAPPLICATION as UINT_PTR, x, y)
    }
}

hook! {
    "FSlateApplication::Tick",
    FSLATEAPPLICATION_TICK,
    hook_slateapp,
    unhook_slateapp,
    get_slateapp,
    true,
}

hook_fn_once! {
    get_slateapp,
    save_slateapp,
    unhook_slateapp,
    FSLATEAPPLICATION_TICK,
}

#[inline(never)]
extern "thiscall" fn save_slateapp(this: usize) {
    SLATEAPP.set(this + 0x3c);
    unsafe { FSLATEAPPLICATION = this + 0x3c };
    log!("Got FSlateApplication: {:#x}", this);
}
