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

use std::sync::atomic::Ordering;
use hook::{RawHook, TypedHook};
use crate::native::character::AMyCharacterUE;
use crate::native::hud::{AHudUE, UMaterialInterfaceUE};
use crate::semaphore::Semaphore;
#[cfg(unix)] use self::linux::*;
#[cfg(windows)] use self::windows::*;

#[cfg(unix)] pub use self::linux::INITIALIZE_CTOR;
#[cfg(windows)] pub use self::windows::{DllMain, suspend_threads, resume_threads};
pub use self::character::AMyCharacter;
pub use self::slateapp::{
    FSlateApplication,
    EMouseButtonsType,
};
pub use self::app::FApp;
pub use self::memory::FMemory;
pub use self::hud::{AMyHud, EBlendMode};
pub use self::uworld::{UWorld, UGameplayStatics, TimeOfDay};
pub use self::level_state::LevelState;
pub use self::platform_misc::FPlatformMisc;
pub use self::texture::{UTexture2D, EPixelFormat};
pub use self::gameinstance::UMyGameInstance;
pub use self::reflection::*;
pub use self::map_editor::*;
pub use self::kismet_system_library::KismetSystemLibrary;
pub use self::engine::{UEngine, FViewport, UWidgetBlueprintLibrary};

/// Rebo code must only be executed once all `this*` have been found.
/// There are currently 3 such `this`-pointers - rebo starts once the semaphore reaches 1.
pub static REBO_DOESNT_START_SEMAPHORE: Semaphore = Semaphore::new(-2);

#[cfg(unix)]
type RefunctIsaAbi = hook::X86_64_SystemV;
#[cfg(windows)]
type RefunctIsaAbi = hook::I686_MSVC_Thiscall;

pub struct Hooks {
    pub fslateapplication: FSlateApplication,
    pub _amycharacter_forceduncreouch: &'static RawHook<RefunctIsaAbi, ()>,
    pub _tick: &'static RawHook<RefunctIsaAbi, ()>,
    pub aliftbase: ALiftBase,
    pub _amyhud_drawhud: &'static TypedHook<RefunctIsaAbi, fn(*mut AMyHud), ()>,
    pub _ahud_drawmaterialsimple: &'static TypedHook<RefunctIsaAbi, fn(*mut AHudUE, *mut UMaterialInterfaceUE, f32, f32, f32, f32, f32, bool), ()>,
    pub _ugameusersettings_applyresolutionsettings: &'static RawHook<RefunctIsaAbi, ()>,
    pub _uuserwidget_addtoscreen: &'static RawHook<RefunctIsaAbi, ()>,
    pub _amycharacter_tick: &'static RawHook<RefunctIsaAbi, ()>,
}

pub fn init() -> Hooks {
    #[cfg(windows)] windows::init();
    #[cfg(unix)] linux::init();
    uworld::init();
    map_editor::init();
    font::init();


    unsafe {
        Hooks {
            fslateapplication: FSlateApplication::hook(),
            _amycharacter_forceduncreouch: RawHook::create(AMYCHARACTER_FORCEDUNCROUCH.load(Ordering::Relaxed), newgame::new_game_hook).enabled(),
            _tick: RawHook::create(UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE.load(Ordering::Relaxed), tick::tick_hook).enabled(),
            aliftbase: ALiftBase::hook(),
            _amyhud_drawhud: TypedHook::create(AMYHUD_DRAWHUD.load(Ordering::Relaxed), hud::draw_hud_hook).enabled(),
            _ahud_drawmaterialsimple: TypedHook::create(AHUD_DRAWMATERIALSIMPLE.load(Ordering::Relaxed), hud::draw_material_simple_hook).enabled(),
            _ugameusersettings_applyresolutionsettings: RawHook::create(UGAMEUSERSETTINGS_APPLYRESOLUTIONSETTINGS.load(Ordering::Relaxed), gameusersettings::apply_resolution_settings).enabled(),
            _uuserwidget_addtoscreen: RawHook::create(UUSERWIDGET_ADDTOSCREEN.load(Ordering::Relaxed), uworld::add_to_screen_hook).enabled(),
            _amycharacter_tick: RawHook::create(AMYCHARACTER_TICK.load(Ordering::Relaxed), character::tick_hook).enabled(),
        }
    }
}

pub enum ALiftBaseUE {}
pub struct ALiftBase {
    addbasedcharacter: &'static TypedHook<RefunctIsaAbi, fn(*mut ALiftBaseUE, *mut AMyCharacterUE), ()>,
    removebasedcharacter: &'static TypedHook<RefunctIsaAbi, fn(*mut ALiftBaseUE, *mut AMyCharacterUE), ()>,
}
impl ALiftBase {
    fn hook() -> Self {
        unsafe {
            Self {
                addbasedcharacter: TypedHook::create(ALIFTBASE_ADDBASEDCHARACTER.load(Ordering::Relaxed), tick::add_based_character_hook).enabled(),
                removebasedcharacter: TypedHook::create(ALIFTBASE_REMOVEBASEDCHARACTER.load(Ordering::Relaxed), tick::remove_based_character_hook).enabled(),
            }
        }
    }
    pub unsafe fn add_based_character(&self, this: *mut ALiftBaseUE, character: *mut AMyCharacterUE) {
        self.addbasedcharacter.call_original_function((this, character));
    }
    pub unsafe fn remove_based_character(&self, this: *mut ALiftBaseUE, character: *mut AMyCharacterUE) {
        self.removebasedcharacter.call_original_function((this, character));
    }
}
