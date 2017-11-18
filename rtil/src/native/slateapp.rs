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

