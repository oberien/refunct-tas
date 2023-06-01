use std::sync::atomic::Ordering;
use crate::native::ue::UeF64;
use super::FAPP_DELTATIME;

pub struct FApp;

impl FApp {
    unsafe fn delta_ptr() -> *mut UeF64 {
        FAPP_DELTATIME.load(Ordering::SeqCst) as *mut UeF64
    }

    pub fn delta() -> f64 {
        unsafe { (*FApp::delta_ptr()).get() }
    }

    pub fn set_delta(d: f64) {
        unsafe { (*FApp::delta_ptr()).set(d) };
    }
}