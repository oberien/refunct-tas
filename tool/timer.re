static mut TIMER_STATE = TimerState {
    is_timer_active: false,
};

struct TimerState {
    is_timer_active: bool,
}

fn save_splits(path: string) {
    match LiveSplit::save_splits(path) {
        Result::Ok(_unit) => (),
        Result::Err(e) => match e {
            SplitsSaveError::CreationFailed(filename, error) => log(f"ERROR: Failed to create {filename}: {error}"),
            SplitsSaveError::SaveFailed(filename, error) => log(f"ERROR: Failed to save {filename}: {error}"),
            SplitsSaveError::DisallowedFilePath(filename) => log(f"ERROR: Failed to save {filename}: Disallowed file path"),
        },
    }
    Result::Ok(())
}
fn load_splits(path: string) {
    match LiveSplit::load_splits(path) {
        Result::Ok(_unit) => (),
        Result::Err(e) => match e {
            SplitsLoadError::OpenFailed(filename, error) => log(f"ERROR: Failed to open {filename}: {error}"),
            SplitsLoadError::ParseFailed(filename, error) => log(f"ERROR: Failed to parse {filename}: {error}"),
        },
    }
    Result::Ok(())
}

impl TimerState {
    fn get_start_time() -> float {
        let ls = Tas::get_level_state();
        ls.start_seconds.to_float() + ls.start_partial_seconds
    }
    fn get_end_time() -> float {
        let ls = Tas::get_level_state();
        ls.end_seconds.to_float() + ls.end_partial_seconds
    }
}

static TIMER_COMPONENT = Component {
    id: TIMER_COMPONENT_ID,
    conflicts_with: List::of(TIMER_COMPONENT_ID),
    draw_hud_text: fn(text: string) -> string {
        let time = LiveSplit::get_game_time();
        let time = f"{time.to_int()/60}:{time.to_int() % 60:02}.{float::to_int(time * 100.) % 100:02}";
        let mut text = f"{time}\n{text}";
        text
    },
    draw_hud_always: fn() {
        let time = LiveSplit::get_game_time();
        let time = f"{time.to_int()/60}:{time.to_int() % 60:02}.{float::to_int(time * 100.) % 100:02}";
//        Tas::draw_text(DrawText {
//            text: f"{LiveSplit::get_game_name()}\n{LiveSplit::get_category_name()}\n{LiveSplit::get_attempt_count()}\n{time}",
//            color: Color { red: 0., green: 0., blue: 0., alpha: 1.},
//            x: 10. + (2. * SETTINGS.ui_scale),
//            y: 100. + (2. * SETTINGS.ui_scale),
//            scale: SETTINGS.ui_scale,
//            scale_position: false,
//        });
        Tas::draw_text(DrawText {
            text: f"{LiveSplit::get_game_name()}\n{LiveSplit::get_category_name()}\n{LiveSplit::get_attempt_count()}\n{time}",
            color: Color { red: 1., green: 1., blue: 1., alpha: 1.},
            x: 10.,
            y: 100.,
            scale: SETTINGS.ui_scale,
            scale_position: false,
        });
        let viewport = Tas::get_viewport_size();
        let txt = f"{time}";
        let text_size = Tas::get_text_size(txt, 1.);
        Tas::draw_text(DrawText {
            text: f"{time}",
            color: Color { red: 0., green: 1., blue: 1., alpha: 1.},
            x: (viewport.width.to_float() / 2.) - (text_size.width / 2.),
            y: 0.,
            scale: 1.,
            scale_position: false,
        });
    },
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {
        if TIMER_STATE.is_timer_active {
            LiveSplit::set_game_time(Tas::get_accurate_real_time() - TimerState::get_start_time());
        }
    },
    on_yield: fn() {},
    on_new_game: fn() {
        TIMER_STATE.is_timer_active = true;
        LiveSplit::start();
    },
    on_level_change: fn(old: int, new: int) {
        if new == 10 {
            LiveSplit::pause_game_time();
            LiveSplit::split();
            TIMER_STATE.is_timer_active = false;
        }
        match new {
            31 => {
                LiveSplit::pause_game_time();
                LiveSplit::set_game_time(TimerState::get_end_time() - TimerState::get_start_time());
                LiveSplit::split();
                TIMER_STATE.is_timer_active = false;
            },
            _ => {},
        }
    },
    on_buttons_change: fn(old: int, new: int) {},
    on_cubes_change: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_element_pressed: fn(index: ElementIndex) {},
    on_element_released: fn(index: ElementIndex) {},
    on_key_down: fn(key_code: KeyCode, is_repeat: bool) {},
    on_key_down_always: fn(key: KeyCode, is_repeat: bool) {},
    on_key_up: fn(key_code: KeyCode) {},
    on_key_up_always: fn(key: KeyCode) {},
    on_mouse_move: fn(x: int, y: int) {},
    on_component_enter: fn() {},
    on_component_exit: fn() {},
    on_resolution_change: fn() {},
    on_menu_open: fn() {},
};
