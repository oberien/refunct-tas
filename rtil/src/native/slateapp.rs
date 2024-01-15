use std::sync::atomic::{AtomicPtr, Ordering};
use crate::native::{Args, FSLATEAPPLICATION_ONKEYDOWN, FSLATEAPPLICATION_ONKEYUP, FSLATEAPPLICATION_ONRAWMOUSEMOVE, REBO_DOESNT_START_SEMAPHORE};

static SLATEAPP: AtomicPtr<FSlateApplication> = AtomicPtr::new(std::ptr::null_mut());

pub enum FSlateApplication {}

macro_rules! get_fslateapplication {
    ($fnname:literal) => {{
        let slateapp = SLATEAPP.load(Ordering::SeqCst);
        if slateapp.is_null() {
            let msg = concat!("called FSlateApplication::", $fnname, " while FSlateApplication-pointer wasn't initialized yet");
            log!("{}", msg);
            panic!("{}", msg);
        }
        slateapp
    }}
}

impl FSlateApplication {
    fn on_key_down(key_code: i32, character_code: u32, is_repeat: u32) {
        let fun: extern_fn!(fn(this: *mut FSlateApplication, key_code: i32, character_code: u32, is_repeat: u32)) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONKEYDOWN.load(Ordering::SeqCst)) };
        fun(get_fslateapplication!("on_key_down"), key_code, character_code, is_repeat)
    }
    fn on_key_up(key_code: i32, character_code: u32, is_repeat: u32) {
        let fun: extern_fn!(fn(this: *mut FSlateApplication, key_code: i32, character_code: u32, is_repeat: u32)) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONKEYUP.load(Ordering::SeqCst)) };
        fun(get_fslateapplication!("on_key_up"), key_code, character_code, is_repeat)
    }
    fn on_raw_mouse_move(x: i32, y: i32) {
        let fun: extern_fn!(fn(this: *mut FSlateApplication, x: i32, y: i32)) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONRAWMOUSEMOVE.load(Ordering::SeqCst)) };
        fun(get_fslateapplication!("on_raw_mouse_move"), x, y)
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
fn save(args: &mut Args) {
    let this: *mut FSlateApplication = unsafe { args.with_this_pointer::<*mut FSlateApplication>() };
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
    REBO_DOESNT_START_SEMAPHORE.release();
}

#[rtil_derive::hook_before(FSlateApplication::OnKeyDown)]
fn key_down(args: &mut Args) {
    let (
        _this, key_code, character_code, is_repeat
    ) = unsafe { args.with_this_pointer::<(usize, i32, u32, usize)>() };
    let is_repeat = *is_repeat & 0xff != 0;
    #[cfg(unix)] {
        // on Linux UE applies a (1<<30) mask to mod keys
        crate::threads::ue::key_down(*key_code & !(1<<30), *character_code, is_repeat);
    }
    #[cfg(windows)] {
        crate::threads::ue::key_down(*key_code, *character_code, is_repeat);
    }
}

#[rtil_derive::hook_before(FSlateApplication::OnKeyUp)]
fn key_up(args: &mut Args) {
    let (
        _this, key_code, character_code, is_repeat
    ) = unsafe { args.with_this_pointer::<(usize, i32, u32, usize)>() };
    let is_repeat = *is_repeat & 0xff != 0;
    #[cfg(unix)] {
        // on Linux UE applies a (1<<30) mask to mod keys
        crate::threads::ue::key_up(*key_code & !(1 << 30), *character_code, is_repeat);
    }
    #[cfg(windows)] {
        crate::threads::ue::key_up(*key_code, *character_code, is_repeat);
    }
}

#[rtil_derive::hook_before(FSlateApplication::OnRawMouseMove)]
fn on_raw_mouse_move(args: &mut Args) {
    let (_this, x, y) = unsafe { args.with_this_pointer::<(usize, i32, i32)>() };
    crate::threads::ue::mouse_move(*x, *y);
}
