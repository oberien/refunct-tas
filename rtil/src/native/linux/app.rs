use native::FApp;
use native::linux::FAPP_DELTATIME;

impl FApp {
    pub(in native) unsafe fn delta_ptr() -> *mut f64 {
        FAPP_DELTATIME as *mut f64
    }
}