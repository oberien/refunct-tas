include "settings.re";
include "keys.re";
include "component.re"
include "ui.re";
include "teleport.re";
include "randomizer.re";
include "newgame.re";
include "practice.re";
include "util.re";
include "multiplayer.re";

static mut START_MENU_TEXT = Text { text: "Press 'm' for menu." };
static START_MENU = Ui {
    name: START_MENU_TEXT,
    elements: List::new(),
    on_key_down: Option::Some(fn(key: KeyCode) {
        if key == KEY_M {
            enter_ui(BASE_MENU);
        }
    }),
    on_draw: Option::Some(fn() {
        let mut text = "Press 'm' for menu.";
        let draw_hud = CURRENT_COMPONENT.draw_hud;
        text = draw_hud(text);
        if SETTINGS.show_character_stats {
            let loc = Tas::get_location();
            let vel = Tas::get_velocity();
            let rot = Tas::get_rotation();
            let acc = Tas::get_acceleration();
            let velxy = vel.x*vel.x + vel.y*vel.y;
            let velxy = velxy.sqrt();
            let velxyz = vel.x*vel.x + vel.y*vel.y + vel.z*vel.z;
            let velxyz = velxyz.sqrt();
            text = f"{text}
x: {loc.x:8.2}    y: {loc.y:8.2}    z: {loc.z:8.2}
velx {vel.x:8.2}    vely: {vel.y:8.2}    velz: {vel.z:8.2}
velxy: {velxy:8.2}
velxyz: {velxyz:8.2}
accx {acc.x:8.2}    accy: {acc.y:8.2}    accz: {acc.z:8.2}
pitch {rot.pitch:8.2}    yaw: {rot.yaw:8.2}    roll: {rot.roll:8.2}";
        }
        if SETTINGS.show_game_stats {
            text = f"{text}
Level: {GAME_STATS.current_level} (Total: {GAME_STATS.total_levels})
Buttons: {GAME_STATS.current_buttons} (Total: {GAME_STATS.total_buttons})
Cubes: {GAME_STATS.current_cubes} (Total: {GAME_STATS.total_cubes})
Platforms: {GAME_STATS.current_platforms} (Total: {GAME_STATS.total_platforms})
Resets: {GAME_STATS.total_resets} | Any%: {GAME_STATS.total_runs_completed} | 100%: {GAME_STATS.total_100_runs_completed} | All Platforms: {GAME_STATS.total_all_platforms_runs_completed} | All Cubes: {GAME_STATS.total_all_cubes_runs_completed} | Lowest #Platforms: {GAME_STATS.fewest_platform_run}";
        }
        START_MENU_TEXT.text = text;
    }),
    selected: 0,
};
static BASE_MENU = Ui::new("Menu:", List::of(
    UiElement::Button(UiButton {
        label: Text { text: "Practice" },
        onclick: fn(label: Text) { enter_ui(PRACTICE_MENU); },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Randomizer" },
        onclick: fn(label: Text) { enter_ui(RANDOMIZER_MENU); },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "New Game Actions" },
        onclick: fn(label: Text) { enter_ui(NEW_GAME_ACTIONS_MENU); },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Multiplayer" },
        onclick: fn(label: Text) { enter_ui(MULTIPLAYER_MENU); },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Util" },
        onclick: fn(label: Text) { enter_ui(UTIL_MENU); },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Settings" },
        onclick: fn(label: Text) { enter_ui(SETTINGS_MENU); },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Back" },
        onclick: fn(label: Text) { leave_ui() },
    }),
));
static PRACTICE_MENU = Ui::new("Practice:", {
    let mut buttons = List::of(
        UiElement::Button(UiButton {
            label: Text { text: "Nothing" },
            onclick: fn(label: Text) {
                set_current_component(NOOP_COMPONENT);
                leave_ui();
            },
        }),
    );
    for practice in PRACTICE_POINTS {
        buttons.push(UiElement::Button(UiButton {
            label: Text { text: practice.name },
            onclick: fn(label: Text) {
                for practice in PRACTICE_POINTS {
                    if practice.name == label.text {
                        CURRENT_PRACTICE = practice;
                        set_current_component(PRACTICE_COMPONENT);
                        break;
                    }
                }
                leave_ui();
            },
        }));
    }
    buttons
});
static mut RANDOMIZER_DIFFICULTY = 0;
static mut RANDOMIZER_NEW_GAME_NEW_SEED = 0;
static mut RANDOMIZER_SET_SEED_LABEL = Text { text: "Set Seed" };
static mut RANDOMIZER_SET_SEQUENCE_LABEL = Text { text: "Set Sequence" };
static RANDOMIZER_MENU = Ui::new("Randomizer:", List::of(
    UiElement::Button(UiButton {
        label: Text { text: "Disable" },
        onclick: fn(label: Text) {
            set_current_component(NOOP_COMPONENT);
            leave_ui();
        },
    }),
    UiElement::Chooser(Chooser {
        label: Text { text: "Difficulty" },
        options: List::of(
            Text { text: "Beginner" },
            Text { text: "Intermediate" },
            Text { text: "Advanced" },
        ),
        selected: RANDOMIZER_DIFFICULTY,
        onchange: fn(index: int) {
            RANDOMIZER_DIFFICULTY = index;
        },
    }),
    UiElement::Chooser(Chooser {
        label: Text { text: "New Seed when starting New Game" },
        options: List::of(
            Text { text: "Auto (On for Random Seed / Off for Set Seed)" },
            Text { text: "On" },
            Text { text: "Off" },
        ),
        selected: RANDOMIZER_NEW_GAME_NEW_SEED,
        onchange: fn(index: int) {
            RANDOMIZER_NEW_GAME_NEW_SEED = index;
            RANDOMIZER_STATE.new_game_new_seed = convert_new_game_new_seed(RANDOMIZER_NEW_GAME_NEW_SEED);
        },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Random Seed" },
        onclick: fn(label: Text) {
            randomizer_random_seed(randomizer_convert_difficulty(RANDOMIZER_DIFFICULTY));
            set_current_component(RANDOMIZER_COMPONENT);
            leave_ui();
        },
    }),
    UiElement::Input(Input {
        label: RANDOMIZER_SET_SEED_LABEL,
        input: "",
        onclick: fn(input: string) {
            match randomizer_parse_seed(input) {
                Result::Err(msg) => RANDOMIZER_SET_SEED_LABEL.text = f"Set Seed ({msg})",
                Result::Ok(seed) => {
                    randomizer_set_seed(seed, randomizer_convert_difficulty(RANDOMIZER_DIFFICULTY));
                    set_current_component(RANDOMIZER_COMPONENT);
                    leave_ui();
                },
            }
        },
        onchange: fn(input: string) {},
    }),
    UiElement::Input(Input {
        label: RANDOMIZER_SET_SEQUENCE_LABEL,
        input: "",
        onclick: fn(input: string) {
            match randomizer_parse_sequence(input) {
                Result::Err(msg) => RANDOMIZER_SET_SEQUENCE_LABEL.text = f"Set Sequence ({msg})",
                Result::Ok(seq) => {
                    randomizer_set_sequence(seq);
                    set_current_component(RANDOMIZER_COMPONENT);
                    leave_ui();
                },
            }
        },
        onchange: fn(input: string) {},
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Copy previous Seed to Clipboard" },
        onclick: fn(label: Text) { randomizer_copy_prev_seed() },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Copy previous Sequence to Clipboard" },
        onclick: fn(label: Text) { randomizer_copy_prev_sequence() },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Back" },
        onclick: fn(label: Text) { leave_ui() },
    }),
));
static NEW_GAME_ACTIONS_MENU = Ui::new("New Game Actions:", List::of(
    UiElement::Button(UiButton {
        label: Text { text: "Nothing" },
        onclick: fn(label: Text) { set_current_component(NOOP_COMPONENT); leave_ui(); },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "100%" },
        onclick: fn(label: Text) { set_current_component(NEW_GAME_100_PERCENT_COMPONENT); leave_ui(); },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "All Buttons" },
        onclick: fn(label: Text) { set_current_component(NEW_GAME_ALL_BUTTONS_COMPONENT); leave_ui(); },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "NGG" },
        onclick: fn(label: Text) { set_current_component(NEW_GAME_NGG_COMPONENT); leave_ui(); },
    }),
));
static MULTIPLAYER_MENU = Ui::new("Multiplayer:", List::of(
    UiElement::Input(Input {
        label: Text { text: "Name" },
        input: SETTINGS.multiplayer_name,
        onclick: fn(input: string) {},
        onchange: fn(input: string) {
            SETTINGS.set_multiplayer_name(input);
        },
    }),
    UiElement::Input(Input {
        label: Text { text: "Join/Create Room" },
        input: "",
        onclick: fn(input: string) {
            if input.len_utf8() == 0 {
                return;
            }
            multiplayer_join_room(input);
            set_current_component(MULTIPLAYER_COMPONENT);
            leave_ui();
        },
        onchange: fn(input: string) {},
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Disconnect" },
        onclick: fn(label: Text) {
            set_current_component(NOOP_COMPONENT);
            leave_ui();
        },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Back" },
        onclick: fn(label: Text) { leave_ui(); },
    }),
));
static UTIL_MENU = Ui::new("Util:", List::of(
    UiElement::Input(Input {
        label: Text { text: "Teleport (x,y,z)" },
        input: "",
        onclick: fn(input: string) {
            let xyz = input.split(",");
            let x = match xyz.get(0) {
                Option::Some(x) => match x.parse_float() {
                    Result::Ok(x) => x,
                    Result::Err(e) => return,
                },
                Option::None => return,
            };
            let y = match xyz.get(1) {
                Option::Some(y) => match y.parse_float() {
                    Result::Ok(y) => y,
                    Result::Err(e) => return,
                },
                Option::None => return,
            };
            let z = match xyz.get(2) {
                Option::Some(z) => match z.parse_float() {
                    Result::Ok(z) => z,
                    Result::Err(e) => return,
                },
                Option::None => return,
            };
            let loc = Location { x: x, y: y, z: z };
            Tas::set_location(loc);
        },
        onchange: fn(input: string) {},
    }),
    UiElement::Input(Input {
        label: Text { text: "Set Roll" },
        input: "0",
        onclick: fn(input: string) {},
        onchange: fn(input: string) {
            let roll = match input.parse_float() {
                Result::Ok(roll) => roll,
                Result::Err(e) => return,
            };
            let mut rot = Tas::get_rotation();
            rot.roll = roll;
            Tas::set_rotation(rot);
        },
    }),
    UiElement::Input(Input {
        label: Text { text: "Windscreen Wipers (s/wipe)" },
        input: "2",
        onclick: fn(input: string) {
            let seconds_per_wipe = match input.parse_float() {
                Result::Ok(seconds_per_wipe) => seconds_per_wipe,
                Result::Err(e) => return,
            };
            start_windscreen_wipers(seconds_per_wipe);
            set_current_component(WINDSCREEN_WIPERS_COMPONENT);
        },
        onchange: fn(input: string) {},
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Frame-Step Mode" },
        onclick: fn(label: Text) {
            set_current_component(FRAME_STEP_COMPONENT);
            leave_ui();
            leave_ui();
            Tas::step();
        }
    }),
    UiElement::Input(Input {
        label: Text { text: "Spawn Pawn (x,y,z)" },
        input: "",
        onclick: fn(input: string) {
            static mut UTIL_PAWNS = List::new();
            let xyz = input.split(",");
            let x = match xyz.get(0) {
                Option::Some(x) => match x.parse_float() {
                    Result::Ok(x) => x,
                    Result::Err(e) => return,
                },
                Option::None => return,
            };
            let y = match xyz.get(1) {
                Option::Some(y) => match y.parse_float() {
                    Result::Ok(y) => y,
                    Result::Err(e) => return,
                },
                Option::None => return,
            };
            let z = match xyz.get(2) {
                Option::Some(z) => match z.parse_float() {
                    Result::Ok(z) => z,
                    Result::Err(e) => return,
                },
                Option::None => return,
            };
            let id = Tas::spawn_pawn(Location { x: 0., y: 0., z: 0. }, Rotation { pitch: 0., yaw: 0., roll: 0. });
            Tas::move_pawn(id, Location { x: x, y: y, z: z });
            UTIL_PAWNS.push(id);
        },
        onchange: fn(input: string) {},
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Delete all pawns" },
        onclick: fn(label: Text) {
            loop {
                match UTIL_PAWNS.swap_remove(0) {
                    Option::Some(id) => Tas::destroy_pawn(id),
                    Option::None => break,
                }
            }
        }
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Back" },
        onclick: fn(label: Text) { leave_ui() },
    }),
));
static mut UI_SCALE_TEXT = Text { text: f"{SETTINGS.ui_scale}" };
static mut SHOW_CHARACTER_STATS_BUTTON_TEXT = Text { text: f"Show Character Stats: {SETTINGS.show_character_stats}" };
static mut SHOW_GAME_STATS_BUTTON_TEXT = Text { text: f"Show Game Stats: {SETTINGS.show_game_stats}" };
static SETTINGS_MENU = Ui::new("Settings:", List::of(
    UiElement::Slider(Slider {
        label: Text { text: "UI Scale" },
        content: UI_SCALE_TEXT,
        onleft: fn() {
            SETTINGS.decrease_ui_scale();
            UI_SCALE_TEXT.text = f"{SETTINGS.ui_scale}";
        },
        onright: fn() {
            SETTINGS.increase_ui_scale();
            UI_SCALE_TEXT.text = f"{SETTINGS.ui_scale}";
        },
    }),
    UiElement::Button(UiButton {
        label: SHOW_CHARACTER_STATS_BUTTON_TEXT,
        onclick: fn(label: Text) {
            SETTINGS.toggle_show_character_stats();
            SHOW_CHARACTER_STATS_BUTTON_TEXT.text = f"Show Character Stats: {SETTINGS.show_character_stats}";
        },
    }),
    UiElement::Button(UiButton {
        label: SHOW_GAME_STATS_BUTTON_TEXT,
        onclick: fn(label: Text) {
            SETTINGS.toggle_show_game_stats();
            SHOW_GAME_STATS_BUTTON_TEXT.text = f"Show Game Stats: {SETTINGS.show_game_stats}";
        },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Reset Game Stats" },
        onclick: fn(label: Text) { GAME_STATS.reset() },
    }),
    UiElement::Button(UiButton {
        label: Text { text: "Back" },
        onclick: fn(label: Text) { leave_ui() },
    }),
));

enter_ui(START_MENU);

loop {
    let tick_fn = CURRENT_COMPONENT.tick_fn;
    match tick_fn() {
        Step::Tick => (),
        Step::NewGame => {
            let on_new_game = CURRENT_COMPONENT.on_new_game;
            on_new_game();
        },
        Step::Yield => {
            let on_yield = CURRENT_COMPONENT.on_yield;
            on_yield();
            continue
        }
    }
    let on_tick = CURRENT_COMPONENT.on_tick;
    on_tick();
}
