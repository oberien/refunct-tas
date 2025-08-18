use hook::{ArgsRef, IsaAbi, RawHook};

pub fn new_game<IA: IsaAbi>(_hook: &RawHook<IA, ()>, _args: ArgsRef<'_, IA>) {
    crate::threads::ue::new_game();
}
