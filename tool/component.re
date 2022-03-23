struct Component {
    draw_hud: fn(string) -> string,
    on_new_game: fn(),
    on_level_change: fn(int),
    on_reset: fn(int),
    on_component_exit: fn(),
}

static NOOP_COMPONENT = Component {
    draw_hud: fn(text: string) -> string { text },
    on_new_game: fn() {},
    on_level_change: fn(level: int) {},
    on_reset: fn(reset: int) {},
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
