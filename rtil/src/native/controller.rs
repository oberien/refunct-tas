use native::ACONTROLLER_GETCONTROLROTATION;
#[cfg(unix)] use native::linux::controller::save;
#[cfg(windows)] use native::windows::controller::save;

lazy_static! {
    pub(in native) static ref CONTROLLER: Static<usize> = Static::new();
}

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
}

hook! {
    "AController::GetControlRotation",
    ACONTROLLER_GETCONTROLROTATION,
    hook,
    unhook,
    get,
    true,
}

hook_fn_once! {
    get,
    save,
    unhook,
    ACONTROLLER_GETCONTROLROTATION,
}
