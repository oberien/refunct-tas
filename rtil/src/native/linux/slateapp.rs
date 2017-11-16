use native::SLATEAPP;

use libc::{uintptr_t, int32_t, uint32_t};

use super::{
    FSLATEAPPLICATION_TICK,
    FSLATEAPPLICATION_ONKEYDOWN,
    FSLATEAPPLICATION_ONKEYUP,
    FSLATEAPPLICATION_ONRAWMOUSEMOVE,
};

pub struct FSlateApplication;

impl FSlateApplication {
    pub unsafe fn on_key_down(key_code: i32, character_code: u32, is_repeat: bool) {
        let fun: unsafe extern fn(this: uintptr_t, key_code: int32_t, character_code: uint32_t, is_repeat: uint32_t) =
                ::std::mem::transmute(FSLATEAPPLICATION_ONKEYDOWN);
        fun(*SLATEAPP.get() as uintptr_t, key_code, character_code, is_repeat as u32)
    }
    pub unsafe fn on_key_up(key_code: i32, character_code: u32, is_repeat: bool) {
        let fun: unsafe extern fn(this: uintptr_t, key_code: int32_t, character_code: uint32_t, is_repeat: uint32_t) =
                ::std::mem::transmute(FSLATEAPPLICATION_ONKEYUP);
        fun(*SLATEAPP.get() as uintptr_t, key_code, character_code, is_repeat as u32)
    }

    pub unsafe fn on_raw_mouse_move(x: i32, y: i32) {
        let fun: unsafe extern fn(this: uintptr_t, x: int32_t, y: int32_t) =
                ::std::mem::transmute(FSLATEAPPLICATION_ONRAWMOUSEMOVE);
        fun(*SLATEAPP.get() as uintptr_t, x, y)
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
extern fn save_slateapp(this: usize) {
    SLATEAPP.set(this);
    log!("Got FSlateApplication: {:#x}", this);
}
