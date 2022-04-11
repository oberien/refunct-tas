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
mod level_state;
mod gameinstance;
mod platform_misc;

#[cfg(unix)] use self::linux::*;
#[cfg(windows)] use self::windows::*;

#[cfg(unix)] pub use self::linux::INITIALIZE_CTOR;
#[cfg(windows)] pub use self::windows::{DllMain, suspend_threads, resume_threads};
pub use self::character::AMyCharacter;
pub use self::slateapp::{
    FSlateApplication,
    unhook_fslateapplication_onkeydown,
    hook_fslateapplication_onkeydown,
    unhook_fslateapplication_onkeyup,
    hook_fslateapplication_onkeyup,
    unhook_fslateapplication_onrawmousemove,
    hook_fslateapplication_onrawmousemove,
};
pub use self::app::FApp;
pub use self::memory::FMemory;
pub use self::hud::AMyHud;
pub use self::uworld::{APawn, UWorld};
pub use self::level_state::LevelState;
pub use self::platform_misc::FPlatformMisc;

pub fn init() {
    #[cfg(windows)] windows::init();
    #[cfg(unix)] linux::init();
    slateapp::hook_fslateapplication_tick();
    slateapp::hook_fslateapplication_onkeydown();
    slateapp::hook_fslateapplication_onkeyup();
    slateapp::hook_fslateapplication_onrawmousemove();
    newgame::hook_amycharacter_forceduncrouch();
    tick::hook_uengine_updatetimeandhandlemaxtickrate();
    hud::hook_amyhud_drawhud();
    character::hook_amycharacter_tick();
}
