use std::sync::atomic::Ordering;
use super::FAPP_DELTATIME;

pub struct FApp;

impl FApp {
    unsafe fn delta_ptr() -> *mut f64 {
        FAPP_DELTATIME.load(Ordering::SeqCst) as *mut f64
    }

    pub fn delta() -> f64 {
        unsafe { *FApp::delta_ptr() }
    }

    pub fn set_delta(d: f64) {
        unsafe { *FApp::delta_ptr() = d };
    }
}