use std::slice;

use byteorder::{WriteBytesExt, LittleEndian};

use super::AMYCHARACTER_EXECFORCEDUNCROUCH;

hook! {
    "AMyCharacter::execForcedUnCrouch",
    AMYCHARACTER_EXECFORCEDUNCROUCH,
    hook_newgame,
    unhook_newgame,
    new_game,
    true
}

hook_fn_always! {
    new_game,
    ::native::new_game,
    hook_newgame,
    unhook_newgame,
    AMYCHARACTER_EXECFORCEDUNCROUCH,
    intercept before original,
}
