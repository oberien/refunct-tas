use crate::native::Args;
use crate::native::UObject;

#[rtil_derive::hook_after(UEngine::UpdateTimeAndHandleMaxTickRate)]
fn tick() {
    crate::threads::ue::tick();
}

#[rtil_derive::hook_before(ALiftBase::AddBasedCharacter)]
fn add_based_player(args: &mut Args) {
    crate::threads::ue::add_based_character(unsafe { args.with_this_pointer::<*mut UObject>() })
}
#[rtil_derive::hook_before(ALiftBase::RemoveBasedCharacter)]
fn remove_based_player(args: &mut Args) {
    crate::threads::ue::remove_based_character(unsafe { args.with_this_pointer::<*mut UObject>() })
}
