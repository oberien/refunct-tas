use crate::native::Args;

#[rtil_derive::hook_before(AMyCharacter::ForcedUnCrouch)]
fn new_game(_args: &mut Args) {
    crate::threads::ue::new_game();
}
