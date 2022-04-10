static mut WINDSCREEN_WIPERS_STATE = WindscreenWipersState {
    last_update: 0,
    seconds_per_wipe: 0.,
    direction: 1.,
};

struct WindscreenWipersState {
    last_update: int,
    seconds_per_wipe: float,
    direction: float,
}

fn start_windscreen_wipers(seconds_per_wipe: float) {
    WINDSCREEN_WIPERS_STATE.last_update = current_time_millis();
    WINDSCREEN_WIPERS_STATE.seconds_per_wipe = seconds_per_wipe;
    WINDSCREEN_WIPERS_STATE.direction = 1.;
}

static WINDSCREEN_WIPERS_COMPONENT = Component {
    draw_hud: fn(text: string) -> string {
        f"{text}\nWindscreen Wipers ({WINDSCREEN_WIPERS_STATE.seconds_per_wipe}s/wipe)"
    },
    tick_fn: Tas::step,
    on_tick: fn() {
        if WINDSCREEN_WIPERS_STATE.seconds_per_wipe == 0. {
            return;
        }
        let time = current_time_millis();
        let mut rot = Tas::get_rotation();
        let delta = (time - WINDSCREEN_WIPERS_STATE.last_update);
        let delta =  delta.to_float() / 1000.;
        let turn_per_second = 360. / WINDSCREEN_WIPERS_STATE.seconds_per_wipe;
        rot.roll += turn_per_second * delta * WINDSCREEN_WIPERS_STATE.direction;
        Tas::set_rotation(rot);
        WINDSCREEN_WIPERS_STATE.last_update = time;
        if 89.5 <= rot.roll && rot.roll <= 180. {
            WINDSCREEN_WIPERS_STATE.direction = -1.;
        } else if rot.roll <= -89.5 || 180. <= rot.roll && rot.roll <= 280. {
            WINDSCREEN_WIPERS_STATE.direction = 1.;
        }
    },
    on_yield: fn() {},
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_key_down: fn(key: KeyCode, is_repeat: bool) {},
    on_key_up: fn(key: KeyCode) {},
    on_component_exit: fn() {},
};

// Frame-Step Component

static mut IS_F_PRESSED = false;

static FRAME_STEP_COMPONENT = Component {
    draw_hud: fn(text: string) -> string {
        f"{text}\nFrame-Step: f advance one frame, q to quit (requires 60 FPS)"
    },
    tick_fn: Tas::yield,
    on_tick: fn() {},
    on_yield: fn() {
        if IS_F_PRESSED {
            advance_frame();
        }
    },
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_key_down: fn(key: KeyCode, is_repeat: bool) {
        let key = key.to_small();
        if key == KEY_F.to_small() {
            if is_repeat {
                IS_F_PRESSED = true;
            }
            advance_frame();
        } else if key == KEY_Q.to_small() {
            set_current_component(NOOP_COMPONENT);
        }
    },
    on_key_up: fn(key: KeyCode) {
        if key.to_small() == KEY_F.to_small() {
            IS_F_PRESSED = false;
        }
    },
    on_component_exit: fn() {},
};
fn advance_frame() {
    let old = Tas::get_delta();
    Tas::set_delta(Option::Some(1./60.));
    Tas::step();
    Tas::set_delta(old);
}
