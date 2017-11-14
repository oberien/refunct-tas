use std::slice;

use byteorder::{WriteBytesExt, LittleEndian};

use super::UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE;

hook! {
    "UEngine::UpdateTimeAndHandleMaxTickRate",
    UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE,
    hook_tick,
    unhook_tick,
    tick,
    false,
}

hook_fn_always! {
    tick,
    ::native::tick_intercept,
    hook_tick,
    unhook_tick,
    UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE,
    intercept after original,
}
