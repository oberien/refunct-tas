use hook::{ArgsRef, IsaAbi, RawHook};

pub fn apply_resolution_settings<IA: IsaAbi>(hook: &RawHook<IA, ()>, args: ArgsRef<'_, IA>) {
    crate::threads::ue::apply_resolution_settings();
    unsafe { hook.call_original_function(args); }
}