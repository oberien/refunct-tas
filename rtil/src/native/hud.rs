use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use bit_field::BitField;

#[cfg(unix)] use libc::c_void;
#[cfg(windows)] use winapi::ctypes::c_void;

use crate::native::ue::{FLinearColor, FString, FVector};
use crate::native::{AHUD_DRAWLINE, AHUD_DRAWTEXT, AHUD_DRAWTEXTURESIMPLE, AHUD_PROJECT, AHUD_GETTEXTSIZE, Args, REBO_DOESNT_START_SEMAPHORE, UTexture2D};
use crate::native::texture::UTexture2DUE;
use crate::threads::ue;

static AMYHUD: AtomicPtr<AMyHud> = AtomicPtr::new(ptr::null_mut());

macro_rules! get_amyhud {
    ($fnname:literal) => {{
        let amyhud = AMYHUD.load(Ordering::SeqCst);
        if amyhud.is_null() {
            let msg = concat!("called AMyHud::", $fnname, " while AMyHud-pointer wasn't initialized yet");
            log!("{}", msg);
            panic!("{}", msg);
        }
        amyhud
    }}
}

#[repr(C)]
pub struct AMyHud {
    #[cfg(windows)] _pad: [u8; 0x2b8],
    #[cfg(unix)] _pad: [u8; 0x390],
    // bLostFocusPaused, bShowHUD, bShowDebugInfo, bShowHitBoxDebugInfo, bShowOverlays, bEnableDebugTextShadow
    pub bitfield: u8,
}

impl AMyHud {
    pub fn draw_line<C: Into<FLinearColor>>(startx: f32, starty: f32, endx: f32, endy: f32, color: C, thickness: f32) {
        let fun: extern_fn!(fn(
            this: *mut AMyHud, start_screen_x: f32, start_screen_y: f32, end_screen_x: f32,
            end_screen_y: f32, line_color: FLinearColor, thickness: f32
        )) = unsafe { mem::transmute(AHUD_DRAWLINE.load(Ordering::SeqCst)) };
        fun(get_amyhud!("draw_line"), startx, starty, endx, endy, color.into(), thickness)
    }

    pub fn draw_text<S: Into<FString>, C:Into<FLinearColor>>(text: S, color: C, x: f32, y: f32, scale: f32, scale_position: bool) {
        unsafe {
            let fun: extern_fn!(fn(
                this: *mut AMyHud, text: *const FString, text_color: FLinearColor, screen_x: f32,
                screen_y: f32, font: *const c_void, scale: f32, scale_position: bool))
                = mem::transmute(AHUD_DRAWTEXT.load(Ordering::SeqCst));
            let s = text.into();
            fun(get_amyhud!("draw_text"), &s as *const FString, color.into(), x, y, ptr::null(), scale, scale_position)
        }
    }

    pub fn draw_texture_simple(texture: &UTexture2D, x: f32, y: f32, scale: f32, scale_position: bool) {
        unsafe {
            let fun: extern_fn!(fn(
                this: *mut AMyHud, texture: *mut UTexture2DUE, screen_x: f32,
                screen_y: f32, scale: f32, scale_position: bool))
                = mem::transmute(AHUD_DRAWTEXTURESIMPLE.load(Ordering::SeqCst));
            fun(get_amyhud!("draw_text"), texture.as_ptr(), x, y, scale, scale_position)
        }
    }

    pub fn project(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        unsafe {
            let fun: extern_fn!(fn(this: *mut AMyHud, location: FVector) -> FVector)
                = mem::transmute(AHUD_PROJECT.load(Ordering::SeqCst));
            let vec = fun(get_amyhud!("project"), FVector { x, y, z });
            (vec.x, vec.y, vec.z)
        }
    }

    pub fn get_text_size<S: Into<FString>>(text: S, scale: f32) -> (f32, f32) {
        unsafe {
            let fun: extern_fn!(fn(
                this: *mut AMyHud, text: *const FString, out_width: &mut f32, out_height: &mut f32,
                font: *const c_void, scale: f32
            )) = mem::transmute(AHUD_GETTEXTSIZE.load(Ordering::SeqCst));
            let mut width = 0.;
            let mut height = 0.;
            let s = text.into();
            fun(get_amyhud!("get_text_size"), &s as *const FString, &mut width, &mut height, ptr::null(), scale);
            (width, height)
        }
    }

    pub fn show_hud() {
        unsafe { (*get_amyhud!("show_hud")).bitfield.set_bit(1, true); }
    }
}

#[rtil_derive::hook_before(AMyHUD::DrawHUD)]
fn draw_hud(args: &mut Args) {
    let this = unsafe { args.nth_integer_arg(0) } as *mut AMyHud;
    if AMYHUD.load(Ordering::SeqCst).is_null() {
        AMYHUD.store(this, Ordering::SeqCst);
        log!("Got AMyHUD: {:p}", this);
        log!("Got AMyHUD::bitfield: {:p}", unsafe { &(*this).bitfield });
        REBO_DOESNT_START_SEMAPHORE.release();
    }
    ue::draw_hud();
}
