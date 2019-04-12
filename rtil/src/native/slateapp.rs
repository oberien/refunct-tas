use native::FSLATEAPPLICATION_TICK;
#[cfg(unix)] use native::linux::slateapp::save;
#[cfg(windows)] use native::windows::slateapp::save;

lazy_static! {
    pub(in native) static ref SLATEAPP: Static<usize> = Static::new();
}

pub struct FSlateApplication;

impl FSlateApplication {
    pub fn press_key(key: i32, code: u32, repeat: bool) {
        FSlateApplication::on_key_down(key, code, repeat);
    }

    pub fn release_key(key: i32, code: u32, repeat: bool) {
        FSlateApplication::on_key_up(key, code, repeat);
    }

    pub fn move_mouse(x: i32, y: i32) {
        FSlateApplication::on_raw_mouse_move(x, y);
    }
}

#[rtil_derive::hook_once(FSlateApplication::Tick)]
fn save(this: usize) {
    #[cfg(unix)] { SLATEAPP.set(this); }
    #[cfg(winodws)] { SLATEAPP.set(this + 0x3c); }
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
