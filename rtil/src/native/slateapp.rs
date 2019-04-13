use native::{FSLATEAPPLICATION_ONKEYDOWN, FSLATEAPPLICATION_ONKEYUP, FSLATEAPPLICATION_ONRAWMOUSEMOVE};
use crate::statics::Static;

lazy_static! {
    static ref SLATEAPP: Static<usize> = Static::new();
}

pub struct FSlateApplication;

impl FSlateApplication {
    fn on_key_down(key_code: i32, character_code: u32, is_repeat: u32) {
        let fun: extern_fn!(fn(this: usize, key_code: i32, character_code: u32, is_repeat: u32)) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONKEYDOWN) };
        fun(*SLATEAPP.get(), key_code, character_code, is_repeat)
    }
    fn on_key_up(key_code: i32, character_code: u32, is_repeat: u32) {
        let fun: extern_fn!(fn(this: usize, key_code: i32, character_code: u32, is_repeat: u32)) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONKEYUP) };
        fun(*SLATEAPP.get(), key_code, character_code, is_repeat)
    }
    fn on_raw_mouse_move(x: i32, y: i32) {
        let fun: extern_fn!(fn(this: usize, x: i32, y: i32)) =
            unsafe { ::std::mem::transmute(FSLATEAPPLICATION_ONRAWMOUSEMOVE) };
        fun(*SLATEAPP.get(), x, y)
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
fn save(this: usize) {
    #[cfg(unix)] { SLATEAPP.set(this); }
    #[cfg(windows)] { SLATEAPP.set(this + 0x3c); }
    log!("Got FSlateApplication: {:#x}", this);
}

#[rtil_derive::hook_before(FSlateApplication::OnKeyDown)]
fn key_down(_this: usize, key_code: i32, character_code: u32, is_repeat: bool) {
    #[cfg(unix)] {
        // on Linux UE applies a (1<<30) mask to mod keys
        ::threads::ue::key_down(key_code & !(1<<30), character_code, is_repeat);
    }
    #[cfg(windows)] {
        ::threads::ue::key_down(key_code, character_code, is_repeat);
    }
}

#[rtil_derive::hook_before(FSlateApplication::OnKeyUp)]
fn key_up(_this: usize, key_code: i32, character_code: u32, is_repeat: bool) {
    #[cfg(unix)] {
        // on Linux UE applies a (1<<30) mask to mod keys
        ::threads::ue::key_up(key_code & !(1 << 30), character_code, is_repeat);
    }
    #[cfg(windows)] {
        ::threads::ue::key_up(key_code, character_code, is_repeat);
    }
}
