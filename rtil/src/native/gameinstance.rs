use std::sync::atomic::Ordering;
use std::ffi::c_void;
use crate::native::{UWorld, LevelState, ObjectWrapper, UObject};
use crate::native::AACTOR_PROCESSEVENT;

#[repr(C)]
pub struct UMyGameInstance {
    #[cfg(windows)]
    pad: [u8; 0x9c],
    #[cfg(unix)]
    pad: [u8; 0x134],
    level_state: LevelState,
}
impl UMyGameInstance {
    pub fn get_umygameinstance() -> *mut UMyGameInstance {
        UWorld::get_umygameinstance()
    }
    pub fn get_levelstate() -> *mut LevelState {
        unsafe {
            &mut (*Self::get_umygameinstance()).level_state as *mut LevelState
        }
    }
    pub fn restart_game() {
        let obj = unsafe { ObjectWrapper::new(UMyGameInstance::get_umygameinstance() as *mut UObject) };
        let offset = obj.class().as_struct().iter_properties()
            .find(|p| p.name() == "RestartGame")
            .unwrap().as_ptr();
        let ufunction = offset as *mut c_void;
        #[repr(C)]
        struct RestartGameParams { reset: bool }
        let restart_game_params = RestartGameParams { reset: false };
        let fun: extern_fn!(fn(this: *mut UMyGameInstance, function: *const c_void, args: *const c_void)) =
            unsafe { ::std::mem::transmute(AACTOR_PROCESSEVENT.load(Ordering::SeqCst)) };
        fun(UMyGameInstance::get_umygameinstance(), ufunction, &restart_game_params as *const _ as *const c_void);
    }
}

