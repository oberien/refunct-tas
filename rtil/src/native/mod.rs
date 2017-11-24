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

#[cfg(unix)] use self::linux::*;
#[cfg(windows)] use self::windows::*;

#[cfg(unix)] pub use self::linux::INITIALIZE_CTOR;
#[cfg(windows)] pub use self::windows::DllMain;
pub use self::character::AMyCharacter;
pub use self::controller::AController;
pub use self::slateapp::FSlateApplication;
pub use self::app::FApp;
pub use self::memory::FMemory;

pub fn init() {
    #[cfg(windows)] windows::init();
    #[cfg(unix)] linux::init();
    slateapp::hook();
    newgame::hook();
    tick::hook();
    controller::hook();
    character::hook();
}
