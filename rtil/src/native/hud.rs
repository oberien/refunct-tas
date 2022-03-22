use std::mem;
use std::ptr;
use std::sync::atomic::Ordering;

#[cfg(unix)] use libc::c_void;
use once_cell::sync::Lazy;
#[cfg(windows)] use winapi::ctypes::c_void;

use crate::native::ue::{FLinearColor, FString, FVector};
use crate::native::{AHUD_DRAWLINE, AHUD_DRAWTEXT, AHUD_PROJECT, AHUD_GETTEXTSIZE};
use crate::threads::ue;
use crate::statics::Static;

static AMYHUD: Lazy<Static<usize>> = Lazy::new(Static::new);

pub struct AMyHud;

impl AMyHud {
    pub fn draw_line<C: Into<FLinearColor>>(startx: f32, starty: f32, endx: f32, endy: f32, color: C, thickness: f32) {
        let fun: extern_fn!(fn(
            this: usize, start_screen_x: f32, start_screen_y: f32, end_screen_x: f32,
            end_screen_y: f32, line_color: FLinearColor, thickness: f32
        )) = unsafe { mem::transmute(AHUD_DRAWLINE.load(Ordering::SeqCst)) };
        fun(*AMYHUD.get(), startx, starty, endx, endy, color.into(), thickness)
    }

    pub fn draw_text<S: Into<FString>, C:Into<FLinearColor>>(text: S, color: C, x: f32, y: f32, scale: f32, scale_position: bool) {
        unsafe {
            let fun: extern_fn!(fn(
                this: usize, text: *const FString, text_color: FLinearColor, screen_x: f32,
                screen_y: f32, font: *const c_void, scale: f32, scale_position: bool))
                = mem::transmute(AHUD_DRAWTEXT.load(Ordering::SeqCst));
            let s = text.into();
            fun(*AMYHUD.get(), &s as *const FString, color.into(), x, y, ptr::null(), scale, scale_position)
        }
    }

    pub fn project(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        unsafe {
            let fun: extern_fn!(fn(this: usize, location: FVector) -> FVector)
                = mem::transmute(AHUD_PROJECT.load(Ordering::SeqCst));
            let vec = fun(*AMYHUD.get(), FVector { x, y, z });
            (vec.x, vec.y, vec.z)
        }
    }

    pub fn get_text_size<S: Into<FString>>(text: S, scale: f32) -> (f32, f32) {
        unsafe {
            let fun: extern_fn!(fn(
                this: usize, text: *const FString, out_width: &mut f32, out_height: &mut f32,
                font: *const c_void, scale: f32
            )) = mem::transmute(AHUD_GETTEXTSIZE.load(Ordering::SeqCst));
            let mut width = 0.;
            let mut height = 0.;
            let s = text.into();
            fun(*AMYHUD.get(), &s as *const FString, &mut width, &mut height, ptr::null(), scale);
            (width, height)
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
