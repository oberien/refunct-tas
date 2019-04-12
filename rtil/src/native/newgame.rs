use native::AMYCHARACTER_FORCEDUNCROUCH;

#[rtil_derive::hook_before(AMyCharacter::ForcedUnCrouch)]
fn new_game() {
    ::threads::ue::new_game();
}
