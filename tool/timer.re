static mut TIMER_STATE = TimerState {
    cur_time: 0.,
    is_timer_active: false,
};

struct TimerState {
    cur_time: float,
    is_timer_active: bool,
}

impl TimerState {
    fn get_start_time(self) -> float {
        let ls = Tas::get_level_state();
        ls.start_seconds.to_float() + ls.start_partial_seconds
    }
    fn get_end_time(self) -> float {
        let ls = Tas::get_level_state();
        ls.end_seconds.to_float() + ls.end_partial_seconds
    }
    fn start_timer(self) {
        TIMER_STATE.cur_time = 0.;
        TIMER_STATE.is_timer_active = true;
    }
}

static TIMER_COMPONENT = Component {
    id: TIMER_COMPONENT_ID,
    conflicts_with: List::of(TIMER_COMPONENT_ID),
    draw_hud: fn(text: string) -> string {
        let mut foo = TIMER_STATE.cur_time;
        let mut time = f"{foo.to_int()/60}:{foo.to_int() % 60:02}.{float::to_int(foo * 100.) % 100:02}";
        let mut text = f"{time}\n{text}";
        text
    },
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {
        if TIMER_STATE.is_timer_active {
            TIMER_STATE.cur_time = Tas::get_accurate_real_time() - TIMER_STATE.get_start_time();
        }
    },
    on_yield: fn() {},
    on_new_game: fn() {
        TIMER_STATE.start_timer();
    },
    on_level_change: fn(old: int, new: int) {
        match new {
            31 => {
                TIMER_STATE.is_timer_active = false;
                TIMER_STATE.cur_time = TIMER_STATE.get_end_time() - TIMER_STATE.get_start_time();
            },
            _ => {},
        }
    },
    on_reset: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_key_down: fn(key_code: KeyCode, is_repeat: bool) {},
    on_key_up: fn(key_code: KeyCode) {},
    on_mouse_move: fn(x: int, y: int) {},
    on_component_exit: fn() {},
};
