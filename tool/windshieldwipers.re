static mut WINDSCREEN_WIPERS_STATE = WindscreenWipersState {
    seconds_per_wipe: 0.,
    direction: 1.,
};

struct WindscreenWipersState {
    seconds_per_wipe: float,
    direction: float,
}

fn start_windscreen_wipers(seconds_per_wipe: float) {
    WINDSCREEN_WIPERS_STATE.seconds_per_wipe = seconds_per_wipe;
    WINDSCREEN_WIPERS_STATE.direction = 1.;
}

static WINDSCREEN_WIPERS_COMPONENT = Component {
    id: WINDSCREEN_WIPERS_COMPONENT_ID,
    conflicts_with: List::of(WINDSCREEN_WIPERS_COMPONENT_ID),
    draw_hud_text: fn(text: string) -> string {
        f"{text}\nWindscreen Wipers ({WINDSCREEN_WIPERS_STATE.seconds_per_wipe}s/wipe)"
    },
    draw_hud_always: fn() {},
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {
        if WINDSCREEN_WIPERS_STATE.seconds_per_wipe == 0. {
            return;
        }
        let mut rot = Tas::get_rotation();
        let delta = Tas::get_last_frame_delta();
        let turn_per_second = 360. / WINDSCREEN_WIPERS_STATE.seconds_per_wipe;
        rot.roll += turn_per_second * delta * WINDSCREEN_WIPERS_STATE.direction;
        Tas::set_rotation(rot);
        if 89.5 <= rot.roll && rot.roll <= 180. {
            WINDSCREEN_WIPERS_STATE.direction = -1.;
        } else if rot.roll <= -89.5 || 180. <= rot.roll && rot.roll <= 280. {
            WINDSCREEN_WIPERS_STATE.direction = 1.;
        }
    },
    on_yield: fn() {},
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_cubes_change: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_element_pressed: fn(index: ElementIndex) {},
    on_element_released: fn(index: ElementIndex) {},
    on_key_down: fn(key: KeyCode, is_repeat: bool) {},
    on_key_down_always: fn(key: KeyCode, is_repeat: bool) {},
    on_key_up: fn(key: KeyCode) {},
    on_key_up_always: fn(key: KeyCode) {},
    on_mouse_move: fn(x: int, y: int) {},
    on_component_enter: fn() {},
    on_component_exit: fn() {},
    on_resolution_change: fn() {},
    on_menu_open: fn() {},
};
