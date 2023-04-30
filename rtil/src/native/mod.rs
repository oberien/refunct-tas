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
mod texture;

use crate::semaphore::Semaphore;
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
pub use self::uworld::{APawn, UWorld, UGameplayStatics};
pub use self::level_state::LevelState;
pub use self::platform_misc::FPlatformMisc;
pub use self::texture::{UTexture2D, UTexture2DUE};

/// Rebo code must only be executed once all `this*` have been found.
/// There are currently 3 such `this`-pointers - rebo starts once the semaphore reaches 1.
pub static REBO_DOESNT_START_SEMAPHORE: Semaphore = Semaphore::new(-2);

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

#[cfg(unix)]
#[repr(C)]
struct Args {
    _xmm7: u128,
    _xmm6: u128,
    _xmm5: u128,
    _xmm4: u128,
    _xmm3: u128,
    _xmm2: u128,
    _xmm1: u128,
    _xmm0: u128,
    // rdi, rsi, rdx, rcx, r8, r9
    args: [usize; 6],
    _r10: usize,
    _r11: usize,
    _rax: usize,
}

#[cfg(windows)]
#[repr(C)]
struct Args {
    _xmm7: u128,
    _xmm6: u128,
    _xmm5: u128,
    _xmm4: u128,
    _xmm3: u128,
    _xmm2: u128,
    _xmm1: u128,
    _xmm0: u128,
    _ebp: usize,
    _edi: usize,
    _esi: usize,
    _edx: usize,
    ecx: usize,
    _ebx: usize,
    _eax: usize,
    _ret: usize,
    other_args: [usize; 0x60/::std::mem::size_of::<usize>()],
}

impl Args {
    /// Return the nth of the first 6 integer arguments
    ///
    /// # Panic
    /// Panics if `n` is larger than 5.
    ///
    /// # Safety
    /// The caller must ensure that at least `n` integer args are passed to the original function.
    unsafe fn nth_integer_arg(&self, n: usize) -> usize {
        if n > 5 {
            panic!("Args::nth_integer_arg called with number greater than 5: {n}");
        }
        #[cfg(unix)] {
            self.args[n]
        }
        #[cfg(windows)] {
            match n {
                0 => self.ecx,
                n => self.other_args[n-1],
            }
        }
    }

    /// Set the nth of the first 6 integer arguments to the given value
    ///
    /// # Panic
    /// Panics if `n` is larger than 5.
    ///
    /// # Safety
    /// The caller must ensure that at least `n` integer args are passed to the original function.
    unsafe fn set_nth_integer_arg(&mut self, n: usize, val: usize) {
        if n > 5 {
            panic!("Args::nth_integer_arg called with number greater than 5: {n}");
        }
        #[cfg(unix)] {
            self.args[n] = val;
        }
        #[cfg(windows)] {
            match n {
                0 => self.ecx = val,
                n => self.other_args[n-1] = val,
            }
        }
    }
}
