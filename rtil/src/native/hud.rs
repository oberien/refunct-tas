use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use bit_field::BitField;
use atomic_float::AtomicF32;

#[cfg(unix)] use libc::c_void;
use hook::{IsaAbi, TypedHook};
#[cfg(windows)] use winapi::ctypes::c_void;

use crate::native::ue::{FLinearColor, FString, FVector, FVector2D};
use crate::native::{AHUD_DRAWLINE, AHUD_DRAWTEXT, AHUD_DRAWTEXTURESIMPLE, AHUD_DRAWTEXTURE, AHUD_DRAWRECT, AHUD_PROJECT, AHUD_GETTEXTSIZE, REBO_DOESNT_START_SEMAPHORE, UTexture2D, UObject, ObjectWrapper, AMyCharacter};
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

pub enum AHudUE {}
pub enum UMaterialInterfaceUE {}
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
            fun(get_amyhud!("draw_texture_simple"), texture.as_ptr(), x, y, scale, scale_position)
        }
    }
    pub fn draw_texture<C: Into<FLinearColor>>(texture: &UTexture2D, x: f32, y: f32, width: f32, height: f32, u: f32, v: f32, uwidth: f32, vheight: f32, tint_color: C, blend_mode: EBlendMode, scale: f32, scale_position: bool, rotation: f32, rot_pivot_x: f32, rot_pivot_y: f32) {
        unsafe {
            let fun: extern_fn!(fn(
                this: *mut AMyHud, texture: *mut UTexture2DUE,
                screen_x: f32, screen_y: f32, screen_w: f32, screen_h: f32,
                texture_u: f32, texture_v: f32, texture_uwidth: f32, texture_vheight: f32,
                tint_color: FLinearColor, blend_mode: EBlendMode, scale: f32, scale_position: bool,
                rotation: f32, rot_pivot: FVector2D
            )) = mem::transmute(AHUD_DRAWTEXTURE.load(Ordering::SeqCst));
            fun(get_amyhud!("draw_texture"), texture.as_ptr(), x, y, width, height, u, v, uwidth, vheight,
                tint_color.into(), blend_mode, scale, scale_position, rotation, FVector2D { x: rot_pivot_x, y: rot_pivot_y })
        }
    }

    pub fn draw_rect(color: FLinearColor, x: f32, y: f32, width: f32, height: f32) {
        unsafe {
            let fun: extern_fn!(fn(this: *mut AMyHud, color: FLinearColor, screen_x: f32, screen_y: f32,
                screen_w: f32, screen_h: f32))
                = mem::transmute(AHUD_DRAWRECT.load(Ordering::SeqCst));
            fun(get_amyhud!("draw_rect"), color, x, y, width, height);
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

    pub fn set_reticle_width(width: f32) {
        RETICLE_W.store(width, Ordering::Relaxed);
    }

    pub fn set_reticle_height(height: f32) {
        RETICLE_H.store(height, Ordering::Relaxed);
    }
}

pub fn draw_hud_hook<IA: IsaAbi>(hook: &TypedHook<IA, fn(*mut AMyHud), ()>, this: *mut AMyHud) {
    if AMYHUD.load(Ordering::SeqCst).is_null() {
        AMYHUD.store(this, Ordering::SeqCst);
        log!("Got AMyHUD: {:p}", this);
        log!("Got AMyHUD::bitfield: {:p}", unsafe { &(*this).bitfield });
        REBO_DOESNT_START_SEMAPHORE.release();
    }
    ue::draw_hud();
    unsafe { hook.call_original_function(this); }
}

static RETICLE_W: AtomicF32 = AtomicF32::new(6.);
static RETICLE_H: AtomicF32 = AtomicF32::new(6.);

pub fn draw_material_simple_hook<IA: IsaAbi>(
    hook: &TypedHook<IA, fn(*mut AHudUE, *mut UMaterialInterfaceUE, f32, f32, f32, f32, f32, bool), ()>,
    this: *mut AHudUE, material: *mut UMaterialInterfaceUE, mut screen_x: f32, mut screen_y: f32,
    mut screen_w: f32, mut screen_h: f32, scale: f32, scale_position: bool,
) {
    let obj = unsafe { ObjectWrapper::new(material as *mut UObject) };
    if obj.name() == "M_Player_Crosshair" {
        screen_w = RETICLE_W.load(Ordering::Relaxed);
        screen_h = RETICLE_H.load(Ordering::Relaxed);
        let sw = screen_w * scale;
        let sh = screen_h * scale;
        screen_x = (AMyCharacter::get_player().get_viewport_size().0 as f32 / 2.) - (sw / 2.);
        screen_y = (AMyCharacter::get_player().get_viewport_size().1 as f32 / 2.) - (sh / 2.);
    }
    unsafe { hook.call_original_function((this, material, screen_x, screen_y, screen_w, screen_h, scale, scale_position)); }
}
#[allow(unused)]
#[repr(i32)]
pub enum EBlendMode {
    Opaque,
    Masked,
    Translucent,
    Additive,
    Modulate,
    AlphaComposite,
}
