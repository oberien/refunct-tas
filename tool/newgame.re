fn create_new_game_actions_menu() -> Ui {
    Ui::new("New Game Actions:", List::of(
        UiElement::Button(UiButton {
            label: Text { text: "Nothing" },
            onclick: fn(label: Text) {
                remove_component(NEW_GAME_100_PERCENT_COMPONENT);
                remove_component(NEW_GAME_ALL_BUTTONS_COMPONENT);
                remove_component(NEW_GAME_NGG_COMPONENT);
                leave_ui();
             },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "100%" },
            onclick: fn(label: Text) { add_component(NEW_GAME_100_PERCENT_COMPONENT); leave_ui(); },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "All Buttons" },
            onclick: fn(label: Text) { add_component(NEW_GAME_ALL_BUTTONS_COMPONENT); leave_ui(); },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "NGG" },
            onclick: fn(label: Text) { add_component(NEW_GAME_NGG_COMPONENT); leave_ui(); },
        }),
    ))
}

struct NewGame100PercentState {
    tick: int,
    platform_pawns: List<int>,
    button_pawns: List<int>,
}
static mut NEW_GAME_100_PERCENT_STATE = NewGame100PercentState {
    tick: 100,
    platform_pawns: List::new(),
    button_pawns: List::new(),
};
static mut NEW_GAME_100_PERCENT_COMPONENT = Component {
    id: NEW_GAME_100_PERCENT_COMPONENT_ID,
    conflicts_with: List::of(MULTIPLAYER_COMPONENT_ID, NEW_GAME_100_PERCENT_COMPONENT_ID, NEW_GAME_ALL_BUTTONS_COMPONENT_ID, NEW_GAME_NGG_COMPONENT_ID, PRACTICE_COMPONENT_ID, RANDOMIZER_COMPONENT_ID),
    draw_hud_text: fn(text: string) -> string {
        f"{text}\nNew Game Action: 100%"
    },
    draw_hud_always: fn() {},
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {
        if 0 <= NEW_GAME_100_PERCENT_STATE.tick && NEW_GAME_100_PERCENT_STATE.tick < 9 {
            // wait for all platforms to rise
        } else if 9 == NEW_GAME_100_PERCENT_STATE.tick {
            NEW_GAME_100_PERCENT_STATE.platform_pawns = create_all_platform_pawns();
            NEW_GAME_100_PERCENT_STATE.button_pawns = create_all_button_pawns_up_to(36);
            NEW_GAME_100_PERCENT_COMPONENT.requested_delta_time = Option::None;
        } else if 10 <= NEW_GAME_100_PERCENT_STATE.tick && NEW_GAME_100_PERCENT_STATE.tick < 10 + 18*3 {
            let tick_in_cubes = NEW_GAME_100_PERCENT_STATE.tick - 10;
            let cube = tick_in_cubes / 3;
            let modulo = tick_in_cubes - cube * 3;
            if modulo == 0 {
                Tas::set_location(CUBES.get(cube).unwrap());
            }
        } else if 10 + 18*3 == NEW_GAME_100_PERCENT_STATE.tick {
            for pawn in NEW_GAME_100_PERCENT_STATE.platform_pawns {
                Tas::destroy_pawn(pawn);
            }
            for pawn in NEW_GAME_100_PERCENT_STATE.button_pawns {
                Tas::destroy_pawn(pawn);
            }
            Tas::set_location(BUTTONS.get(36).unwrap().loc);
        }
        NEW_GAME_100_PERCENT_STATE.tick += 1;
    },
    on_yield: fn() {},
    on_new_game: fn() {
        NEW_GAME_100_PERCENT_COMPONENT.requested_delta_time = Option::Some(1. / 2.);
        NEW_GAME_100_PERCENT_STATE.tick = 0;
        Tas::set_level(30);
    },
    on_level_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_cubes_change: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {
        Tas::set_level(0);
    },
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
static NEW_GAME_ALL_BUTTONS_COMPONENT = Component {
    id: NEW_GAME_ALL_BUTTONS_COMPONENT_ID,
    conflicts_with: List::of(MULTIPLAYER_COMPONENT_ID, NEW_GAME_100_PERCENT_COMPONENT_ID, NEW_GAME_ALL_BUTTONS_COMPONENT_ID, RANDOMIZER_COMPONENT_ID),
    draw_hud_text: fn(text: string) -> string {
        f"{text}\nNew Game Action: All Buttons"
    },
    draw_hud_always: fn() {},
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {},
    on_yield: fn() {},
    on_new_game: fn() {
        Tas::set_level(29);
    },
    on_level_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_cubes_change: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {
        Tas::set_level(0);
    },
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
static NEW_GAME_NGG_COMPONENT = Component {
    id: NEW_GAME_NGG_COMPONENT_ID,
    conflicts_with: List::of(MULTIPLAYER_COMPONENT_ID, NEW_GAME_100_PERCENT_COMPONENT_ID, NEW_GAME_NGG_COMPONENT_ID),
    draw_hud_text: fn(text: string) -> string {
        f"{text}\nNew Game Action: NGG"
    },
    draw_hud_always: fn() {},
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {},
    on_yield: fn() {},
    on_new_game: fn() {
        let level = Tas::get_level();
        if level < 29 {
            Tas::set_level(level + 1);
        }
    },
    on_level_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_cubes_change: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {
        let level = Tas::get_level();
        if level < 29 {
            Tas::set_level(level + 1);
        }
    },
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
        for comp in CURRENT_COMPONENTS {
            let on_level_change = comp.on_level_change;
            on_level_change(old.level, new.level);
        }
    }
    if old.resets != new.resets {
        for comp in CURRENT_COMPONENTS {
            let on_reset = comp.on_reset;
            on_reset(old.resets, new.resets);
        }
    }
    if old.buttons != new.buttons {
        for comp in CURRENT_COMPONENTS {
            let on_buttons_change = comp.on_buttons_change;
            on_buttons_change(old.buttons, new.buttons);
        }
    }
    if old.cubes != new.cubes {
        for comp in CURRENT_COMPONENTS {
            let on_cubes_change = comp.on_cubes_change;
            on_cubes_change(old.cubes, new.cubes);
        }
    }
    if old.platforms != new.platforms {
        for comp in CURRENT_COMPONENTS {
            let on_platforms_change = comp.on_platforms_change;
            on_platforms_change(old.platforms, new.platforms);
        }
    }
}

fn element_pressed(index: ElementIndex) {
    for comp in CURRENT_COMPONENTS {
        let on_element_pressed = comp.on_element_pressed;
        on_element_pressed(index);
    }
}
fn element_released(index: ElementIndex) {
    for comp in CURRENT_COMPONENTS {
        let on_element_released = comp.on_element_released;
        on_element_released(index);
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
