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
pub(crate) mod ue;
pub(crate) mod character;
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
mod gameusersettings;
mod reflection;
mod map_editor;
mod kismet_system_library;
mod engine;
mod font;

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
pub use self::tick::{
    hook_aliftbase_addbasedcharacter,
    hook_aliftbase_removebasedcharacter,
    unhook_aliftbase_addbasedcharacter,
    unhook_aliftbase_removebasedcharacter,
};
pub use self::app::FApp;
pub use self::memory::FMemory;
pub use self::hud::{AMyHud, EBlendMode};
pub use self::uworld::{APawn, UWorld, UGameplayStatics, TimeOfDay};
pub use self::level_state::LevelState;
pub use self::platform_misc::FPlatformMisc;
pub use self::texture::UTexture2D;
pub use self::gameinstance::UMyGameInstance;
pub use self::reflection::*;
pub use self::map_editor::*;
pub use self::kismet_system_library::KismetSystemLibrary;
pub use self::engine::UEngine;
pub use self::font::UFont;

/// Rebo code must only be executed once all `this*` have been found.
/// There are currently 3 such `this`-pointers - rebo starts once the semaphore reaches 1.
pub static REBO_DOESNT_START_SEMAPHORE: Semaphore = Semaphore::new(-2);

pub fn init() {
    #[cfg(windows)] windows::init();
    #[cfg(unix)] linux::init();
    uworld::init();
    map_editor::init();
    font::init();
    slateapp::hook_fslateapplication_tick();
    slateapp::hook_fslateapplication_onkeydown();
    slateapp::hook_fslateapplication_onkeyup();
    slateapp::hook_fslateapplication_onrawmousemove();
    newgame::hook_amycharacter_forceduncrouch();
    tick::hook_uengine_updatetimeandhandlemaxtickrate();
    tick::hook_aliftbase_addbasedcharacter();
    tick::hook_aliftbase_removebasedcharacter();
    hud::hook_amyhud_drawhud();
    character::hook_amycharacter_tick();
    gameusersettings::hook_ugameusersettings_applyresolutionsettings();
    uworld::hook_uuserwidget_addtoscreen();
}

#[cfg(unix)]
#[repr(C)]
#[derive(Debug)]
struct Args {
    // xmm7, xmm6, xmm5, xmm4, xmm3, xmm2, xmm1, xmm0
    xmm7_0: [u128; 8],
    // rdi, rsi, rdx, rcx, r8, r9
    args: [usize; 6],
    _r10: usize,
    _r11: usize,
    _rax: usize,
}

#[doc(hidden)]
// only used internally by Args accessing
const WINDOWS_MAX_ARG_NUM: usize = 0x60/::std::mem::size_of::<usize>();
#[cfg(windows)]
#[repr(C)]
#[derive(Debug)]
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
    other_args: [usize; WINDOWS_MAX_ARG_NUM],
}

#[doc(hidden)]
// only used internally by Args accessing
trait AccessArgs {
    type Pointer;
    type Output<'a> where Self: 'a;
    unsafe fn access(args: &mut ArgsAccessWrapper<'_>) -> Self::Pointer;
    unsafe fn convert_to_output<'a>(ptr: Self::Pointer) -> Self::Output<'a>;
}
macro_rules! impl_access_args {
    ($($t:ty => $next_fn:ident,)*) => {
        $(
            impl AccessArgs for $t {
                type Pointer = *mut $t;
                type Output<'a> = &'a mut $t;
                unsafe fn access(args: &mut ArgsAccessWrapper<'_>) -> Self::Pointer {
                    args.$next_fn() as *mut _ as *mut $t
                }
                unsafe fn convert_to_output<'a>(ptr: Self::Pointer) -> Self::Output<'a> {
                    &mut *ptr
                }
            }
        )*
    };
}
// SAFETY: casts to smaller types are allowed as we only support little-endian platforms
impl_access_args! {
    f32 => next_float_arg,
    usize => next_int_arg,
    u8 => next_int_arg,
    u16 => next_int_arg,
    u32 => next_int_arg,
    i8 => next_int_arg,
    i16 => next_int_arg,
    i32 => next_int_arg,
    // not greater than 32bit supported (apart from usize on linux)
    // because we've only confirmed this for windows' thiscall for word-size arguments
}
macro_rules! impl_access_args_for_tuple {
    ($($t:ident),*) => {
        impl<$($t: AccessArgs),*> AccessArgs for ($($t,)*) {
            type Pointer = ($($t::Pointer,)*);
            type Output<'a> = ($($t::Output<'a>,)*) where Self: 'a;
            unsafe fn access(args: &mut ArgsAccessWrapper<'_>) -> Self::Pointer {
                ($(<$t as AccessArgs>::access(args),)*)
            }
            unsafe fn convert_to_output<'a>(ptr: Self::Pointer) -> Self::Output<'a> {
                #[allow(non_snake_case)]
                let ($($t,)*) = ptr;
                ($($t::convert_to_output($t),)*)
            }
        }
    };
}
// currently our windows implementation only allows this* + 12 args -> impl for max 13 args
impl_access_args_for_tuple!(A);
impl_access_args_for_tuple!(A, B);
impl_access_args_for_tuple!(A, B, C);
impl_access_args_for_tuple!(A, B, C, D);
impl_access_args_for_tuple!(A, B, C, D, E);
impl_access_args_for_tuple!(A, B, C, D, E, F);
impl_access_args_for_tuple!(A, B, C, D, E, F, G);
impl_access_args_for_tuple!(A, B, C, D, E, F, G, H);
impl_access_args_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_access_args_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_access_args_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_access_args_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_access_args_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl<T: 'static> AccessArgs for *mut T {
    type Pointer = *mut T;
    type Output<'a> = *mut T;
    unsafe fn access(args: &mut ArgsAccessWrapper<'_>) -> *mut T {
        // we get a pointer to the value in the args-struct, which is the
        // actual pointer we want, so we need to deref
        *(args.next_int_arg() as *mut _ as *mut *mut T)
    }
    unsafe fn convert_to_output<'a>(ptr: Self::Pointer) -> Self::Output<'a> {
        ptr
    }
}

