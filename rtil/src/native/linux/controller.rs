use native::controller::{CONTROLLER, AController};

impl AController {
    pub(in native) unsafe fn pitch_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x3b8) as *mut f32
    }
    pub(in native) unsafe fn yaw_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x3bc) as *mut f32
    }
    pub(in native) unsafe fn roll_ptr() -> *mut f32 {
        (&*CONTROLLER.get() + 0x3c0) as *mut f32
    }
}

#[inline(never)]
pub(in native) extern fn save(this: usize) {
    CONTROLLER.set(this);
    log!("Got AController: {:#x}", this);
}
