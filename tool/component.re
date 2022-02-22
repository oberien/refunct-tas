struct Component {
    draw_hud: fn(string) -> string,
    on_new_game: fn(),
    on_level_change: fn(int),
    on_reset: fn(int),
}

static NOOP_COMPONENT = Component {
    draw_hud: fn(text: string) -> string { text },
    on_new_game: fn() {},
    on_level_change: fn(level: int) {},
    on_reset: fn(reset: int) {},
};

static mut CURRENT_COMPONENT = NOOP_COMPONENT;

