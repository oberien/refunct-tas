static mut PRACTICE_STATE = PracticeState {
    mode: PracticeMode::Practicing,
};

struct PracticeState {
    mode: PracticeMode
}

fn create_practice_menu() -> Ui {
    let mut buttons = List::of(
        UiElement::Button(UiButton {
            label: Text { text: "Default Practice States" },
            onclick: fn(label: Text) {
                enter_ui(create_default_practice_menu());
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Custom Practice States" },
            onclick: fn(label: Text) {
                enter_ui(create_custom_practice_menu());
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui() },
        }),
    );
    Ui::new("Practice:", buttons)
}

fn create_default_practice_menu() -> Ui {
    let mut buttons = List::of(
        UiElement::Button(UiButton {
            label: Text { text: "Nothing" },
            onclick: fn(label: Text) {
                remove_component(PRACTICE_COMPONENT);
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
                        add_component(PRACTICE_COMPONENT);
                        break;
                    }
                }
                leave_ui();
            },
        }));
    }
    Ui::new("Default Practice:", buttons)
}

fn create_custom_practice_menu() -> Ui {
    let practice_list = Tas::list_practice_states();
    let mut buttons = List::of(
        UiElement::Button(UiButton {
            label: Text { text: "Save Custom Practice State" },
            onclick: fn(label: Text) {
                let practice_list = Tas::list_practice_states();
                enter_ui(Ui::new_filechooser("Save Custom Practice State", practice_list, fn(input: string) {
                    if input.len_utf8() == 0 {
                        return;
                    }
                    let level_state = Tas::get_level_state();
                    Tas::save_practice_state(input, Practice {
                        name: input,
                        cluster: level_state.level,
                        button: level_state.buttons,
                        location: Tas::get_location(),
                        rotation: Tas::get_rotation(),
                        velocity: Tas::get_velocity()
                    });
                    leave_ui();
                    leave_ui();
                }));
            },
        }),
       UiElement::Button(UiButton {
            label: Text { text: "Edit Custom Practice State" },
            onclick: fn(label: Text) {
                let practice_list = Tas::list_practice_states();
                enter_ui(Ui::new_filechooser("Edit Custom Practice State", practice_list, fn(input: string) {
                    if input.len_utf8() == 0 {
                        return;
                    }
                    enter_ui(create_edit_practice_state_ui(Tas::load_practice_state(input)));
                }));
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Load Custom Practice State" },
            onclick: fn(label: Text) {
                let practice_list = Tas::list_practice_states();
                enter_ui(Ui::new_filechooser("Load Custom Practice State", practice_list, fn(input: string) {
                    if !practice_list.contains(input) {
                        return;
                    }
                    CURRENT_PRACTICE = Tas::load_practice_state(input);
                    add_component(PRACTICE_COMPONENT);
                    leave_ui();
                    leave_ui();
                    leave_ui();
                }));
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui() },
        }),
    );
    Ui::new("Custom Practice:", buttons)
}

fn create_edit_practice_state_ui(mut practice: Practice) -> Ui {
    let submit = fn() {
        let selected = UI_STACK.last().unwrap().selected;
        Tas::save_practice_state(practice.name, practice);
        leave_ui();
        CURRENT_PRACTICE = practice;
        enter_ui(create_edit_practice_state_ui(practice));
    };
    static mut PRACTICE_CLUSTER_LABEL = Text { text: "Cluster" };
    static mut PRACTICE_BUTTON_LABEL = Text { text: "Button" };
    static mut PRACTICE_X_LABEL = Text { text: "X" };
    static mut PRACTICE_Y_LABEL = Text { text: "Y" };
    static mut PRACTICE_Z_LABEL = Text { text: "Z" };
    static mut PRACTICE_PITCH_LABEL = Text { text: "Pitch" };
    static mut PRACTICE_YAW_LABEL = Text { text: "Yaw" };
    static mut PRACTICE_ROLL_LABEL = Text { text: "Roll" };
    static mut PRACTICE_VELX_LABEL = Text { text: "VelX" };
    static mut PRACTICE_VELY_LABEL = Text { text: "VelY" };
    static mut PRACTICE_VELZ_LABEL = Text { text: "VelZ" };
    Ui::new(f"Practice - Edit {practice.name}", List::of(
        UiElement::Button(UiButton {
            label: Text { text: "Set to player + game stats" },
            onclick: fn(label: Text) {
                let level_state = Tas::get_level_state();
                practice.cluster = level_state.level - 1;
                practice.button = level_state.buttons - 1;
                practice.location = Tas::get_location();
                practice.rotation = Tas::get_rotation();
                practice.velocity = Tas::get_velocity();
                submit();
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Set to player stats" },
            onclick: fn(label: Text) {
                practice.location = Tas::get_location();
                practice.rotation = Tas::get_rotation();
                practice.velocity = Tas::get_velocity();
                submit();
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Set to player location" },
            onclick: fn(label: Text) {
                practice.location = Tas::get_location();
                submit();
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Set to player rotation" },
            onclick: fn(label: Text) {
                practice.rotation = Tas::get_rotation();
                submit();
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Set to player velocity" },
            onclick: fn(label: Text) {
                practice.velocity = Tas::get_velocity();
                submit();
            },
        }),
        UiElement::Input(Input {
            label: Text { text: "Name" },
            input: practice.name,
            onclick: fn(input: string) {
                practice.name = input;
                CURRENT_PRACTICE = practice;
                Tas::save_practice_state(practice.name, practice);
            },
            onchange: fn(input: string) {},
        }),
        UiElement::IntInput(IntInput {
            label: PRACTICE_CLUSTER_LABEL,
            input: f"{practice.cluster}",
            onclick: fn(input: string) {
                CURRENT_PRACTICE = practice;
                Tas::save_practice_state(practice.name, practice);
            },
            onchange: fn(input: string) {
                PRACTICE_CLUSTER_LABEL.text = "Cluster";
                match input.parse_int() {
                    Result::Ok(num) => {
                        if num > 30 {
                            PRACTICE_CLUSTER_LABEL.text = f"Cluster (invalid value)";
                        } else {
                            practice.cluster = num;
                        }
                    },
                    Result::Err(e) => PRACTICE_CLUSTER_LABEL.text = f"Cluster (invalid value)",
                }
            },
        }),
        UiElement::IntInput(IntInput {
            label: PRACTICE_BUTTON_LABEL,
            input: f"{practice.button}",
            onclick: fn(input: string) {
                CURRENT_PRACTICE = practice;
                Tas::save_practice_state(practice.name, practice);
            },
            onchange: fn(input: string) {
                PRACTICE_BUTTON_LABEL.text = "Button";
                match input.parse_int() {
                    Result::Ok(num) => {
                        if num > 30 {
                            PRACTICE_BUTTON_LABEL.text = f"Button (invalid value)";
                        } else {
                            practice.button = num;
                        }
                    },
                    Result::Err(e) => PRACTICE_BUTTON_LABEL.text = f"Button (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: PRACTICE_X_LABEL,
            input: f"{practice.location.x:.2}",
            onclick: fn(input: string) {
                submit();
            },
            onchange: fn(input: string) {
                PRACTICE_X_LABEL.text = "X";
                match input.parse_float() {
                    Result::Ok(num) => practice.location.x = num,
                    Result::Err(e) => PRACTICE_X_LABEL.text = f"X (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: PRACTICE_Y_LABEL,
            input: f"{practice.location.y:.2}",
            onclick: fn(input: string) {
                submit();
            },
            onchange: fn(input: string) {
                PRACTICE_Y_LABEL.text = "Y";
                match input.parse_float() {
                    Result::Ok(num) => practice.location.y = num,
                    Result::Err(e) => PRACTICE_Y_LABEL.text = f"Y (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: PRACTICE_Z_LABEL,
            input: f"{practice.location.z:.2}",
            onclick: fn(input: string) {
                submit();
            },
            onchange: fn(input: string) {
                PRACTICE_Z_LABEL.text = "Z";
                match input.parse_float() {
                    Result::Ok(num) => practice.location.z = num,
                    Result::Err(e) => PRACTICE_Z_LABEL.text = f"Z (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: PRACTICE_PITCH_LABEL,
            input: f"{practice.rotation.pitch:.2}",
            onclick: fn(input: string) {
                submit();
            },
            onchange: fn(input: string) {
                PRACTICE_PITCH_LABEL.text = "Pitch";
                match input.parse_float() {
                    Result::Ok(num) => practice.rotation.pitch = num,
                    Result::Err(e) => PRACTICE_PITCH_LABEL.text = f"Pitch (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: PRACTICE_YAW_LABEL,
            input: f"{practice.rotation.yaw:.2}",
            onclick: fn(input: string) {
                submit();
            },
            onchange: fn(input: string) {
                PRACTICE_YAW_LABEL.text = "Yaw";
                match input.parse_float() {
                    Result::Ok(num) => practice.rotation.yaw = num,
                    Result::Err(e) => PRACTICE_YAW_LABEL.text = f"Yaw (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: PRACTICE_ROLL_LABEL,
            input: f"{practice.rotation.roll:.2}",
            onclick: fn(input: string) {
                submit();
            },
            onchange: fn(input: string) {
                PRACTICE_ROLL_LABEL.text = "Roll";
                match input.parse_float() {
                    Result::Ok(num) => practice.rotation.roll = num,
                    Result::Err(e) => PRACTICE_ROLL_LABEL.text = f"Roll (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: PRACTICE_VELX_LABEL,
            input: f"{practice.velocity.x:.2}",
            onclick: fn(input: string) {
                submit();
            },
            onchange: fn(input: string) {
                PRACTICE_VELX_LABEL.text = "VelX";
                match input.parse_float() {
                    Result::Ok(num) => practice.velocity.x = num,
                    Result::Err(e) => PRACTICE_VELX_LABEL.text = f"VelX (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: PRACTICE_VELY_LABEL,
            input: f"{practice.velocity.y:.2}",
            onclick: fn(input: string) {
                submit();
            },
            onchange: fn(input: string) {
                PRACTICE_VELY_LABEL.text = "VelY";
                match input.parse_float() {
                    Result::Ok(num) => practice.velocity.y = num,
                    Result::Err(e) => PRACTICE_VELY_LABEL.text = f"VelY (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: PRACTICE_VELZ_LABEL,
            input: f"{practice.velocity.z:.2}",
            onclick: fn(input: string) {
                submit();
            },
            onchange: fn(input: string) {
                PRACTICE_VELZ_LABEL.text = "VelZ";
                match input.parse_float() {
                    Result::Ok(num) => practice.velocity.z = num,
                    Result::Err(e) => PRACTICE_VELZ_LABEL.text = f"VelZ (invalid value)",
                }
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) {
                PRACTICE_X_LABEL.text = "X";
                PRACTICE_Y_LABEL.text = "Y";
                PRACTICE_Z_LABEL.text = "Z";
                PRACTICE_PITCH_LABEL.text = "Pitch";
                PRACTICE_YAW_LABEL.text = "Yaw";
                PRACTICE_ROLL_LABEL.text = "Roll";
                PRACTICE_VELX_LABEL.text = "VelX";
                PRACTICE_VELY_LABEL.text = "VelY";
                PRACTICE_VELZ_LABEL.text = "VelZ";
                leave_ui();
            },
        }),
    ))
}

static mut CURRENT_PRACTICE = Practice {
    name: "none",
    cluster: 0,
    button: 0,
    location: Location { x: 0., y: 0., z: 0. },
    rotation: Rotation { pitch: 0., yaw: 0., roll: 0. },
    velocity: Velocity { x: 0., y: 0., z: 0. },
};
static mut CURRENT_PRACTICE_MAP = Tas::current_map();

fn press_buttons_until(buttons: int) {
    let map = Tas::current_map();
    let mut buttons_pressed = 0;
    let mut level_index = 0;
    while level_index < 31 {
        let mut element_index = 0;
        for button in map.clusters.get(level_index).unwrap().buttons {
            Tas::trigger_element(ElementIndex { cluster_index: level_index, element_type: ElementType::Button, element_index: element_index });
            element_index += 1;
            buttons_pressed += 1;
            if buttons_pressed > buttons {
                return;
            }
        }
        level_index += 1;
    }
}
fn collect_cubes_until(cubes: int) {
    let map = Tas::current_map();
    let mut cubes_collected = 0;
    let mut level_index = 0;
    while level_index < 31 {
        let mut element_index = 0;
        for cube in map.clusters.get(level_index).unwrap().cubes {
            Tas::trigger_element(ElementIndex { cluster_index: level_index, element_type: ElementType::Cube, element_index: element_index });
            element_index += 1;
            cubes_collected += 1;
            if cubes_collected >= cubes {
                return;
            }
        }
        level_index += 1;
    }
}

enum PracticeMode {
    Practicing,
    Editing
}

static PRACTICE_COMPONENT = Component {
    id: PRACTICE_COMPONENT_ID,
    conflicts_with: List::of(MULTIPLAYER_COMPONENT_ID, NEW_GAME_100_PERCENT_COMPONENT_ID, PRACTICE_COMPONENT_ID, RANDOMIZER_COMPONENT_ID),
    draw_hud_text: fn(text: string) -> string {
        match PRACTICE_STATE.mode {
            PracticeMode::Practicing => f"{text}\nPracticing: {CURRENT_PRACTICE.name}",
            PracticeMode::Editing => f"{text}\nEditing: {CURRENT_PRACTICE.name}"
        }
    },
    draw_hud_always: fn() {},
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {},
    on_yield: fn() {},
    on_new_game: fn() {
        CURRENT_PRACTICE_MAP = Tas::current_map();
        Tas::set_all_cluster_speeds(999999999.);
        Tas::set_rotation(CURRENT_PRACTICE.rotation);
        Tas::set_location(CURRENT_PRACTICE.location);
        Tas::set_velocity(CURRENT_PRACTICE.velocity);
        Tas::set_acceleration(Acceleration { x: 0., y: 0., z: 0. });
    },
    on_level_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_cubes_change: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {
        Tas::exit_water();
        press_buttons_until(CURRENT_PRACTICE.button);
        Tas::apply_map_cluster_speeds(CURRENT_PRACTICE_MAP);
        Tas::set_rotation(CURRENT_PRACTICE.rotation);
        Tas::set_location(CURRENT_PRACTICE.location);
        Tas::set_velocity(CURRENT_PRACTICE.velocity);
        Tas::set_acceleration(Acceleration { x: 0., y: 0., z: 0. });
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

static PRACTICE_POINTS = List::of(
    Practice { name: "5 Turn & 6 Elevator", cluster: 4, button: 3, rotation: Rotation { pitch: 0., yaw:  180., roll:  0. }, location: Location { x: -4284., y: -806., z: 840. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Ls Jump", cluster: 6, button: 5, rotation: Rotation { pitch: 0., yaw:  180., roll:  0. }, location: Location { x: -4265., y: -2989., z: 90. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Faster 9", cluster: 7, button: 7, rotation: Rotation { pitch: 355., yaw: 0., roll: 0. }, location: Location { x: -3010., y: -3657., z: 1589. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Dive Skip", cluster: 8, button: 8, rotation: Rotation { pitch: 343., yaw: 180., roll: 0. }, location: Location { x: -244., y: -3374., z: 1589. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Pit", cluster: 10, button: 11, rotation: Rotation { pitch: 0., yaw:  90., roll:  0. }, location: Location { x: 1859., y: -869., z: 89. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "LoF & Spiral Skip", cluster: 18, button: 20, rotation: Rotation { pitch: 353., yaw: 7., roll: 0. }, location: Location { x: -1180., y: -3920., z: 464. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "21", cluster: 19, button: 21, rotation: Rotation { pitch: 0., yaw:  35., roll:  0. }, location: Location { x: 4015., y: -2743., z: 589. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Pillars Superjump", cluster: 25, button: 29, rotation: Rotation { pitch: 356., yaw:  204., roll:  0. }, location: Location { x: 832., y: 6302., z: 89. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Pillars", cluster: 25, button: 30, rotation: Rotation { pitch: 0., yaw:  256., roll:  0. }, location: Location { x: -847., y: 5589., z: 231. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    // same as Button 30
    Practice { name: "Final Climb / Hdnoftr", cluster: 27, button: 33, rotation: Rotation { pitch: 355., yaw: 22., roll: 0. }, location: Location { x: 2802., y: 1779., z: 89. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 2 & 3", cluster: 0, button: -1, rotation: Rotation { pitch: 353., yaw: 146., roll: 0. }, location: Location { x: -820., y: -975., z: 714. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 4", cluster: 2, button: 1, rotation: Rotation { pitch: 339., yaw: 187., roll: 0. }, location: Location { x: 2074., y: -260., z: 1107. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 5", cluster: 3, button: 2, rotation: Rotation { pitch: 355., yaw: 185., roll: 0. }, location: Location { x: -1429., y: -727., z: 89. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 6", cluster: 4, button: 3, rotation: Rotation { pitch: 350., yaw: 180., roll: 0. }, location: Location { x: -4345., y: -807., z: 839. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 7", cluster: 5, button: 4, rotation: Rotation { pitch: 305., yaw: 204., roll: 0. }, location: Location { x: -3241., y: -2295., z: 1607. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 8", cluster: 6, button: 5, rotation: Rotation { pitch: 350., yaw: 197., roll: 0. }, location: Location { x: -4434., y: -2816., z: 89. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 9", cluster: 7, button: 7, rotation: Rotation { pitch: 0., yaw: 0., roll: 0. }, location: Location { x: -2994., y: -3846., z: 1589. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 10", cluster: 8, button: 8, rotation: Rotation { pitch: 0., yaw: 0., roll: 0. }, location: Location { x: -1065., y: -3842., z: 464. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 11", cluster: 9, button: 10, rotation: Rotation { pitch: 341., yaw: 355., roll: 0. }, location: Location { x: 1509., y: -2263., z: 214. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 12", cluster: 10, button: 11, rotation: Rotation { pitch: 358., yaw: 90., roll: 0. }, location: Location { x: 1875., y: 0., z: 89. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 13", cluster: 11, button: 12, rotation: Rotation { pitch: 345., yaw: 90., roll: 0. }, location: Location { x: 2382., y: -431., z: 107. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 14 & 15", cluster: 12, button: 13, rotation: Rotation { pitch: 347., yaw: 152., roll: 0. }, location: Location { x: 782., y: 2361., z: 214. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 16", cluster: 15, button: 16, rotation: Rotation { pitch: 330., yaw: 203., roll: 0. }, location: Location { x: -730., y: 1534., z: 839. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 17", cluster: 15, button: 16, rotation: Rotation { pitch: 334., yaw: 31., roll: 0. }, location: Location { x: -2969., y: 1226., z: 839. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 18", cluster: 17, button: 18, rotation: Rotation { pitch: 352., yaw: 230., roll: 0. }, location: Location { x: -2905., y: 0., z: 89. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 18.2", cluster: 17, button: 19, rotation: Rotation { pitch: 357., yaw: 319., roll: 0. }, location: Location { x: -4490., y: -2761., z: 89. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 19", cluster: 18, button: 20, rotation: Rotation { pitch: 341., yaw: 358., roll: 0. }, location: Location { x: -4148., y: -4008., z: 1607. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 20", cluster: 18, button: 20, rotation: Rotation { pitch: 337., yaw: 1., roll: 0. }, location: Location { x: 1732., y: -3711., z: 1214. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 21", cluster: 19, button: 21, rotation: Rotation { pitch: 337., yaw: 185., roll: 0. }, location: Location { x: 4507., y: -2205., z: 1089. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 22", cluster: 20, button: 22, rotation: Rotation { pitch: 358., yaw: 81., roll: 0. }, location: Location { x: 2777., y: -3962., z: 53. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 23", cluster: 21, button: 23, rotation: Rotation { pitch: 351., yaw: 101., roll: 0. }, location: Location { x: 3143., y: -1298., z: 214. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 24", cluster: 22, button: 24, rotation: Rotation { pitch: 331., yaw: 87., roll: 0. }, location: Location { x: 2346., y: 1976., z: 589. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 25", cluster: 23, button: 26, rotation: Rotation { pitch: 340., yaw: 355., roll: 0. }, location: Location { x: 622., y: 4773., z: 1339. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 26", cluster: 24, button: 27, rotation: Rotation { pitch: 351., yaw: 138., roll: 0. }, location: Location { x: 4122., y: 5066., z: 214. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 27", cluster: 25, button: 30, rotation: Rotation { pitch: 357., yaw: 257., roll: 0. }, location: Location { x: -1137., y: 4550., z: 89. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 28", cluster: 26, button: 31, rotation: Rotation { pitch: 349., yaw: 232., roll: 0. }, location: Location { x: -1504., y: 3045., z: 971. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 29", cluster: 27, button: 33, rotation: Rotation { pitch: 347., yaw: 15., roll: 0. }, location: Location { x: -4130., y: -3., z: 589. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 30", cluster: 27, button: 33, rotation: Rotation { pitch: 355., yaw: 22., roll: 0. }, location: Location { x: 2802., y: 1779., z: 89. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 31", cluster: 29, button: 34, rotation: Rotation { pitch: 359., yaw: 253., roll: 0. }, location: Location { x: 4123., y: 977., z: 89. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
    Practice { name: "Button 32", cluster: 30, button: 35, rotation: Rotation { pitch: 300., yaw: 108., roll: 0. }, location: Location { x: 2625., y: -2250., z: 1357. }, velocity: Velocity { x: 0., y: 0., z: 0. } },
);
