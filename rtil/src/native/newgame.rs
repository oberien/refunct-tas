use hook::{ArgsRef, IsaAbi, RawHook};

pub fn new_game_hook<IA: IsaAbi>(hook: &RawHook<IA, ()>, args: ArgsRef<'_, IA>) {
    crate::threads::ue::new_game();
    unsafe { hook.call_original_function(args) };
}
