include "keys.re";
include "ui.re";
include "teleport.re";
include "newgame.re";

static START_MENU = Ui {
    name: "Press 'm' for menu.",
    elements: List::new(),
    on_key_down: Option::Some(fn(key: KeyCode) {
        if key == KEY_M {
            enter_ui(BASE_MENU);
        }
    }),
    selected: 0,
};
static BASE_MENU = Ui::new("Menu:", List::of(
    UiElement::Button(Button {
        label: "New Game Actions",
        onclick: fn() { enter_ui(NEW_GAME_ACTIONS_MENU); },
    }),
    UiElement::Button(Button {
        label: "Back",
        onclick: leave_ui,
    }),
));
static NEW_GAME_ACTIONS_MENU = Ui::new("New Game Actions:", List::of(
    UiElement::Button(Button {
        label: "Nothing",
        onclick: fn() { new_game_nothing(); leave_ui(); },
    }),
    UiElement::Button(Button {
        label: "100%",
        onclick: fn() { new_game_100_percent(); leave_ui(); },
    }),
    UiElement::Button(Button {
        label: "All Buttons",
        onclick: fn() { new_game_all_buttons(); leave_ui(); },
    }),
    UiElement::Button(Button {
        label: "NGG",
        onclick: fn() { new_game_ngg(); leave_ui(); },
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
