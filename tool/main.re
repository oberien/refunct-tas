include "keys.re";
include "ui.re";
include "component.re"
include "teleport.re";
include "randomizer.re";
include "newgame.re";
include "practice.re";

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
        if SHOW_STATS {
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
        START_MENU_TEXT.text = text;
    }),
    selected: 0,
};
static BASE_MENU = Ui::new("Menu:", List::of(
    UiElement::Button(Button {
        label: Text { text: "Practice" },
        onclick: fn(label: Text) { enter_ui(PRACTICE_MENU); },
    }),
    UiElement::Button(Button {
        label: Text { text: "Randomizer" },
        onclick: fn(label: Text) { enter_ui(RANDOMIZER_MENU); },
    }),
    UiElement::Button(Button {
        label: Text { text: "New Game Actions" },
        onclick: fn(label: Text) { enter_ui(NEW_GAME_ACTIONS_MENU); },
    }),
    UiElement::Button(Button {
        label: Text { text: "Settings" },
        onclick: fn(label: Text) { enter_ui(SETTINGS_MENU); },
    }),
    UiElement::Button(Button {
        label: Text { text: "Back" },
        onclick: fn(label: Text) { leave_ui() },
    }),
));
static PRACTICE_MENU = Ui::new("Practice:", {
    let mut buttons = List::of(
        UiElement::Button(Button {
            label: Text { text: "Nothing" },
            onclick: fn(label: Text) {
                CURRENT_COMPONENT = NOOP_COMPONENT;
                leave_ui();
            },
        }),
    );
    for practice in PRACTICE_POINTS {
        buttons.push(UiElement::Button(Button {
            label: Text { text: practice.name },
            onclick: fn(label: Text) {
                for practice in PRACTICE_POINTS {
                    if practice.name == label.text {
                        CURRENT_PRACTICE = practice;
                        CURRENT_COMPONENT = PRACTICE_COMPONENT;
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
    UiElement::Button(Button {
        label: Text { text: "Disable" },
        onclick: fn(label: Text) {
            CURRENT_COMPONENT = NOOP_COMPONENT;
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
    UiElement::Button(Button {
        label: Text { text: "Random Seed" },
        onclick: fn(label: Text) {
            randomizer_random_seed(randomizer_convert_difficulty(RANDOMIZER_DIFFICULTY));
            CURRENT_COMPONENT = RANDOMIZER_COMPONENT;
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
                    CURRENT_COMPONENT = RANDOMIZER_COMPONENT;
                    leave_ui();
                },
            }
        }
    }),
    UiElement::Input(Input {
        label: RANDOMIZER_SET_SEQUENCE_LABEL,
        input: "",
        onclick: fn(input: string) {
            match randomizer_parse_sequence(input) {
                Result::Err(msg) => RANDOMIZER_SET_SEQUENCE_LABEL.text = f"Set Sequence ({msg})",
                Result::Ok(seq) => {
                    randomizer_set_sequence(seq);
                    CURRENT_COMPONENT = RANDOMIZER_COMPONENT;
                    leave_ui();
                },
            }
        }
    }),
    UiElement::Button(Button {
        label: Text { text: "Copy previous Seed to Clipboard" },
        onclick: fn(label: Text) { randomizer_copy_prev_seed() },
    }),
    UiElement::Button(Button {
        label: Text { text: "Copy previous Sequence to Clipboard" },
        onclick: fn(label: Text) { randomizer_copy_prev_sequence() },
    }),
    UiElement::Button(Button {
        label: Text { text: "Back" },
        onclick: fn(label: Text) { leave_ui() },
    }),
));
static NEW_GAME_ACTIONS_MENU = Ui::new("New Game Actions:", List::of(
    UiElement::Button(Button {
        label: Text { text: "Nothing" },
        onclick: fn(label: Text) { CURRENT_COMPONENT = NOOP_COMPONENT; leave_ui(); },
    }),
    UiElement::Button(Button {
        label: Text { text: "100%" },
        onclick: fn(label: Text) { CURRENT_COMPONENT = NEW_GAME_100_PERCENT_COMPONENT; leave_ui(); },
    }),
    UiElement::Button(Button {
        label: Text { text: "All Buttons" },
        onclick: fn(label: Text) { CURRENT_COMPONENT = NEW_GAME_ALL_BUTTONS_COMPONENT; leave_ui(); },
    }),
    UiElement::Button(Button {
        label: Text { text: "NGG" },
        onclick: fn(label: Text) { CURRENT_COMPONENT = NEW_GAME_NGG_COMPONENT; leave_ui(); },
    }),
));
static mut UI_SCALE_TEXT = Text { text: f"{UI_SCALE}" };
static mut SHOW_STATS = false;
static mut SHOW_STATS_BUTTON_TEXT = Text { text: f"Show Stats: {SHOW_STATS}" };
static SETTINGS_MENU = Ui::new("Settings:", List::of(
    UiElement::Slider(Slider {
        label: Text { text: "UI Scale" },
        content: UI_SCALE_TEXT,
        onleft: fn() {
            UI_SCALE = UI_SCALE - 0.5;
            UI_SCALE = UI_SCALE.max(0.5);
            UI_SCALE_TEXT.text = f"{UI_SCALE}";
        },
        onright: fn() {
            UI_SCALE = UI_SCALE + 0.5;
            UI_SCALE = UI_SCALE.min(10.);
            UI_SCALE_TEXT.text = f"{UI_SCALE}";
        },
    }),
    UiElement::Button(Button {
        label: SHOW_STATS_BUTTON_TEXT,
        onclick: fn(label: Text) {
            SHOW_STATS = !SHOW_STATS;
            SHOW_STATS_BUTTON_TEXT.text = f"Show Stats: {SHOW_STATS}";
        },
    }),
    UiElement::Button(Button {
        label: Text { text: "Back" },
        onclick: fn(label: Text) { leave_ui() },
    }),
));

enter_ui(START_MENU);

while true {
    Tas::wait_for_new_game();
    let on_new_game = CURRENT_COMPONENT.on_new_game;
    on_new_game();
}
fn tcp_joined(id: int, x: float, y: float, z: float) {}
fn tcp_left(id: int) {}
fn tcp_moved(id: int, x: float, y: float, z: float) {}

