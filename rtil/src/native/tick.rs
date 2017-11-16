use native::UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE;

hook! {
    "UEngine::UpdateTimeAndHandleMaxTickRate",
    UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE,
    hook,
    unhook,
    tick,
    false,
}

hook_fn_always! {
    tick,
    ::native::tick_intercept,
    hook,
    unhook,
    UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE,
    intercept after original,
}
