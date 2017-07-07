use super::ACONTROLLER_GETCONTROLROTATION;
use native::CONTROLLER;

pub struct AController;

impl AController {
    pub fn rotation() -> (f32, f32, f32) {
        let pitch = unsafe { *AController::pitch_ptr() };
        let yaw = unsafe { *AController::yaw_ptr() };
        let roll = unsafe { *AController::roll_ptr() };
        (pitch, yaw, roll)
    }

    pub fn set_rotation(pitch: f32, yaw: f32, roll: f32) {
        unsafe {
            *AController::pitch_ptr() = pitch;
            *AController::yaw_ptr() = yaw;
            *AController::roll_ptr() = roll;
        }
    }

    unsafe fn pitch_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x3b8) as *mut f32
    }
    unsafe fn yaw_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x3bc) as *mut f32
    }
    unsafe fn roll_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x3c0) as *mut f32
    }
}

hook_beginning! {
    hook_controller,
    restore_controller,
    get_controller,
    "AController::GetControlRotation",
    ACONTROLLER_GETCONTROLROTATION,
}

hook_fn_once! {
    get_controller,
    save_controller,
    restore_controller,
    ACONTROLLER_GETCONTROLROTATION,
}

#[inline(never)]
extern fn save_controller(this: usize) {
    CONTROLLER.set(this);
    log!("Got AController: {:#x}", this);
}