#[doc(hidden)]
// only used internally by Args accessing
struct ArgsAccessWrapper<'a> {
    args: &'a mut Args,
    has_this_pointer: bool,
    num_int_args: usize,
    num_float_args: usize,
}
impl<'a> ArgsAccessWrapper<'a> {
    unsafe fn take_this_pointer(&mut self) -> *mut usize {
        assert!(self.has_this_pointer);
        assert_eq!(self.num_int_args, 0);
        self.has_this_pointer = false;
        #[cfg(unix)] {
            self.num_int_args += 1;
            &mut self.args.args[0]
        }
        #[cfg(windows)] {
            &mut self.args.ecx
        }
    }
    unsafe fn next_int_arg(&mut self) -> *mut usize {
        if self.has_this_pointer {
            return self.take_this_pointer();
        }
        // our linux impl only supports rdi, rsi, rdx, rcx, r8, r9 for int-args and not the stack
        assert!(1 + self.num_int_args <= 6);
        // our windows impl only supports WINDOWS_MAX_ARG_LEN word-size arguments
        // usize on windows is 32bits as Refunct is a Win32 binary
        assert!(1 + self.num_int_args + self.num_float_args <= WINDOWS_MAX_ARG_NUM);

        let index = self.num_int_args;
        self.num_int_args += 1;
        #[cfg(unix)] {
            &mut self.args.args[index]
        }
        #[cfg(windows)] {
            &mut self.args.other_args[index + self.num_float_args]
        }
    }
    unsafe fn next_float_arg(&mut self) -> *mut f32 {
        assert!(!self.has_this_pointer);
        // our linux impl only supports xmm0-xmm7 for float-args and not the stack
        assert!(1 + self.num_float_args <= 8);
        // our windows impl only supports WINDOWS_MAX_ARG_LEN word-size arguments
        // usize on windows is 32bits as Refunct is a Win32 binary
        assert!(1 + self.num_int_args + self.num_float_args <= WINDOWS_MAX_ARG_NUM);

        let index = self.num_float_args;
        self.num_float_args += 1;
        #[cfg(unix)] {
            // SAFETY: xmmX is stored on the stack in little endian
            &mut self.args.xmm7_0[7 - index]
                as *mut u128
                as *mut f32
        }
        #[cfg(windows)] {
            // SAFETY: all args in thiscall are word-size, i.e., both usize and f32 are word-size
            &mut self.args.other_args[index + self.num_int_args]
                as *mut usize
                as *mut f32
        }
    }
}

impl Args {
    unsafe fn with_this_pointer<'a, T: AccessArgs>(&'a mut self) -> T::Output<'a> {
        T::convert_to_output(T::access(&mut ArgsAccessWrapper {
            args: self,
            has_this_pointer: true,
            num_int_args: 0,
            num_float_args: 0,
        }))
    }
    unsafe fn _without_this_pointer<'a, T: AccessArgs>(&'a mut self) -> T::Output<'a> {
        T::convert_to_output(T::access(&mut ArgsAccessWrapper {
            args: self,
            has_this_pointer: false,
            num_int_args: 0,
            num_float_args: 0,
        }))
    }
}
