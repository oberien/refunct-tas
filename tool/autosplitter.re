fn button_index_to_setting(index: int) -> bool {
    match index {
        1 => SETTINGS.autosplitter_button_1,
        2 => SETTINGS.autosplitter_button_2,
        3 => SETTINGS.autosplitter_button_3,
        4 => SETTINGS.autosplitter_button_4,
        5 => SETTINGS.autosplitter_button_5,
        6 => SETTINGS.autosplitter_button_6,
        7 => SETTINGS.autosplitter_button_7,
        8 => SETTINGS.autosplitter_button_8,
        9 => SETTINGS.autosplitter_button_9,
        10 => SETTINGS.autosplitter_button_10,
        11 => SETTINGS.autosplitter_button_11,
        12 => SETTINGS.autosplitter_button_12,
        13 => SETTINGS.autosplitter_button_13,
        14 => SETTINGS.autosplitter_button_14,
        15 => SETTINGS.autosplitter_button_15,
        16 => SETTINGS.autosplitter_button_16,
        17 => SETTINGS.autosplitter_button_17,
        18 => SETTINGS.autosplitter_button_18,
        19 => SETTINGS.autosplitter_button_19,
        20 => SETTINGS.autosplitter_button_20,
        21 => SETTINGS.autosplitter_button_21,
        22 => SETTINGS.autosplitter_button_22,
        23 => SETTINGS.autosplitter_button_23,
        24 => SETTINGS.autosplitter_button_24,
        25 => SETTINGS.autosplitter_button_25,
        26 => SETTINGS.autosplitter_button_26,
        27 => SETTINGS.autosplitter_button_27,
        28 => SETTINGS.autosplitter_button_28,
        29 => SETTINGS.autosplitter_button_29,
        30 => SETTINGS.autosplitter_button_30,
        31 => SETTINGS.autosplitter_button_31,
        32 => SETTINGS.autosplitter_button_32,
        33 => SETTINGS.autosplitter_button_33,
        34 => SETTINGS.autosplitter_button_34,
        35 => SETTINGS.autosplitter_button_35,
        36 => SETTINGS.autosplitter_button_36,
        37 => SETTINGS.autosplitter_button_37,
        _ => false,
    }
}
fn cube_index_to_setting(index: int) -> bool {
    match index {
        1 => SETTINGS.autosplitter_cube_1,
        2 => SETTINGS.autosplitter_cube_2,
        3 => SETTINGS.autosplitter_cube_3,
        4 => SETTINGS.autosplitter_cube_4,
        5 => SETTINGS.autosplitter_cube_5,
        6 => SETTINGS.autosplitter_cube_6,
        7 => SETTINGS.autosplitter_cube_7,
        8 => SETTINGS.autosplitter_cube_8,
        9 => SETTINGS.autosplitter_cube_9,
        10 => SETTINGS.autosplitter_cube_10,
        11 => SETTINGS.autosplitter_cube_11,
        12 => SETTINGS.autosplitter_cube_12,
        13 => SETTINGS.autosplitter_cube_13,
        14 => SETTINGS.autosplitter_cube_14,
        15 => SETTINGS.autosplitter_cube_15,
        16 => SETTINGS.autosplitter_cube_16,
        17 => SETTINGS.autosplitter_cube_17,
        18 => SETTINGS.autosplitter_cube_18,
        _ => false,
    }
}

static AUTOSPLITTER_COMPONENT = Component {
    id: AUTOSPLITTER_COMPONENT_ID,
    conflicts_with: List::of(AUTOSPLITTER_COMPONENT_ID),
    draw_hud_text: fn(text: string) -> string { text },
    draw_hud_always: fn() {},
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {},
    on_yield: fn() {},
    on_new_game: fn() {
        if SETTINGS.autosplitter_reset {
            LiveSplit::reset(true);
            LiveSplit::start();
        }
    },
    on_level_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {
        if new > old && button_index_to_setting(new) {
            LiveSplit::split();
        }
    },
    on_cubes_change: fn(old: int, new: int) {
        if new > old && cube_index_to_setting(new) {
            LiveSplit::split();
        }
    },
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
