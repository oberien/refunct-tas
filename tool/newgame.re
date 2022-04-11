static NEW_GAME_100_PERCENT_COMPONENT = Component {
    draw_hud: fn(text: string) -> string {
        f"{text}\nNew Game Action: 100%"
    },
    tick_fn: Tas::step,
    on_tick: fn() {},
    on_yield: fn() {},
    on_new_game: fn() {
        Tas::set_level(30);
        // wait for all platforms to rise
        let delta = Tas::get_delta();
        Tas::set_delta(Option::Some(1. / 2.));
        wait(9);
        Tas::set_delta(delta);

        trigger_all_platforms();
        trigger_all_buttons_up_to(36);
        teleport_all_cubes();
        teleport_exact(30);
    },
    on_level_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {
        Tas::set_level(0);
    },
    on_platforms_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_key_down: fn(key: KeyCode, is_repeat: bool) {},
    on_key_up: fn(key: KeyCode) {},
    on_mouse_move: fn(x: int, y: int) {},
    on_component_exit: fn() {},
};
static NEW_GAME_ALL_BUTTONS_COMPONENT = Component {
    draw_hud: fn(text: string) -> string {
        f"{text}\nNew Game Action: All Buttons"
    },
    tick_fn: Tas::step,
    on_tick: fn() {},
    on_yield: fn() {},
    on_new_game: fn() {
        Tas::set_level(29);
    },
    on_level_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {
        Tas::set_level(0);
    },
    on_platforms_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_key_down: fn(key: KeyCode, is_repeat: bool) {},
    on_key_up: fn(key: KeyCode) {},
    on_mouse_move: fn(x: int, y: int) {},
    on_component_exit: fn() {},
};
static NEW_GAME_NGG_COMPONENT = Component {
    draw_hud: fn(text: string) -> string {
        f"{text}\nNew Game Action: NGG"
    },
    tick_fn: Tas::step,
    on_tick: fn() {},
    on_yield: fn() {},
    on_new_game: fn() {
        Tas::set_level(1);
    },
    on_level_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {
        Tas::set_level(1);
    },
    on_platforms_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_key_down: fn(key: KeyCode, is_repeat: bool) {},
    on_key_up: fn(key: KeyCode) {},
    on_mouse_move: fn(x: int, y: int) {},
    on_component_exit: fn() {},
};

fn on_level_state_change(old: LevelState, new: LevelState) {
    GAME_STATS.current_level = new.level;
    GAME_STATS.current_buttons = new.buttons;
    GAME_STATS.current_cubes = new.cubes;
    GAME_STATS.current_platforms = new.platforms;
    GAME_STATS.total_resets = new.resets;

    if old.level < new.level && new.level == 31 {
        GAME_STATS.total_runs_completed += 1;
        if new.platforms == 251 {
            GAME_STATS.total_all_platforms_runs_completed += 1;
        }
        if new.cubes == 18 {
            GAME_STATS.total_all_cubes_runs_completed += 1;
        }
        if new.platforms == 251 && new.cubes == 18 {
            GAME_STATS.total_100_runs_completed += 1;
        }
        if GAME_STATS.fewest_platform_run == 0 {
            GAME_STATS.fewest_platform_run = new.platforms;
        } else {
            GAME_STATS.fewest_platform_run = GAME_STATS.fewest_platform_run.min(new.platforms);
        }
    }

    if old.level < new.level {
        GAME_STATS.total_levels += 1;
    }
    if old.buttons < new.buttons {
        GAME_STATS.total_buttons += 1;
    }
    if old.cubes < new.cubes {
        GAME_STATS.total_cubes += 1;
    }
    if old.platforms < new.platforms {
        GAME_STATS.total_platforms += 1;
    }

    // NGG works due to the following:
    // When "New Game" is pressed, the first tick increments the resets value
    // and resets all values like the level. But only during the second tick
    // the level value is actually read and the pillars raised accordingly and
    // the level value reset again.
    // Thus, if we set the level value to 1 on-reset, two pillars / levels will rise.
    // We still need to set the level value to 1 again on-level-change.

    // level changed
    if old.level != new.level
        // new game but no level change will be triggered because we hit new game
        // when level was still 0
        || old.resets < new.resets && old.level == 0
    {
        let on_level_change = CURRENT_COMPONENT.on_level_change;
        on_level_change(old.level, new.level);
    }
    if old.resets != new.resets {
        let on_reset = CURRENT_COMPONENT.on_reset;
        on_reset(old.resets, new.resets);
    }
    if old.platforms != new.platforms {
        let on_platforms_change = CURRENT_COMPONENT.on_platforms_change;
        on_platforms_change(old.platforms, new.platforms);
    }
    if old.buttons != new.buttons {
        let on_buttons_change = CURRENT_COMPONENT.on_buttons_change;
        on_buttons_change(old.buttons, new.buttons);
    }
}

struct GameStats {
    current_level: int,
    current_buttons: int,
    current_cubes: int,
    current_platforms: int,
    total_resets: int,
    total_runs_completed: int,
    total_100_runs_completed: int,
    total_all_platforms_runs_completed: int,
    total_all_cubes_runs_completed: int,
    total_levels: int,
    total_buttons: int,
    total_cubes: int,
    total_platforms: int,
    fewest_platform_run: int,
}
impl GameStats {
    fn new() -> GameStats {
        let level_state = Tas::get_level_state();
        GameStats {
            current_level: level_state.level,
            current_buttons: level_state.buttons,
            current_cubes: level_state.cubes,
            current_platforms: level_state.platforms,
            total_resets: 0,
            total_runs_completed: 0,
            total_100_runs_completed: 0,
            total_all_platforms_runs_completed: 0,
            total_all_cubes_runs_completed: 0,
            total_levels: 0,
            total_buttons: 0,
            total_cubes: 0,
            total_platforms: 0,
            fewest_platform_run: 0,
        }
    }
    fn reset(mut self) {
        let level_state = Tas::get_level_state();
        self.current_level = level_state.level;
        self.current_buttons = level_state.buttons;
        self.current_cubes = level_state.cubes;
        self.current_platforms = level_state.platforms;
        self.total_resets = 0;
        self.total_runs_completed = 0;
        self.total_100_runs_completed = 0;
        self.total_all_platforms_runs_completed = 0;
        self.total_all_cubes_runs_completed = 0;
        self.total_levels = 0;
        self.total_buttons = 0;
        self.total_cubes = 0;
        self.total_platforms = 0;
        self.fewest_platform_run = 0;
    }
}

static mut GAME_STATS = GameStats::new();
