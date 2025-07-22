fn step_frame(tick_mode: TickMode) {
    let mut delta = Option::None;
    for comp in CURRENT_COMPONENTS {
        match comp.requested_delta_time {
            Option::None => (),
            Option::Some(d) => if delta.is_none() {
                delta = Option::Some(d);
            } else {
                delta = Option::Some(delta.unwrap().max(d));
            }
        }
    }
    Tas::set_delta(delta);
    let tick_fn = match tick_mode {
        TickMode::DontCare => Tas::step,
        TickMode::Yield => Tas::yield_,
    };
    match tick_fn() {
        Step::Tick => (),
        Step::NewGame => {
            for comp in CURRENT_COMPONENTS {
                let on_new_game = comp.on_new_game;
                on_new_game();
            }
        },
        Step::Yield => {
            for comp in CURRENT_COMPONENTS {
                let on_yield = comp.on_yield;
                on_yield();
            }
            return
        }
    }
    for comp in CURRENT_COMPONENTS {
        let on_tick = comp.on_tick;
        on_tick();
    }
}

