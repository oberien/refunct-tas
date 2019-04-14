use std::mem;
use std::ptr;

#[cfg(unix)] use libc::c_void;
#[cfg(windows)] use winapi::ctypes::c_void;

use native::ue::{FLinearColor, FString, FVector};
use native::{AHUD_DRAWLINE, AHUD_DRAWTEXT, AHUD_PROJECT};
use threads::ue;
use crate::statics::Static;

lazy_static! {
    static ref AMYHUD: Static<usize> = Static::new();
}

pub struct AMyHud;

impl AMyHud {
    pub fn draw_line<C: Into<FLinearColor>>(startx: f32, starty: f32, endx: f32, endy: f32, color: C, thickness: f32) {
        let fun: extern_fn!(fn(
            this: usize, start_screen_x: f32, start_screen_y: f32, end_screen_x: f32,
            end_screen_y: f32, line_color: FLinearColor, thickness: f32
        )) = unsafe { mem::transmute(AHUD_DRAWLINE) };
        fun(*AMYHUD.get(), startx, starty, endx, endy, color.into(), thickness)
    }

    pub fn draw_text<S: Into<FString>, C:Into<FLinearColor>>(text: S, color: C, x: f32, y: f32, scale: f32, scale_position: bool) {
        unsafe {
            let fun: extern_fn!(fn(
                this: usize, text: *const FString, text_color: FLinearColor, screen_x: f32,
                screen_y: f32, font: *const c_void, scale: f32, scale_position: bool))
                = mem::transmute(AHUD_DRAWTEXT);
            let s = text.into();
            fun(*AMYHUD.get(), &s as *const FString, color.into(), x, y, ptr::null(), scale, scale_position)
        }
    }

    pub fn project(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        unsafe {
            let fun: extern_fn!(fn(this: usize, location: FVector) -> FVector)
                = mem::transmute(AHUD_PROJECT);
            let vec = fun(*AMYHUD.get(), FVector { x, y, z });
            (vec.x, vec.y, vec.z)
        }
    }
}

#[rtil_derive::hook_before(AMyHUD::DrawHUD)]
fn draw_hud(this: usize) {
    if AMYHUD.is_none() {
        AMYHUD.set(this);
        log!("Got AMyHUD: {:#x}", this);
    }
    ue::draw_hud();
}
