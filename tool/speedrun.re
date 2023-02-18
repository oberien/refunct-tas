static mut SPEEDRUN_STATE = SpeedrunState {
    cur_time: 0.,
    timer_active: false,
};

struct SpeedrunState {
    cur_time: float,
    timer_active: bool,
}

impl SpeedrunState {
    fn get_start_time(self) -> float {
        let ls = Tas::get_level_state();
        ls.start_seconds.to_float() + ls.start_partial_seconds
    }
    fn start_timer(self) {
        SPEEDRUN_STATE.timer_active = false;
        SPEEDRUN_STATE.cur_time = 0.;
        SPEEDRUN_STATE.timer_active = true;
    }
    fn format_time(self, time: float) -> float {
        float::floor(time, 2)
    }
}

static SPEEDRUN_COMPONENT = Component {
    id: SPEEDRUN_COMPONENT_ID,
    conflicts_with: List::of(SPEEDRUN_COMPONENT_ID),
    draw_hud: fn(text: string) -> string {
        let mut time = f"{SPEEDRUN_STATE.cur_time.to_int()/60}:{SPEEDRUN_STATE.cur_time.to_int() % 60:02}.{(SPEEDRUN_STATE.cur_time - SPEEDRUN_STATE.cur_time.to_int().to_float()) * 100.:02.0}";
        let mut text = f"{time}\n{text}";
        text
    },
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {
        match SPEEDRUN_STATE.timer_active {
            true => SPEEDRUN_STATE.cur_time = float::floor(Tas::get_accurate_real_time() - SPEEDRUN_STATE.get_start_time(), 2),
            false => SPEEDRUN_STATE.cur_time = SPEEDRUN_STATE.cur_time,
        }
    },
    on_yield: fn() {},
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {
        match new {
            31 => SPEEDRUN_STATE.timer_active = false,
            0 => SPEEDRUN_STATE.start_timer(),
            _ => print(SPEEDRUN_STATE.cur_time),
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
