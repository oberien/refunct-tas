lazy_static! {
    static ref LEVEL_STATE_ADDRESS: usize = level_state_address();
}

use super::consts::LEVEL_POINTER_PATH;
#[cfg(unix)] use libc::c_void;
#[cfg(windows)] use winapi::ctypes::c_void;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct LevelState {
    pub cubes: i32,
    unknown: i32,
    unknown2: i32,
    pub level: i32,
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
        let addr: usize = *LEVEL_STATE_ADDRESS;
        let ptr = addr as *mut LevelState;
        ptr
    }

    pub fn set_level(level: i32) {
        unsafe {
            (*LevelState::get_ptr()).level = level;
        }
    }
}

fn level_state_address() -> usize {
    // pointer chasing from base with offsets
    let base = super::base_address();
    let level_offset = memoffset::offset_of!(LevelState, level);
    let mut level_addr = base;
    log!("level-path: {:#x}", level_addr);
    let (offsets, last) = LEVEL_POINTER_PATH.split_at(LEVEL_POINTER_PATH.len() - 1);
    for offset in offsets {
        let addr = level_addr + offset;
        log!("dereferencing {:#x}", addr);
        level_addr = unsafe { *(addr as *const usize) };
        log!("level-path: {:#x}", level_addr);
    }
    let state_addr = level_addr + last[0] - level_offset;
    log!("level_state-addr: {:#x}", state_addr);
    state_addr
}

