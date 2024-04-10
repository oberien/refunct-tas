use std::cell::Cell;
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

#[cfg(unix)] use libc::c_void;
#[cfg(windows)] use winapi::ctypes::c_void;

use crate::native::ue::{FLinearColor, FString};
use crate::native::{AHUD_DRAWTEXT, AHUD_GETTEXTSIZE, Args, REBO_DOESNT_START_SEMAPHORE, UTexture2D, ObjectWrapper, UObject, BoolValueWrapper, StructValueWrapper};
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

pub struct AMyHud {}

impl AMyHud {
    pub fn draw_line<C: Into<FLinearColor> + Clone + Copy>(startx: f32, starty: f32, endx: f32, endy: f32, color: C, thickness: f32) {
        let hud = unsafe { ObjectWrapper::new(get_amyhud!("draw_line") as *mut UObject) };
        let fun = hud.class().find_function("DrawLine").unwrap();
        let params = fun.create_argument_struct();
        params.get_field("StartScreenX").unwrap::<&Cell<f32>>().set(startx);
        params.get_field("StartScreenY").unwrap::<&Cell<f32>>().set(starty);
        params.get_field("EndScreenX").unwrap::<&Cell<f32>>().set(endx);
        params.get_field("EndScreenY").unwrap::<&Cell<f32>>().set(endy);
        params.get_field("LineColor").field("R").unwrap::<&Cell<f32>>().set(color.into().red);
        params.get_field("LineColor").field("G").unwrap::<&Cell<f32>>().set(color.into().green);
        params.get_field("LineColor").field("B").unwrap::<&Cell<f32>>().set(color.into().blue);
        params.get_field("LineColor").field("A").unwrap::<&Cell<f32>>().set(color.into().alpha);
        params.get_field("LineThickness").unwrap::<&Cell<f32>>().set(thickness);
        unsafe {
            fun.call(hud.as_ptr(), &params);
        }
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
        let hud = unsafe { ObjectWrapper::new(get_amyhud!("draw_texture_simple") as *mut UObject) };
        let fun = hud.class().find_function("DrawTextureSimple").unwrap();
        let params = fun.create_argument_struct();
        let texture = unsafe { ObjectWrapper::new(texture.as_ptr() as *mut UObject) };
        params.get_field("Texture").set_object(&texture);
        params.get_field("ScreenX").unwrap::<&Cell<f32>>().set(x);
        params.get_field("ScreenY").unwrap::<&Cell<f32>>().set(y);
        params.get_field("Scale").unwrap::<&Cell<f32>>().set(scale);
        params.get_field("bScalePosition").unwrap::<BoolValueWrapper>().set(scale_position);
        unsafe {
            fun.call(hud.as_ptr(), &params);
        }
    }
    pub fn draw_texture<C: Into<FLinearColor> + Clone + Copy>(texture: &UTexture2D, x: f32, y: f32, width: f32, height: f32, u: f32, v: f32, uwidth: f32, vheight: f32, tint_color: C, blend_mode: u8, scale: f32, scale_position: bool, rotation: f32, rot_pivot_x: f32, rot_pivot_y: f32) {
        let hud = unsafe { ObjectWrapper::new(get_amyhud!("draw_texture") as *mut UObject) };
        let fun = hud.class().find_function("DrawTexture").unwrap();
        let params = fun.create_argument_struct();
        let texture = unsafe { ObjectWrapper::new(texture.as_ptr() as *mut UObject) };
        params.get_field("Texture").set_object(&texture);
        params.get_field("ScreenX").unwrap::<&Cell<f32>>().set(x);
        params.get_field("ScreenY").unwrap::<&Cell<f32>>().set(y);
        params.get_field("ScreenW").unwrap::<&Cell<f32>>().set(width);
        params.get_field("ScreenH").unwrap::<&Cell<f32>>().set(height);
        params.get_field("TextureU").unwrap::<&Cell<f32>>().set(u);
        params.get_field("TextureV").unwrap::<&Cell<f32>>().set(v);
        params.get_field("TextureUWidth").unwrap::<&Cell<f32>>().set(uwidth);
        params.get_field("TextureVHeight").unwrap::<&Cell<f32>>().set(vheight);
        params.get_field("TintColor").field("R").unwrap::<&Cell<f32>>().set(tint_color.into().red);
        params.get_field("TintColor").field("G").unwrap::<&Cell<f32>>().set(tint_color.into().green);
        params.get_field("TintColor").field("B").unwrap::<&Cell<f32>>().set(tint_color.into().blue);
        params.get_field("TintColor").field("A").unwrap::<&Cell<f32>>().set(tint_color.into().alpha);
        params.get_field("BlendMode").unwrap::<&Cell<u8>>().set(blend_mode);
        params.get_field("Scale").unwrap::<&Cell<f32>>().set(scale);
        params.get_field("bScalePosition").unwrap::<BoolValueWrapper>().set(scale_position);
        params.get_field("Rotation").unwrap::<&Cell<f32>>().set(rotation);
        params.get_field("RotPivot").field("X").unwrap::<&Cell<f32>>().set(rot_pivot_x);
        params.get_field("RotPivot").field("Y").unwrap::<&Cell<f32>>().set(rot_pivot_y);
        unsafe {
            fun.call(hud.as_ptr(), &params);
        }
    }

    pub fn project(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        let hud = unsafe { ObjectWrapper::new(get_amyhud!("project") as *mut UObject) };
        let fun = hud.class().find_function("Project").unwrap();
        let params = fun.create_argument_struct();
        let loc = params.get_field("Location").unwrap::<StructValueWrapper>();
        loc.get_field("X").unwrap::<&Cell<f32>>().set(x);
        loc.get_field("Y").unwrap::<&Cell<f32>>().set(y);
        loc.get_field("Z").unwrap::<&Cell<f32>>().set(z);
        unsafe {
            fun.call(hud.as_ptr(), &params);
        }
        let new_loc = params.get_field("ReturnValue").unwrap::<StructValueWrapper>();
        (new_loc.get_field("X").unwrap::<f32>(),
         new_loc.get_field("Y").unwrap::<f32>(),
         new_loc.get_field("Z").unwrap::<f32>())
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
        let obj = unsafe { ObjectWrapper::new(*&get_amyhud!("show_hud") as *mut UObject) };
        obj.get_field("bShowHUD").unwrap::<BoolValueWrapper>().set(true);
    }
}

#[rtil_derive::hook_before(AMyHUD::DrawHUD)]
fn draw_hud(args: &mut Args) {
    let this = unsafe { args.nth_integer_arg(0) } as *mut AMyHud;
    if AMYHUD.load(Ordering::SeqCst).is_null() {
        AMYHUD.store(this, Ordering::SeqCst);
        log!("Got AMyHUD: {:p}", this);
        REBO_DOESNT_START_SEMAPHORE.release();
    }
    ue::draw_hud();
}
