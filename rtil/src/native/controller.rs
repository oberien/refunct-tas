use native::ACONTROLLER_GETCONTROLROTATION;
#[cfg(unix)] use native::linux::controller::save;
#[cfg(windows)] use native::windows::controller::save;

lazy_static! {
    static ref CONTROLLER: Static<usize> = Static::new();
}

pub struct AController;

impl AController {
    unsafe fn pitch_ptr() -> *mut f32 {
        #[cfg(unix)] { (&*CONTROLLER.get() + 0x3b8) as *mut f32 }
        #[cfg(windows)] { (&*CONTROLLER.get() + 0x2d0) as *mut f32 }
    }
    unsafe fn yaw_ptr() -> *mut f32 {
        #[cfg(unix)] { (&*CONTROLLER.get() + 0x3bc) as *mut f32 }
        #[cfg(windows)] { (&*CONTROLLER.get() + 0x2d4) as *mut f32 }
    }
    unsafe fn roll_ptr() -> *mut f32 {
        #[cfg(unix)] { (&*CONTROLLER.get() + 0x3c0) as *mut f32 }
        #[cfg(windows)] { (&*CONTROLLER.get() + 0x2d8) as *mut f32 }
    }

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

#[rtil_derive::hook_once(AController::GetControlRotation)]
fn save(this: usize) {
    CONTROLLER.set(this);
    log!("Got AController: {:#x}", this);
}
