use super::AMYCHARACTER_FORCEDUNCROUCH;

hook! {
    "AMyCharacter::ForcedUnCrouch",
    AMYCHARACTER_FORCEDUNCROUCH,
    hook_newgame,
    unhook_newgame,
    new_game,
    true,
}

hook_fn_always! {
    new_game,
    ::native::new_game,
    hook_newgame,
    unhook_newgame,
    AMYCHARACTER_FORCEDUNCROUCH,
    intercept before original,
}
