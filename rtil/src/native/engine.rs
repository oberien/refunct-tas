use std::cell::Cell;
use std::ffi::c_void;
use std::ptr;
use std::sync::atomic::Ordering;
use crate::native::{uworld::ENGINE_INDEX, AMyCharacter, BoolValueWrapper, ObjectWrapper, UObject, UeScope, FVIEWPORT_SETGAMERENDERINGENABLED, UWIDGETBLUEPRINTLIBRARY_SETINPUTMODE_GAMEONLY, UWIDGETBLUEPRINTLIBRARY_SETINPUTMODE_UIONLYEX};
use crate::native::character::APlayerController;

pub enum UEngine {}
pub enum FViewport {}
pub enum UWidgetBlueprintLibrary {}
#[allow(unused)]
pub enum EMouseLockMode {
    DoNotLock,
    LockOnCapture,
    LockAlways,
    LockInFullscreen,
}
impl UEngine {
    pub fn set_gamma(gamma: f32) {
        UeScope::with(|scope| {
            let object = scope.get(ENGINE_INDEX.get().unwrap());
            object.get_field("DisplayGamma").unwrap::<&Cell<f32>>().set(gamma);
        });
    }
}
impl FViewport {
    pub fn set_game_rendering_enabled(enable: bool) {
        let fun: extern "C" fn(is_enabled: bool, present_and_stop_movie_delay: i32)
            = unsafe { ::std::mem::transmute(FVIEWPORT_SETGAMERENDERINGENABLED.load(Ordering::SeqCst)) };
        fun(enable, 0);
    }
}

impl UWidgetBlueprintLibrary {
    pub fn set_input_mode_game_only() {
        let fun: extern "C" fn(player_controller: *mut APlayerController)
            = unsafe { ::std::mem::transmute(UWIDGETBLUEPRINTLIBRARY_SETINPUTMODE_GAMEONLY.load(Ordering::SeqCst)) };
        fun(AMyCharacter::get_player().controller());
        let controller = unsafe { ObjectWrapper::new(AMyCharacter::get_player().controller() as *mut UObject) };
        controller.get_field("bShowMouseCursor").unwrap::<BoolValueWrapper>().set(false);
    }
    pub fn set_input_mode_ui_only() {
        let fun: extern "C" fn(player_controller: *mut APlayerController, widget: *const c_void, mouse_lock_mode: EMouseLockMode)
            = unsafe { ::std::mem::transmute(UWIDGETBLUEPRINTLIBRARY_SETINPUTMODE_UIONLYEX.load(Ordering::SeqCst)) };
        fun(AMyCharacter::get_player().controller(), ptr::null(), EMouseLockMode::DoNotLock);
        let controller = unsafe { ObjectWrapper::new(AMyCharacter::get_player().controller() as *mut UObject) };
        controller.get_field("bShowMouseCursor").unwrap::<BoolValueWrapper>().set(true);
    }
}
