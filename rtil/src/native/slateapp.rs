use std::sync::atomic::{AtomicPtr, Ordering};
use std::os::raw::c_void;
use crate::native::{FSLATEAPPLICATION_ONKEYDOWN, FSLATEAPPLICATION_ONKEYUP, FSLATEAPPLICATION_ONRAWMOUSEMOVE};

static SLATEAPP: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

pub struct FSlateApplication;

impl FSlateApplication {
    fn on_key_down(key_code: i32, character_code: u32, is_repeat: u32) {
        let fun: extern_fn!(fn(this: *mut c_void, key_code: i32, character_code: u32, is_repeat: u32)) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONKEYDOWN.load(Ordering::SeqCst)) };
        fun(SLATEAPP.load(Ordering::SeqCst), key_code, character_code, is_repeat)
    }
    fn on_key_up(key_code: i32, character_code: u32, is_repeat: u32) {
        let fun: extern_fn!(fn(this: *mut c_void, key_code: i32, character_code: u32, is_repeat: u32)) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONKEYUP.load(Ordering::SeqCst)) };
        fun(SLATEAPP.load(Ordering::SeqCst), key_code, character_code, is_repeat)
    }
    fn on_raw_mouse_move(x: i32, y: i32) {
        let fun: extern_fn!(fn(this: *mut c_void, x: i32, y: i32)) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONRAWMOUSEMOVE.load(Ordering::SeqCst)) };
        fun(SLATEAPP.load(Ordering::SeqCst), x, y)
    }

    pub fn press_key(key: i32, code: u32, repeat: bool) {
        FSlateApplication::on_key_down(key, code, repeat as u32);
    }

    pub fn release_key(key: i32, code: u32, repeat: bool) {
        FSlateApplication::on_key_up(key, code, repeat as u32);
    }

    pub fn move_mouse(x: i32, y: i32) {
        FSlateApplication::on_raw_mouse_move(x, y);
    }
}

#[rtil_derive::hook_once(FSlateApplication::Tick)]
fn save(this: *mut c_void) {
    #[cfg(unix)] { SLATEAPP.store(this, Ordering::SeqCst); }
    #[cfg(windows)] {
        let this_addr = this as usize;
        // don't ask why this offset is needed, it's there since Feb 2017
        // introduced in 882dc51a5345deb50f3166a4ce4855133c993fb8
        // and it works, so don't touch it
        let this_fixed_addr = this_addr + 0x3c;
        SLATEAPP.store(this_fixed_addr as *mut _, Ordering::SeqCst);
    }
    log!("Got FSlateApplication: {:#x}", this as usize);
}

#[rtil_derive::hook_before(FSlateApplication::OnKeyDown)]
fn key_down(_this: usize, key_code: i32, character_code: u32, is_repeat: bool) {
    #[cfg(unix)] {
        // on Linux UE applies a (1<<30) mask to mod keys
        crate::threads::ue::key_down(key_code & !(1<<30), character_code, is_repeat);
    }
    #[cfg(windows)] {
        crate::threads::ue::key_down(key_code, character_code, is_repeat);
    }
}

#[rtil_derive::hook_before(FSlateApplication::OnKeyUp)]
fn key_up(_this: usize, key_code: i32, character_code: u32, is_repeat: bool) {
    #[cfg(unix)] {
        // on Linux UE applies a (1<<30) mask to mod keys
        crate::threads::ue::key_up(key_code & !(1 << 30), character_code, is_repeat);
    }
    #[cfg(windows)] {
        crate::threads::ue::key_up(key_code, character_code, is_repeat);
    }
}

#[rtil_derive::hook_before(FSlateApplication::OnRawMouseMove)]
fn on_raw_mouse_move(_this: usize, x: i32, y: i32) {
    crate::threads::ue::mouse_move(x, y);
}
