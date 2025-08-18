use hook::{ArgsRef, IsaAbi, RawHook, TypedHook};
use crate::native::character::AMyCharacterUE;
use crate::native::ALiftBaseUE;

pub fn tick_hook<IA: IsaAbi>(hook: &'static RawHook<IA, ()>, args: ArgsRef<'_, IA>) {
    unsafe { hook.call_original_function(args); }
    crate::threads::ue::tick();
}

pub fn add_based_character_hook<IA: IsaAbi>(hook: &TypedHook<IA, fn(*mut ALiftBaseUE, *mut AMyCharacterUE), ()>, this: *mut ALiftBaseUE, character: *mut AMyCharacterUE) {
    crate::threads::ue::add_based_character(this);
    unsafe { hook.call_original_function((this, character)) };
}

pub fn remove_based_character_hook<IA: IsaAbi>(hook: &TypedHook<IA, fn(*mut ALiftBaseUE, *mut AMyCharacterUE), ()>, this: *mut ALiftBaseUE, character: *mut AMyCharacterUE) {
    crate::threads::ue::remove_based_character(this);
    unsafe { hook.call_original_function((this, character)) };
}
