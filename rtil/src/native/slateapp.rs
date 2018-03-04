use native::FSLATEAPPLICATION_TICK;
#[cfg(unix)] use native::linux::slateapp::save;
#[cfg(windows)] use native::windows::slateapp::save;

lazy_static! {
    pub(in native) static ref SLATEAPP: Static<usize> = Static::new();
}

pub struct FSlateApplication;

impl FSlateApplication {
    pub fn press_key(key: i32) {
        FSlateApplication::on_key_down(key, key as u32, false);
    }

    pub fn release_key(key: i32) {
        FSlateApplication::on_key_up(key, key as u32, false);
    }

    pub fn move_mouse(x: i32, y: i32) {
        FSlateApplication::on_raw_mouse_move(x, y);
    }
}

hook! {
    "FSlateApplication::Tick",
    FSLATEAPPLICATION_TICK,
    hook,
    unhook,
    get,
    true,
}

hook_fn_once! {
    get,
    save,
    unhook,
    FSLATEAPPLICATION_TICK,
}

mod fixme {
    use native::FSLATEAPPLICATION_ONKEYDOWN;
    #[cfg(unix)] use native::linux::slateapp::key_down;
    #[cfg(windows)] use native::windows::slateapp::key_down;
    hook! {
        "FSlateApplication::OnKeyDown",
        FSLATEAPPLICATION_ONKEYDOWN,
        hook_keydown,
        unhook_keydown,
        keydown,
        false,
    }

    hook_fn_always! {
        keydown,
        key_down,
        hook_keydown,
        unhook_keydown,
        FSLATEAPPLICATION_ONKEYDOWN,
        intercept before original,
    }
}
pub use self::fixme::{hook_keydown, unhook_keydown};

mod fixme2 {
    use native::FSLATEAPPLICATION_ONKEYUP;
    #[cfg(unix)] use native::linux::slateapp::key_up;
    #[cfg(windows)] use native::windows::slateapp::key_up;
    hook! {
        "FSlateApplication::OnKeyUp",
        FSLATEAPPLICATION_ONKEYUP,
        hook_keyup,
        unhook_keyup,
        keyup,
        false,
    }

    hook_fn_always! {
        keyup,
        key_up,
        hook_keyup,
        unhook_keyup,
        FSLATEAPPLICATION_ONKEYUP,
        intercept before original,
    }
}
pub use self::fixme2::{hook_keyup, unhook_keyup};
