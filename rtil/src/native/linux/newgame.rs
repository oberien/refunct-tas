use super::AMYCHARACTER_EXECFORCEDUNCROUCH;

hook_beginning! {
    hook_newgame,
    restore_newgame,
    new_game,
    "AMyCharacter::execForcedUnCrouch",
    AMYCHARACTER_EXECFORCEDUNCROUCH,
}

hook_fn_always! {
    new_game,
    ::native::new_game,
    hook_newgame,
    restore_newgame,
    AMYCHARACTER_EXECFORCEDUNCROUCH,
}
