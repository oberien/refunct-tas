use native::AMYCHARACTER_FORCEDUNCROUCH;

hook! {
    "AMyCharacter::ForcedUnCrouch",
    AMYCHARACTER_FORCEDUNCROUCH,
    hook,
    unhook,
    new_game,
    true,
}

hook_fn_always! {
    new_game,
    ::threads::ue::new_game,
    hook,
    unhook,
    AMYCHARACTER_FORCEDUNCROUCH,
    intercept before original,
}
