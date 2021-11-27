include "keys.re";
include "ui.re";
include "teleport.re";
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
        if !SHOW_STATS {
            START_MENU_TEXT.text = f"Press 'm' for menu.";
        } else {
            let loc = Tas::get_location();
            let vel = Tas::get_velocity();
            let rot = Tas::get_rotation();
            let acc = Tas::get_acceleration();
            let velxy = vel.x*vel.x + vel.y*vel.y;
            let velxy = velxy.sqrt();
            let velxyz = vel.x*vel.x + vel.y*vel.y + vel.z*vel.z;
            let velxyz = velxyz.sqrt();
            START_MENU_TEXT.text = f"Press 'm' for menu.
x: {loc.x:8.2}    y: {loc.y:8.2}    z: {loc.z:8.2}
velx {vel.x:8.2}    vely: {vel.y:8.2}    velz: {vel.z:8.2}
velxy: {velxy:8.2}
velxyz: {velxyz:8.2}
accx {acc.x:8.2}    accy: {acc.y:8.2}    accz: {acc.z:8.2}
pitch {rot.pitch:8.2}    yaw: {rot.yaw:8.2}    roll: {rot.roll:8.2}
";
        }
    }),
    selected: 0,
};
static BASE_MENU = Ui::new("Menu:", List::of(
    UiElement::Button(Button {
        label: Text { text: "Practice" },
        onclick: fn(label: Text) { enter_ui(PRACTICE_MENU); },
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
                new_game_nothing();
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
                        new_game_practice(practice);
                    }
                }
                leave_ui();
            }
        }));
    }
    buttons
});
static NEW_GAME_ACTIONS_MENU = Ui::new("New Game Actions:", List::of(
    UiElement::Button(Button {
        label: Text { text: "Nothing" },
        onclick: fn(label: Text) { new_game_nothing(); leave_ui(); },
    }),
    UiElement::Button(Button {
        label: Text { text: "100%" },
        onclick: fn(label: Text) { new_game_100_percent(); leave_ui(); },
    }),
    UiElement::Button(Button {
        label: Text { text: "All Buttons" },
        onclick: fn(label: Text) { new_game_level_reset(29, 0); leave_ui(); },
    }),
    UiElement::Button(Button {
        label: Text { text: "NGG" },
        onclick: fn(label: Text) { new_game_level_reset(1, 1); leave_ui(); },
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
            UI_SCALE_TEXT.text = f"{UI_SCALE}";
        },
        onright: fn() {
            UI_SCALE = UI_SCALE + 0.5;
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
    NEW_GAME_FUNCTION();
}
fn tcp_joined(id: int, x: float, y: float, z: float) {}
fn tcp_left(id: int) {}
fn tcp_moved(id: int, x: float, y: float, z: float) {}
