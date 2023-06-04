use std::sync::atomic::Ordering;
use crate::native::FPLATFORMMISC_PUMPMESSAGES;

pub enum FPlatformMisc {}

impl FPlatformMisc {
    pub fn pump_messages() {
        unsafe {
            let fun: extern "C" fn(from_main_loop: bool)
                = std::mem::transmute(FPLATFORMMISC_PUMPMESSAGES.load(Ordering::SeqCst));
            fun(true);
        }
    }
}