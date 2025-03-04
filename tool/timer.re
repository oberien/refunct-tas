static mut TIMER_STATE = TimerState {
    is_timer_active: false,
};

struct TimerState {
    is_timer_active: bool,
}

fn save_splits(path: string) {
    match Tas::timer_save_splits(path) {
        Result::Ok(foo) => return,
        Result::Err(e) => {
            let msg = match e {
                SplitsSaveError::CreationError(msg) => msg,
                SplitsSaveError::SaveError(msg) => msg,
            };
            log(f"{msg}");
        }
    }
}
fn load_splits(path: string) {
    match Tas::timer_load_splits(path) {
        Result::Ok(foo) => return,
        Result::Err(e) => {
            let msg = match e {
                SplitsLoadError::OpenError(msg) => msg,
                SplitsLoadError::ParseError(msg) => msg,
            };
            log(f"{msg}");
        }
    }
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
        let time = Tas::timer_get_game_time();
        let time = f"{time.to_int()/60}:{time.to_int() % 60:02}.{float::to_int(time * 100.) % 100:02}";
        let mut text = f"{time}\n{text}";
        text
    },
    draw_hud_always: fn() {},
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {
        if TIMER_STATE.is_timer_active {
            Tas::timer_set_game_time(Tas::get_accurate_real_time() - TimerState::get_start_time());
        }
    },
    on_yield: fn() {},
    on_new_game: fn() {
        TIMER_STATE.is_timer_active = true;
        Tas::timer_start();
    },
    on_level_change: fn(old: int, new: int) {
        match new {
            31 => {
                Tas::timer_pause_game_time();
                Tas::timer_set_game_time(TimerState::get_end_time() - TimerState::get_start_time());
                Tas::timer_split();
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
