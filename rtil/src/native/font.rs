use std::cell::Cell;
use crate::native::reflection::{ObjectWrapper, UObject};
use crate::native::uworld::ENGINE_INDEX;
use crate::native::{ArrayWrapper, StructValueWrapper, UeScope, UFONTBULKDATA_INITIALIZE};
use std::sync::atomic::Ordering;
use std::mem;

pub struct UFont(*mut UFontUE);
pub(in crate::native) enum UFontUE {}

// WARNING: somewhat unsound - see AMyCharacter
unsafe impl Send for UFont {}

#[repr(C)]
struct UFontBulkData {
    // stub, we only need a pointer to this struct
}

impl UFont {
    pub fn get_font(name: &str) -> *mut UFontUE {
        UeScope::with(|scope| {
            let engine = scope.get(ENGINE_INDEX.get().unwrap());
            let font = engine.get_field(name).unwrap::<ObjectWrapper>();
            font.as_ptr() as *mut UFontUE
        })
    }

    pub fn set_font(font: *mut UFontUE) {
        unsafe {
            let font2 = ObjectWrapper::new(font as *mut UObject);
            let composite = font2.get_field("CompositeFont").unwrap::<StructValueWrapper>();
            let typefaces = composite.get_field("DefaultTypeface").unwrap::<StructValueWrapper>();
            let fonts = typefaces.get_field("Fonts").unwrap::<ArrayWrapper<StructValueWrapper>>();
            let font_entry = fonts.get(0).unwrap();
            let font_data = font_entry.get_field("Font").unwrap::<StructValueWrapper>();
            let bulk_data = font_data.get_field("BulkDataPtr").unwrap::<ObjectWrapper>();
            const CUSTOM_FONT_PTR: &'static [u8] = include_bytes!("../../DejaVuSansMono.ttf");
            let fun: extern_fn!(fn(this: *mut UFontBulkData, data: *const u8, int: i32)) = unsafe { mem::transmute(UFONTBULKDATA_INITIALIZE.load(Ordering::SeqCst)) };
            fun(bulk_data.as_ptr() as *mut UFontBulkData, CUSTOM_FONT_PTR.as_ptr(), CUSTOM_FONT_PTR.len() as i32);
            font2.get_field("LegacyFontSize").unwrap::<&Cell<i32>>().set(32);
        }
    }
}