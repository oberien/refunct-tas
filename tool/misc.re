enum ReplayMenuOp {
    Save,
    Load,
    Delete,
}

static mut TIMER_LABEL = Text { text: if CURRENT_COMPONENTS.contains(TIMER_COMPONENT) { "Disable Timer" } else { "Enable Timer" } };
static mut TAS_LABEL = Text { text: if CURRENT_COMPONENTS.contains(TAS_COMPONENT) { "Disable TAS Mode" } else { "Enable TAS Mode" } };

fn create_misc_menu() -> Ui {
    Ui::new("Misc:", List::of(
        UiElement::Button(UiButton {
            label: TIMER_LABEL,
            onclick: fn(label: Text) {
                if CURRENT_COMPONENTS.contains(TIMER_COMPONENT) {
                    remove_component(TIMER_COMPONENT);
                    TIMER_LABEL.text = "Enable Timer";
                } else {
                    add_component(TIMER_COMPONENT);
                    TIMER_LABEL.text = "Disable Timer";
                }
            }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Save Recording" },
            onclick: fn(label: Text) {
                enter_ui(Ui::new_filechooser("Save Recording", Tas::list_recordings(), fn(input: string) {
                    if TAS_STATE.is_recording {
                        log("[TAS Component] Error: You cannot save file whilst recording!");
                        return;
                    }
                    tas_save_recording(input);
                    leave_ui();
                }));
            }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Load Recording" },
            onclick: fn(label: Text) {
                let recordings_list = Tas::list_recordings();
                enter_ui(Ui::new_filechooser("Load Recording", recordings_list, fn(input: string) {
                    if !recordings_list.contains(input) {
                        return;
                    }
                    tas_load_recording(input);
                    add_component(TAS_COMPONENT);
                    leave_ui();
                    leave_ui();
                    leave_ui();
                }));
            }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Delete Recording" },
            onclick: fn(label: Text) {
                fn create_tas_delete_recording_menu() -> Ui {
                    let recordings_list = Tas::list_recordings();
                    Ui::new_filechooser("Delete Recording", recordings_list, fn(input: string) {
                        if recordings_list.contains(input) {
                            Tas::remove_recording(input);
                            leave_ui();
                            enter_ui(create_tas_delete_recording_menu());
                        }
                    })
                }
                enter_ui(create_tas_delete_recording_menu());
            }
        }),
        UiElement::Button(UiButton {
           label: Text { text: "Open Recordings Folder" },
           onclick: fn(label: Text) { Tas::open_recordings_folder(); },
        }),
        UiElement::Button(UiButton {
            label: TAS_LABEL,
            onclick: fn(label: Text) {
                if CURRENT_COMPONENTS.contains(TAS_COMPONENT) {
                    remove_component(TAS_COMPONENT);
                    TAS_LABEL.text = "Enable TAS Mode";
                } else {
                    add_component(TAS_COMPONENT);
                    TAS_LABEL.text = "Disable TAS Mode";
                }
            }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Movement" },
            onclick: fn(label: Text) {
                add_component(MOVEMENT_COMPONENT);
                enter_ui(create_movement_menu());
            }
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
            selected: Tas::get_movement_mode(),
            onchange: fn(index: int) { Tas::set_movement_mode(index); },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Player" },
            onclick: fn(label: Text) {
                enter_ui(create_player_menu());
            }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Minimap" },
            onclick: fn(label: Text) {
                enter_ui(create_minimap_menu());
            }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "World Options" },
            onclick: fn(label: Text) {
                enter_ui(create_world_options_menu());
            }
        }),
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
        UiElement::FloatInput(FloatInput {
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
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Windscreen Wipers (s/wipe)" },
            input: "2",
            onclick: fn(input: string) {
                let seconds_per_wipe = match input.parse_float() {
                    Result::Ok(seconds_per_wipe) => seconds_per_wipe,
                    Result::Err(e) => return,
                };
                start_windscreen_wipers(seconds_per_wipe);
                add_component(WINDSCREEN_WIPERS_COMPONENT);
            },
            onchange: fn(input: string) {},
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Stop Windscreen Wipers" },
            onclick: fn(label: Text) {
                remove_component(WINDSCREEN_WIPERS_COMPONENT);
            },
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
    ))
}
