use std::cell::Cell;
use std::sync::atomic::Ordering;
use crate::native::{uworld::ENGINE_INDEX, UeScope, FVIEWPORT_SETGAMERENDERINGENABLED};

pub enum UEngine {}
pub enum FViewport {}
impl UEngine {
    pub fn set_gamma(gamma: f32) {
        UeScope::with(|scope| {
            let object = scope.get(ENGINE_INDEX.get().unwrap());
            object.get_field("DisplayGamma").unwrap::<&Cell<f32>>().set(gamma);
        });
    }
}
impl FViewport {
    pub fn set_game_rendering_enabled(enable: bool) {
        let fun: extern "C" fn(is_enabled: bool, present_and_stop_movie_delay: i32)
            = unsafe { ::std::mem::transmute(FVIEWPORT_SETGAMERENDERINGENABLED.load(Ordering::SeqCst)) };
        fun(enable, 0);
    }
}
