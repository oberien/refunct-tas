static mut FLYING_UP_DOWN_VELOCITY_LABEL = Text { text: "Up/Down Flying Velocity" };
static mut FLYING_FORWARD_BACKWARD_VELOCITY_LABEL = Text { text: "Forward/Backward Flying Velocity" };

static mut MOVEMENT_STATE = MovementState {
    is_flying_down: false,
    is_flying_up: false,
    enable_fly: false,
    fly_down_up_velocity: 300.,
};

struct MovementState {
    is_flying_down: bool,
    is_flying_up: bool,
    enable_fly: bool,
    fly_down_up_velocity: float,
}

fn create_movement_menu() -> Ui {
    let mut movement_mode = Number { number: 1 };
    Ui::new("Movement:", List::of(
        UiElement::Button(UiButton {
            label: Text { text: "Enable Flying" },
            onclick: fn(label: Text) {
                MOVEMENT_STATE.enable_fly = true;
                leave_ui();
                leave_ui();
                leave_ui();
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Disable Flying" },
            onclick: fn(label: Text) {
                MOVEMENT_STATE.enable_fly = false;
                Tas::set_movement_mode(1);
            },
        }),
        UiElement::Chooser(Chooser {
            label: Text { text: "Movement Mode" },
            options: List::of(
                Text { text: "None" },
                Text { text: "Walking" },
                Text { text: "Navwalking" },
                Text { text: "Falling" },
                Text { text: "Swimming" },
                Text { text: "Flying" },
            ),
            selected: movement_mode.number,
            onchange: fn(index: int) { movement_mode.number = index; },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Set Movement Mode" },
            onclick: fn(label: Text) { Tas::set_movement_mode(movement_mode.number); },
        }),
        UiElement::Input(Input {
            label: FLYING_UP_DOWN_VELOCITY_LABEL,
            input: "300",
            onclick: fn(input: string) {
                let val = input.parse_float();
                match val {
                    Result::Ok(val) => { MOVEMENT_STATE.fly_down_up_velocity = val; },
                    Result::Err(e) => { FLYING_UP_DOWN_VELOCITY_LABEL.text = "Up/Down Flying Velocity [error: invalid input]"; },
                }
            },
            onchange: fn(input: string) {},
        }),
        UiElement::Input(Input {
            label: FLYING_FORWARD_BACKWARD_VELOCITY_LABEL,
            input: "600",
            onclick: fn(input: string) {
                let val = input.parse_float();
                match val {
                    Result::Ok(val) => { Tas::set_forward_backward_fly_speed(val); },
                    Result::Err(e) => { FLYING_FORWARD_BACKWARD_VELOCITY_LABEL.text = "Forward/Backward Flying Velocity [error: invalid input]"; },
                }
            },
            onchange: fn(input: string) {},
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui() },
        }),
    ))
}

static MOVEMENT_COMPONENT = Component {
    id: MOVEMENT_COMPONENT_ID,
    conflicts_with: List::of(MOVEMENT_COMPONENT_ID),
    draw_hud: fn(text: string) -> string { text },
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {
        if MOVEMENT_STATE.enable_fly {
            Tas::set_movement_mode(5);
            if MOVEMENT_STATE.is_flying_up && !MOVEMENT_STATE.is_flying_down {
                if Tas::get_movement_mode() == 5 {
                    let vel = Tas::get_velocity();
                    Tas::set_velocity(Velocity { x: vel.x, y: vel.y, z: MOVEMENT_STATE.fly_down_up_velocity });
                }
            } else if !MOVEMENT_STATE.is_flying_up && MOVEMENT_STATE.is_flying_down {
                if Tas::get_movement_mode() == 5 {
                    let vel = Tas::get_velocity();
                    Tas::set_velocity(Velocity { x: vel.x, y: vel.y, z: (MOVEMENT_STATE.fly_down_up_velocity * -1.) });
                }
            } else if !MOVEMENT_STATE.is_flying_up && !MOVEMENT_STATE.is_flying_down {
                if Tas::get_movement_mode() == 5 {
                    let vel = Tas::get_velocity();
                    Tas::set_velocity(Velocity { x: vel.x, y: vel.y, z: 0. });
                }
            }
        }
    },
    on_yield: fn() {},
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_key_down: fn(key_code: KeyCode, is_repeat: bool) {
        let key = key_code.to_small();
        if key == KEY_LEFT_SHIFT.to_small() {
            MOVEMENT_STATE.is_flying_up = false;
            MOVEMENT_STATE.is_flying_down = true;
        } else if key == KEY_SPACE.to_small() {
            MOVEMENT_STATE.is_flying_down = false;
            MOVEMENT_STATE.is_flying_up = true;
        }
    },
    on_key_up: fn(key_code: KeyCode) {
        let key = key_code.to_small();
        if key == KEY_LEFT_SHIFT.to_small() {
            MOVEMENT_STATE.is_flying_down = false;
        } else if key == KEY_SPACE.to_small() {
            MOVEMENT_STATE.is_flying_up = false;
        }
    },
    on_mouse_move: fn(x: int, y: int) {},
    on_component_exit: fn() {},
};
