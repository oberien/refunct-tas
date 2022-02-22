static NEW_GAME_100_PERCENT_COMPONENT = Component {
    draw_hud: fn(text: string) -> string {
        f"{text}\nNew Game Action: 100%"
    },
    on_new_game: fn() {
        teleport_buttons(30);
        teleport_all_platforms();
        teleport_all_cubes();
        teleport_exact(30);
    },
    on_level_change: fn(level: int) {},
    on_reset: fn(reset: int) {},
};
static NEW_GAME_ALL_BUTTONS_COMPONENT = Component {
    draw_hud: fn(text: string) -> string {
        f"{text}\nNew Game Action: All Buttons"
    },
    on_new_game: fn() {},
    on_level_change: fn(level: int) {
        if level == 0 {
            Tas::set_level(29);
        }
    },
    on_reset: fn(reset: int) {
        Tas::set_level(0);
    },
};
static NEW_GAME_NGG_COMPONENT = Component {
    draw_hud: fn(text: string) -> string {
        f"{text}\nNew Game Action: NGG"
    },
    on_new_game: fn() {},
    on_level_change: fn(level: int) {
        if level == 0 {
            Tas::set_level(1);
        }
    },
    on_reset: fn(reset: int) {
        Tas::set_level(1);
    },
};

fn on_level_change(level: int) {
    let on_level_change = CURRENT_COMPONENT.on_level_change;
    on_level_change(level);
}

fn on_reset(reset: int) {
    let on_reset = CURRENT_COMPONENT.on_reset;
    on_reset(reset);
}
