use once_cell::sync::Lazy;
use super::gameinstance::UMyGameInstance;

static LEVEL_STATE_ADDRESS: Lazy<usize> = Lazy::new(|| {
    let levelstate = UMyGameInstance::get_levelstate();
    log!("level_state-addr: {:#x}", levelstate as usize);
    unsafe {
        log!("Got state {:?}", *levelstate);
    }
    levelstate as usize
});

#[repr(C)]
#[derive(Debug, Clone, PartialEq, rebo::ExternalType)]
pub struct LevelState {
    pub level: i32,
    pub platforms: i32,
    pub cubes: i32,
    pub buttons: i32,
    pub resets: i32,
    pub start_seconds: i32,
    pub start_partial_seconds: f32,
    pub end_seconds: i32,
    pub end_partial_seconds: f32,
}

impl LevelState {
    pub fn get() -> LevelState {
        unsafe {
            (*LevelState::get_ptr()).clone()
        }
    }

    unsafe fn get_ptr() -> *mut LevelState {
        Self::get_ptr_raw(*LEVEL_STATE_ADDRESS)
    }

    unsafe fn get_ptr_raw(addr: usize) -> *mut LevelState {
        addr as *mut LevelState
    }

    pub fn get_level() -> i32 {
        unsafe {
            (*LevelState::get_ptr()).level
        }
    }
    pub fn set_level(level: i32) {
        unsafe {
            (*LevelState::get_ptr()).level = level;
        }
    }

    pub fn get_start_seconds() -> i32 {
        unsafe {
            (*LevelState::get_ptr()).start_seconds
        }
    }

    pub fn set_start_seconds(start_seconds: i32) {
        unsafe {
            (*LevelState::get_ptr()).start_seconds = start_seconds;
        }
    }

    pub fn get_start_partial_seconds() -> f32 {
        unsafe {
            (*LevelState::get_ptr()).start_partial_seconds
        }
    }

    pub fn set_start_partial_seconds(start_partial_seconds: f32) {
        unsafe {
            (*LevelState::get_ptr()).start_partial_seconds = start_partial_seconds;
        }
    }

    pub fn get_end_seconds() -> i32 {
        unsafe {
            (*LevelState::get_ptr()).end_seconds
        }
    }

    pub fn set_end_seconds(end_seconds: i32) {
        unsafe {
            (*LevelState::get_ptr()).end_seconds = end_seconds;
        }
    }

    pub fn get_end_partial_seconds() -> f32 {
        unsafe {
            (*LevelState::get_ptr()).end_partial_seconds
        }
    }

    pub fn set_end_partial_seconds(end_partial_seconds: f32) {
        unsafe {
            (*LevelState::get_ptr()).end_partial_seconds = end_partial_seconds;
        }
    }

}

// fn level_state_address() -> usize {
//     // pointer chasing from base with offsets
//     let base = super::base_address();
//     let level_offset = memoffset::offset_of!(LevelState, level);
//     let mut level_addr = base;
//     log!("level-path: {:#x}", level_addr);
//     let (offsets, last) = LEVEL_POINTER_PATH.split_at(LEVEL_POINTER_PATH.len() - 1);
//     for offset in offsets {
//         let addr = level_addr + offset;
//         log!("dereferencing {:#x}", addr);
//         level_addr = unsafe { *(addr as *const usize) };
//         log!("level-path: {:#x}", level_addr);
//     }
//     let state_addr = level_addr + last[0] - level_offset;
//     log!("level_state-addr: {:#x}", state_addr);
//     let state = unsafe { (*LevelState::get_ptr_raw(state_addr)).clone() };
//     log!("Got state {:?}", state);
//     state_addr
// }
