use super::UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE;

hook_beginning! {
    hook_tick,
    restore_tick,
    tick,
    "UEngine::UpdateTimeAndHandleMaxTickRate",
    UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE,
}

hook_fn_always! {
    tick,
    ::native::tick_intercept,
    hook_tick,
    restore_tick,
    UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE,
    intercept after original
}
