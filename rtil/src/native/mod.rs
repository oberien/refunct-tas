#[cfg(unix)]
macro_rules! extern_fn {
    (fn ($($argname:ident : $argtype:ty),*)) => {
        extern "C" fn($($argname : $argtype),*)
    };
    (fn ($($argname:ident : $argtype:ty),*) -> $rettype:ty) => {
        extern "C" fn($($argtype),*) -> $rettype
    };
}
#[cfg(windows)]
macro_rules! extern_fn {
    (fn ($($argname:ident : $argtype:ty),*)) => {
        extern "thiscall" fn($($argname : $argtype),*)
    };
    (fn ($($argname:ident : $argtype:ty),*) -> $rettype:ty) => {
        extern "thiscall" fn($($argtype),*) -> $rettype
    };
}

#[cfg(unix)] mod linux;
#[cfg(windows)] mod windows;
mod ue;
mod character;
mod newgame;
mod slateapp;
mod tick;
mod app;
mod memory;
mod hud;
mod uworld;

#[cfg(unix)] use self::linux::*;
#[cfg(windows)] use self::windows::*;

#[cfg(unix)] pub use self::linux::INITIALIZE_CTOR;
#[cfg(windows)] pub use self::windows::DllMain;
pub use self::character::AMyCharacter;
pub use self::slateapp::{
    FSlateApplication,
    unhook_fslateapplication_onkeydown,
    hook_fslateapplication_onkeydown,
    unhook_fslateapplication_onkeyup,
    hook_fslateapplication_onkeyup,
};
pub use self::app::FApp;
pub use self::memory::FMemory;
pub use self::hud::AMyHud;
pub use self::uworld::{APawn, UWorld};

pub fn init() {
    #[cfg(windows)] windows::init();
    #[cfg(unix)] linux::init();
    slateapp::hook_fslateapplication_tick();
    slateapp::hook_fslateapplication_onkeydown();
    slateapp::hook_fslateapplication_onkeyup();
    newgame::hook_amycharacter_forceduncrouch();
    tick::hook_uengine_updatetimeandhandlemaxtickrate();
    hud::hook_amyhud_drawhud();
    character::hook_amycharacter_tick();
}
