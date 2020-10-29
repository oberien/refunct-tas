use native::{UWorld, LevelState};

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
}

