use std::mem;
use std::ptr;

use native::ue::{FLinearColor, FString};
use native::{AMYHUD_DRAWHUD, AHUD_DRAWLINE, AHUD_DRAWTEXT};
use threads::ue;

lazy_static! {
    static ref AMYHUD: Static<usize> = Static::new();
}

pub struct AMyHud;

impl AMyHud {
    pub fn draw_line<C: Into<FLinearColor>>(startx: f32, starty: f32, endx: f32, endy: f32, color: C, thickness: f32) {
        let fun: extern_fn! { fn(usize, f32, f32, f32, f32, FLinearColor, f32) }
            = unsafe { mem::transmute(AHUD_DRAWLINE) };
        fun(*AMYHUD.get(), startx, starty, endx, endy, color.into(), thickness)
    }

    pub fn draw_text<S: Into<FString>, C:Into<FLinearColor>>(text: S, color: C, x: f32, y: f32, scale: f32, scale_position: bool) {
        unsafe {
            let fun: extern_fn! { fn(usize, *const FString, FLinearColor, f32, f32, * const (), f32, bool) }
                = mem::transmute(AHUD_DRAWTEXT);
            let s = text.into();
            fun(*AMYHUD.get(), &s as *const FString, color.into(), x, y, ptr::null(), scale, scale_position)
        }
    }
}

hook! {
    "AMyHUD::DrawHUD",
    AMYHUD_DRAWHUD,
    hook,
    unhook,
    get,
    false,
}

hook_fn_always! {
    get,
    draw_hud,
    hook,
    unhook,
    AMYHUD_DRAWHUD,
    intercept before original,
}

extern_fn! {
    fn draw_hud(this: usize) {
        if AMYHUD.is_none() {
            AMYHUD.set(this);
            log!("Got AMyHUD: {:#x}", this);
        }
        ue::draw_hud();
    }
}
