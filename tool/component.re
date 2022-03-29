struct Component {
    draw_hud: fn(string) -> string,
    tick: fn(),
    on_new_game: fn(),
    on_level_change: fn(int, int),
    on_reset: fn(int, int),
    on_platforms_change: fn(int, int),
    on_buttons_change: fn(int, int),
    on_component_exit: fn(),
}

static NOOP_COMPONENT = Component {
    draw_hud: fn(text: string) -> string { text },
    tick: fn() {},
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
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
