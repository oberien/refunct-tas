use std::cell::Cell;
use crate::native::reflection::{ObjectWrapper, UObject};
use crate::native::uworld::ENGINE_INDEX;
use crate::native::{ArrayWrapper, StructValueWrapper, UeScope, UFONTBULKDATA_INITIALIZE};
use std::sync::atomic::Ordering;
use std::mem;

pub fn init() {
    UFont::set_font(UFont::get_font("LargeFont"));
}

pub struct UFont {
    font: *mut UFontUE
}

pub enum UFontUE {}

#[repr(C)]
struct UFontBulkData {
    // stub, we only need a pointer to this struct
}

impl UFont {
    pub fn get_font(name: &str) -> UFont {
        UeScope::with(|scope| {
            let engine = scope.get(ENGINE_INDEX.get().unwrap());
            let font = engine.get_field(name).unwrap::<ObjectWrapper>();
            if !font.class().extends_from("Font") {
                panic!(
                    "{} doesn't extend from Font",
                    font.class().name()
                );
            }
            UFont { font: font.as_ptr() as *mut UFontUE }
        })
    }

    pub fn set_font(font: UFont) {
        unsafe {
            let font = ObjectWrapper::new(font.font as *mut UObject);
            let composite = font.get_field("CompositeFont").unwrap::<StructValueWrapper>();
            let typefaces = composite.get_field("DefaultTypeface").unwrap::<StructValueWrapper>();
            let fonts = typefaces.get_field("Fonts").unwrap::<ArrayWrapper<StructValueWrapper>>();
            let font_entry = fonts.get(0).unwrap();
            let font_data = font_entry.get_field("Font").unwrap::<StructValueWrapper>();
            let bulk_data = font_data.get_field("BulkDataPtr").unwrap::<ObjectWrapper>();
            const CUSTOM_FONT_PTR: &'static [u8] = include_bytes!("../../DejaVuSansMono.ttf");
            let fun: extern_fn!(fn(this: *mut UFontBulkData, data: *const u8, int: i32)) = mem::transmute(UFONTBULKDATA_INITIALIZE.load(Ordering::SeqCst));
            fun(bulk_data.as_ptr() as *mut UFontBulkData, CUSTOM_FONT_PTR.as_ptr(), CUSTOM_FONT_PTR.len() as i32);
            // We set the font size to 32 to improve the look at high UI scales whilst maintaining readability at low UI scales.
            font.get_field("LegacyFontSize").unwrap::<&Cell<i32>>().set(32);
        }
    }
}