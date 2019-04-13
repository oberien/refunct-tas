#[rtil_derive::hook_after(UEngine::UpdateTimeAndHandleMaxTickRate)]
fn tick() {
    crate::threads::ue::tick();
}
