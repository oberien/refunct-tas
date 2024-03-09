use std::cell::Cell;
use crate::native::{uworld::ENGINE_INDEX, UeScope};

pub enum UEngine {}
impl UEngine {
    pub fn set_gamma(gamma: f32) {
        UeScope::with(|scope| {
            let object = scope.get(ENGINE_INDEX.get().unwrap());
            object.get_field("DisplayGamma").unwrap::<&Cell<f32>>().set(gamma);
        });
    }
}
