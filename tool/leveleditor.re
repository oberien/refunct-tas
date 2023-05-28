fn create_level_editor_menu() -> Ui {
    Ui::new("Level Editor\nWhat would you like to modify? (format: cl14,pl2)", List::of(
        UiElement::Input(Input {
            label: Text { text: "\n     Input" },
            input: "",
            onclick: fn(input: string) {
                let list = List::new();
                for val in input.find_matches("\\d+") {
                    list.push(val);
                    if list.len() <= 2 {
                        print(val);
                    }
                }
                Tas::set_all_cluster_speeds(9999999.);
                let loc = Tas::get_location();
                let rot = Tas::get_rotation();
                Tas::key_down(KEY_ESCAPE.large_value, KEY_ESCAPE.large_value, false);
                Tas::key_up(KEY_ESCAPE.large_value, KEY_ESCAPE.large_value, false);
                Tas::step();
                Tas::key_down(KEY_RETURN.large_value, KEY_RETURN.large_value, false);
                Tas::key_up(KEY_RETURN.large_value, KEY_RETURN.large_value, false);
                Tas::step();
                Tas::key_down(KEY_LEFT.large_value, KEY_LEFT.large_value, false);
                Tas::key_up(KEY_LEFT.large_value, KEY_LEFT.large_value, false);
                Tas::step();
                Tas::key_down(KEY_RETURN.large_value, KEY_RETURN.large_value, false);
                Tas::key_up(KEY_RETURN.large_value, KEY_RETURN.large_value, false);
                Tas::step();
                Tas::step();
                Tas::set_location(loc);
                Tas::set_rotation(rot);
            },
            onchange: fn(input: string) {},
        }),
        UiElement::Button(UiButton {
            label: Text { text: "\n\nBack" },
            onclick: fn(label: Text) { leave_ui() },
        }),
    ))
};

static LEVEL_EDITOR_COMPONENT = Component {
    id: LEVEL_EDITOR_COMPONENT_ID,
    conflicts_with: List::of(LEVEL_EDITOR_COMPONENT_ID),
    draw_hud_text: fn(text: string) -> string { f"{text}\nLevel Editor" },
    draw_hud_always: fn() {},
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {
    },
    on_yield: fn() {},
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_key_down: fn(key: KeyCode, is_repeat: bool) {},
    on_key_up: fn(key: KeyCode) {},
    on_mouse_move: fn(x: int, y: int) {},
    on_component_exit: fn() {},
    on_resolution_change: fn() {},
    on_menu_open: fn() {},
};