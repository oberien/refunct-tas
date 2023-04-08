enum ReplayMenuOp {
    Save,
    Load,
    Delete,
}

fn create_replay_menu(op: ReplayMenuOp) -> Ui{
    let mut recording_name_label = Text { text: "Recording name" };

    let recordings_list = Tas::list_recordings();
    let do_operation = fn(input: string) {
        match op {
            ReplayMenuOp::Save => {
                tas_save_recording(input);
                leave_ui();
            },
            ReplayMenuOp::Load => {
                if !recordings_list.contains(input) {
                    recording_name_label.text = f"Recording name (Error: no such file)";
                    return;
                }
                tas_load_recording(input);
                add_component(TAS_COMPONENT);
                leave_ui();
                leave_ui();
                leave_ui();
            },
            ReplayMenuOp::Delete => {
                if !recordings_list.contains(input) {
                    recording_name_label.text = f"Recording name (Error: no such file)";
                    return;
                }
                Tas::remove_recording(input);
                leave_ui();
            },
        }
    };
    recording_name_label.text = f"Recording name";
    let mut recordings = List::of(
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui() },
        }),
        UiElement::Input(Input {
            label: recording_name_label,
            input: "",
            onclick: fn(input: string) {
                if input.len_utf8() == 0 {
                    recording_name_label.text = f"Recording name (Error: empty name)";
                    return;
                }
                do_operation(input);
            },
            onchange: fn(input: string) {}
        }),
    );
    for recording in recordings_list {
        recordings.push(UiElement::Button(UiButton {
            label: Text { text: recording },
            onclick: fn(label: Text) {
                do_operation(label.text);
            },
        }));
    }
    Ui::new("Recording Options:", recordings)
}

fn create_misc_menu() -> Ui {
    Ui::new("Misc:", List::of(
        UiElement::Button(UiButton {
            label: Text { text: "Enable Timer" },
            onclick: fn(label: Text) {
                add_component(TIMER_COMPONENT);
                leave_ui();
                leave_ui();
            }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Disable Timer" },
            onclick: fn(label: Text) {
                remove_component(TIMER_COMPONENT);
                leave_ui();
                leave_ui();
            }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Save Recording" },
            onclick: fn(label: Text) {
                enter_ui(create_replay_menu(ReplayMenuOp::Save));
            }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Load Recording" },
            onclick: fn(label: Text) {
                enter_ui(create_replay_menu(ReplayMenuOp::Load));
            }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Delete Recording" },
            onclick: fn(label: Text) {
                enter_ui(create_replay_menu(ReplayMenuOp::Delete));
            }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "TAS Mode" },
            onclick: fn(label: Text) {
                add_component(TAS_COMPONENT);
                leave_ui();
                leave_ui();
            }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Stop TAS Mode" },
            onclick: fn(label: Text) {
                remove_component(TAS_COMPONENT);
                leave_ui();
                leave_ui();
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
            onchange: fn(index: int) { 
                Tas::set_movement_mode(index);
                print(f"Movement Mode [mms]: {index}");
                print(f"Movement Mode [mms]: {Tas::get_movement_mode()}");
            },
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
