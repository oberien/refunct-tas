fn step_frame(delta: Option<float>, tick_fn: fn() -> Step) {
    Tas::set_delta(delta);
    match tick_fn() {
        Step::Tick => (),
        Step::NewGame => {
            let on_new_game = CURRENT_COMPONENT.on_new_game;
            on_new_game();
        },
        Step::Yield => {
            let on_yield = CURRENT_COMPONENT.on_yield;
            on_yield();
            return
        }
    }
    let on_tick = CURRENT_COMPONENT.on_tick;
    on_tick();
}