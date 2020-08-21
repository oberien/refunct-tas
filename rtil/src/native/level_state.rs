lazy_static! {
    static ref STATE_PTR: *mut LevelState = level_state_pointer();
}

use super::consts::LEVEL_POINTER_PATH;

#[repr(C)]
pub struct LevelState {

}

fn level_state_pointer() -> *mut LevelState {
    let base = super::base_address();

}

pub fn level_state() -> LevelState {
    unsafe {
        **STATE_PTR
    }
}