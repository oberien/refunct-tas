use crate::native::{UWorld, LevelState, ObjectWrapper, UObject};

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
        let restart_game = obj.class().find_function("RestartGame").unwrap();
        let params = restart_game.create_argument_struct();
        params.get_field("Reset").unwrap_bool().set(false);
        unsafe { restart_game.call(UMyGameInstance::get_umygameinstance(), &params); }
    }
}

