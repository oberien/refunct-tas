enum FlyState {
    None,
    Up,
    Down,
}

static mut FLYING_UP_DOWN_VELOCITY_LABEL = Text { text: "Up/Down Flying Velocity" };
static mut FLYING_FORWARD_BACKWARD_VELOCITY_LABEL = Text { text: "Forward/Backward Flying Velocity" };

static mut MOVEMENT_STATE = MovementState {
    enable_fly: false,
    fly_down_up_velocity: 300.,
    fly_state: FlyState::None,
};

struct MovementState {
    enable_fly: bool,
    fly_down_up_velocity: float,
    fly_state: FlyState,
}

fn create_movement_menu() -> Ui {
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
        UiElement::Input(Input {
            label: FLYING_UP_DOWN_VELOCITY_LABEL,
            input: f"{SETTINGS.flying_up_down_velocity}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(val) => {
                        SETTINGS.flying_up_down_velocity = val;
                        SETTINGS.store();
                        FLYING_UP_DOWN_VELOCITY_LABEL.text = "Up/Down Flying Velocity";
                    },
                    Result::Err(e) => { FLYING_UP_DOWN_VELOCITY_LABEL.text = "Up/Down Flying Velocity [error: invalid input]"; },
                }
            },
        }),
        UiElement::Input(Input {
            label: FLYING_FORWARD_BACKWARD_VELOCITY_LABEL,
            input: f"{SETTINGS.flying_forward_backward_velocity}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(val) => {
                        Tas::set_max_fly_speed(val);
                        SETTINGS.flying_forward_backward_velocity = val;
                        SETTINGS.store();
                        FLYING_FORWARD_BACKWARD_VELOCITY_LABEL.text = "Forward/Backward Flying Velocity";
                    },
                    Result::Err(e) => { FLYING_FORWARD_BACKWARD_VELOCITY_LABEL.text = "Forward/Backward Flying Velocity [error: invalid input]"; },
                }
            },
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
    draw_hud_text: fn(text: string) -> string {
        match MOVEMENT_STATE.enable_fly {
            false => f"{text}\nFlying: <f> enable flying",
            true => f"{text}\nFlying: <f> disable flying",
        }
    },
    draw_hud_always: fn() {},
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {
        if MOVEMENT_STATE.enable_fly {
            Tas::set_movement_mode(5);
            match MOVEMENT_STATE.fly_state {
                FlyState::None => {
                    let vel = Tas::get_velocity();
                    Tas::set_velocity(Velocity { x: vel.x, y: vel.y, z: 0. });
                },
                FlyState::Up => {
                    let vel = Tas::get_velocity();
                    Tas::set_velocity(Velocity { x: vel.x, y: vel.y, z: SETTINGS.flying_up_down_velocity });
                },
                FlyState::Down => {
                    let vel = Tas::get_velocity();
                    Tas::set_velocity(Velocity { x: vel.x, y: vel.y, z: (SETTINGS.flying_up_down_velocity * -1.) });
                },
            }
        }
    },
    on_yield: fn() {},
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_element_pressed: fn(index: ElementIndex) {},
    on_element_released: fn(index: ElementIndex) {},
    on_key_down: fn(key_code: KeyCode, is_repeat: bool) {
        let key = key_code.to_small();
        if key == KEY_F.to_small() {
            match MOVEMENT_STATE.enable_fly {
                false => MOVEMENT_STATE.enable_fly = true,
                true => {
                    MOVEMENT_STATE.enable_fly = false;
                    Tas::set_movement_mode(1);
                }
            }
        }
    },
    on_key_down_always: fn(key: KeyCode, is_repeat: bool) {
        let key = key.to_small();
        if key == KEY_LEFT_SHIFT.to_small() {
            MOVEMENT_STATE.fly_state = FlyState::Down;
        } else if key == KEY_SPACE.to_small() {
            MOVEMENT_STATE.fly_state = FlyState::Up;
        }
    },
    on_key_up: fn(key_code: KeyCode) {},
    on_key_up_always: fn(key: KeyCode) {
        let key = key.to_small();
        if key == KEY_LEFT_SHIFT.to_small() && MOVEMENT_STATE.fly_state == FlyState::Down {
            MOVEMENT_STATE.fly_state = FlyState::None;
        } else if key == KEY_SPACE.to_small() && MOVEMENT_STATE.fly_state == FlyState::Up {
            MOVEMENT_STATE.fly_state = FlyState::None;
        }
    },
    on_mouse_move: fn(x: int, y: int) {},
    on_component_exit: fn() { MOVEMENT_STATE.enable_fly = false; },
    on_resolution_change: fn() {},
    on_menu_open: fn() {},
};
