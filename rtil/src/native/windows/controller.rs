use native::controller::{CONTROLLER, AController};

impl AController {
    pub(in native) unsafe fn pitch_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x2d0) as *mut f32
    }
    pub(in native) unsafe fn yaw_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x2d4) as *mut f32
    }
    pub(in native) unsafe fn roll_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x2d8) as *mut f32
    }
}

#[inline(never)]
pub(in native) extern "thiscall" fn save(this: usize) {
    CONTROLLER.set(this);
    log!("Got AController: {:#x}", this);
}
