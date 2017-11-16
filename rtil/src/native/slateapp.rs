use native::FSLATEAPPLICATION_TICK;

#[cfg(unix)] use native::linux::slateapp::save;
#[cfg(windows)] use native::windows::slateapp::save;

lazy_static! {
    pub(in native) static ref SLATEAPP: Static<usize> = Static::new();
}

pub struct FSlateApplication;

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

