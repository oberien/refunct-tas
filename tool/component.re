struct Component {
    draw_hud: fn(string) -> string,
    tick_fn: fn() -> Step,
    on_tick: fn(),
    on_yield: fn(),
    on_new_game: fn(),
    on_level_change: fn(int, int),
    on_reset: fn(int, int),
    on_platforms_change: fn(int, int),
    on_buttons_change: fn(int, int),
    on_key_down: fn(KeyCode, bool),
    on_key_up: fn(KeyCode),
    on_mouse_move: fn(int, int),
    on_component_exit: fn(),
}

static NOOP_COMPONENT = Component {
    draw_hud: fn(text: string) -> string { text },
    tick_fn: Tas::step,
    on_tick: fn() {},
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
};

static mut CURRENT_COMPONENT = NOOP_COMPONENT;

fn set_current_component(component: Component) {
    if component == CURRENT_COMPONENT {
        return;
    }
    let on_component_exit = CURRENT_COMPONENT.on_component_exit;
    on_component_exit();
    CURRENT_COMPONENT = component;
}
