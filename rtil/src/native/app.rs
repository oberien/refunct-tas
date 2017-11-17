pub struct FApp;

impl FApp {
    pub fn get_delta(d: f64) {
        unsafe { *FApp::delta_ptr() }
    }

    pub fn set_delta(d: f64) {
        unsafe { *FApp::delta_ptr() = d };
    }

}