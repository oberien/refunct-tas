use std::sync::atomic::Ordering;
use crate::native::FPLATFORMMISC_PUMPMESSAGES;

pub enum FPlatformMisc {}

impl FPlatformMisc {
    pub fn pump_messages() {
        unsafe {
            let fun: extern_fn!(fn(from_main_loop: i32))
                = std::mem::transmute(FPLATFORMMISC_PUMPMESSAGES.load(Ordering::SeqCst));
            fun(1);
        }
    }
}