#[rtil_derive::hook_before(AMyCharacter::ForcedUnCrouch)]
fn new_game() {
    crate::threads::ue::new_game();
}
