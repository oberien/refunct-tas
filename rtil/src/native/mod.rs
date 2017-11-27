#[cfg(unix)] #[macro_use] mod linux;
#[cfg(windows)] #[macro_use] mod windows;
mod ue;
mod character;
mod controller;
mod newgame;
mod slateapp;
mod tick;
mod app;
mod memory;
mod hud;

#[cfg(unix)] use self::linux::*;
#[cfg(windows)] use self::windows::*;

#[cfg(unix)] pub use self::linux::INITIALIZE_CTOR;
#[cfg(windows)] pub use self::windows::DllMain;
pub use self::character::AMyCharacter;
pub use self::controller::AController;
pub use self::slateapp::{FSlateApplication, unhook_keydown, hook_keydown, unhook_keyup, hook_keyup};
pub use self::app::FApp;
pub use self::memory::FMemory;
pub use self::hud::AMyHud;

pub fn init() {
    #[cfg(windows)] windows::init();
    #[cfg(unix)] linux::init();
    slateapp::hook();
    slateapp::hook_keydown();
    slateapp::hook_keyup();
    newgame::hook();
    tick::hook();
    hud::hook();
    controller::hook();
    character::hook();
}
