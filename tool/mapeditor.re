static mut MAP_EDITOR_STATE = MapEditorState {
    map_name: "",
    map: Tas::current_map(),
    mode: MapEditorMode::Edit,
};

struct MapEditorState {
    map_name: string,
    map: RefunctMap,
    mode: MapEditorMode,
}

enum MapEditorMode {
    Play,
    Edit,
}

static MAP_EDITOR_COMPONENT = Component {
    id: MAP_EDITOR_COMPONENT_ID,
    conflicts_with: List::of(MAP_EDITOR_COMPONENT_ID),
    draw_hud_text: fn(text: string) -> string {
        match MAP_EDITOR_STATE.mode {
            MapEditorMode::Edit => f"{text}\nMap Editor - editing map {MAP_EDITOR_STATE.map_name:?}\n    <TAB> edit an element    <e> select looked-at element",
            MapEditorMode::Play => f"{text}\nMap Editor - playing map {MAP_EDITOR_STATE.map_name:?}",
        }
     },
    draw_hud_always: fn() {},
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {},
    on_yield: fn() {},
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_cubes_change: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_element_pressed: fn(index: ElementIndex) {},
    on_element_released: fn(index: ElementIndex) {},
    on_key_down: fn(key: KeyCode, is_repeat: bool) {
        if MAP_EDITOR_STATE.mode != MapEditorMode::Edit {
            return;
        }
        if key.to_small() == KEY_TAB.to_small() {
            enter_ui(create_map_editor_input_ui());
        }
        if key.to_small() == KEY_E.to_small() {
            let index = match Tas::get_looked_at_element_index() {
                Option::Some(index) => index,
                Option::None => return,
            };
            let element = match try_get_element(index) {
                Result::Ok(element) => element,
                _ => return,
            };
            enter_ui(create_map_editor_element_ui(element, index, 0));
        }
    },
    on_key_down_always: fn(key: KeyCode, is_repeat: bool) {},
    on_key_up: fn(key: KeyCode) {},
    on_key_up_always: fn(key: KeyCode) {},
    on_mouse_move: fn(x: int, y: int) {},
    on_component_enter: fn() { Tas::set_max_fly_speed(SETTINGS.flying_forward_backward_velocity) },
    on_component_exit: fn() {},
    on_resolution_change: fn() {},
    on_menu_open: fn() {},
};

fn create_map_editor_menu() -> Ui {
    let list = List::new();
    if !CURRENT_COMPONENTS.contains(MAP_EDITOR_COMPONENT) {
        list.push(UiElement::Button(UiButton {
            label: Text { text: "Edit Map" },
            onclick: fn(label: Text) {
               let map_list = Tas::list_maps();
               enter_ui(Ui::new_filechooser("Map to edit", map_list, fn(input: string) {
                   MAP_EDITOR_STATE.map_name = input;
                   if map_list.contains(input) {
                       MAP_EDITOR_STATE.map = Tas::load_map(input);
                       Tas::apply_map(MAP_EDITOR_STATE.map);
                   } else {
                       MAP_EDITOR_STATE.map = Tas::current_map();
                   };
                   add_component(MAP_EDITOR_COMPONENT);
                   add_component(MOVEMENT_COMPONENT);
                   MAP_EDITOR_STATE.mode = MapEditorMode::Edit;
                   MOVEMENT_STATE.enable_fly = false;
                   leave_ui();
                   leave_ui();
                   leave_ui();
               }));
            },
        }));
        list.push(UiElement::Button(UiButton {
            label: Text { text: "Play Map" },
            onclick: fn(label: Text) {
                let map_list = Tas::list_maps();
                enter_ui(Ui::new_filechooser("Map to play", map_list, fn(input: string) {
                    MAP_EDITOR_STATE.map_name = input;
                    if map_list.contains(input) {
                        MAP_EDITOR_STATE.map = Tas::load_map(input);
                        Tas::apply_map(MAP_EDITOR_STATE.map);
                    } else {
                        MAP_EDITOR_STATE.map = Tas::current_map();
                    };
                    add_component(MAP_EDITOR_COMPONENT);
                    MAP_EDITOR_STATE.mode = MapEditorMode::Play;
                    MOVEMENT_STATE.enable_fly = false;
                    leave_ui();
                    leave_ui();
                    leave_ui();
                }));
            },
        }));
    } else {
        list.push(UiElement::Button(UiButton {
            label: Text { text: "Stop Map Editor" },
            onclick: fn(label: Text) {
               remove_component(MAP_EDITOR_COMPONENT);
               remove_component(MOVEMENT_COMPONENT);
               MAP_EDITOR_STATE.map = Tas::original_map();
               Tas::apply_map(MAP_EDITOR_STATE.map);
               leave_ui();
            },
        }));
    }
    list.push(UiElement::Button(UiButton {
        label: Text { text: "Delete Map" },
        onclick: fn(label: Text) {
            fn create_map_editor_delete_map_menu() -> Ui {
                let map_list = Tas::list_maps();
                Ui::new_filechooser("Map to delete", map_list, fn(input: string) {
                    if map_list.contains(input) {
                        Tas::remove_map(input);
                        leave_ui();
                        enter_ui(create_map_editor_delete_map_menu());
                    }
                })
            }
            enter_ui(create_map_editor_delete_map_menu());
        },
    }));
    list.push(UiElement::Button(UiButton {
       label: Text { text: "Open Maps Folder" },
       onclick: fn(label: Text) { Tas::open_maps_folder(); },
    }));
    list.push(UiElement::Button(UiButton {
       label: Text { text: "Back" },
       onclick: fn(label: Text) { leave_ui() },
    }));
    Ui::new("Map Editor", list)
};

enum TryGetElementError {
    InvalidClusterIndex,
    InvalidElementIndex,
}
fn try_get_element(index: ElementIndex) -> Result<Element, TryGetElementError> {
    let cluster = match MAP_EDITOR_STATE.map.clusters.get(index.cluster_index) {
        Option::Some(cluster) => cluster,
        Option::None => {
            return Result::Err(TryGetElementError::InvalidClusterIndex)
        }
    };
    let mut element_type = ElementType::Platform;
    let element_list = match index.element_type {
        ElementType::Platform => cluster.platforms,
        ElementType::Cube => cluster.cubes,
        ElementType::Button => cluster.buttons,
        ElementType::Lift => cluster.lifts,
        ElementType::Pipe => cluster.pipes,
        ElementType::Springpad => cluster.springpads,
    };

    let element = match element_list.get(index.element_index) {
        Option::Some(element) => element,
        Option::None => {
            return Result::Err(TryGetElementError::InvalidElementIndex)
        }
    };
    Result::Ok(element)
}

static mut MAP_EDITOR_INPUT_LABEL = Text { text: "Input" };
fn create_map_editor_input_ui() -> Ui {
    Ui::new("Map Editor - What do you want to modify? (format: <cluster> or <cluster>pl/b/c/l/pi/s<num>, ex: 1 or 14pl2 or 25s1)", List::of(
        UiElement::Input(Input {
            label: MAP_EDITOR_INPUT_LABEL,
            input: "",
            onclick: fn(input: string) {
                MAP_EDITOR_INPUT_LABEL.text = "Input";
                // check if it's just one number -> edit cluster
                match input.parse_int() {
                    Result::Ok(cluster_index) => {
                        let cluster = match MAP_EDITOR_STATE.map.clusters.get(cluster_index - 1) {
                            Option::Some(cluster) => cluster,
                            Option::None => {
                                MAP_EDITOR_INPUT_LABEL.text = "Input (ERROR: no such cluster exists)";
                                return
                            }
                        };
                        leave_ui();
                        enter_ui(create_map_editor_cluster_ui(cluster, cluster_index - 1));
                        return
                    },
                    Result::Err(err) => (),
                }

                // handle element
                let indexes = input.find_matches("\\d+");
                if indexes.len() != 2 {
                    MAP_EDITOR_INPUT_LABEL.text = "Input (ERROR: need 1 or 2 numbers)";
                    return;
                }
                let cluster_index = indexes.get(0).unwrap().parse_int().unwrap() - 1;
                let element_index = indexes.get(1).unwrap().parse_int().unwrap() - 1;

                let element_type = if input.contains("pl") {
                    ElementType::Platform
                } else if input.contains("c") {
                    ElementType::Cube
                } else if input.contains("b") {
                    ElementType::Button
                } else if input.contains("l") {
                    ElementType::Lift
                } else if input.contains("pi") {
                    ElementType::Pipe
                } else if input.contains("s") {
                    ElementType::Springpad
                } else {
                    MAP_EDITOR_INPUT_LABEL.text = "Input (ERROR: must contain pl / b / c / l / pi / s)";
                    return
                };

                let index = ElementIndex {
                    cluster_index: cluster_index,
                    element_type: element_type,
                    element_index: element_index
                };
                let element = match try_get_element(index) {
                    Result::Ok(element) => element,
                    Result::Err(err) => match err {
                        TryGetElementError::InvalidClusterIndex => {
                            MAP_EDITOR_INPUT_LABEL.text = "Input (ERROR: invalid cluster index)";
                            return
                        },
                        TryGetElementError::InvalidElementIndex => {
                            MAP_EDITOR_INPUT_LABEL.text = f"Input (ERROR: invalid {index.element_type} index)";
                            return
                        },
                    },
                };

                leave_ui();
                enter_ui(create_map_editor_element_ui(element, index, 0));
            },
            onchange: fn(input: string) {}
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui() },
        }),
    ))
}

fn create_map_editor_cluster_ui(mut cluster: Cluster, cluster_index: int) -> Ui {
    static mut MAP_EDITOR_CLUSTER_Z_LABEL = Text { text: "Initial Z" };
    static mut MAP_EDITOR_CLUSTER_SPEED_LABEL = Text { text: "Rise Speed" };
    Ui::new(f"Map Editor - Edit Cluster {cluster_index + 1}", List::of(
        UiElement::FloatInput(FloatInput {
            label: MAP_EDITOR_CLUSTER_Z_LABEL,
            input: f"{cluster.z:.1}",
            onclick: fn(input: string) {
                Tas::save_map(MAP_EDITOR_STATE.map_name, MAP_EDITOR_STATE.map);
                Tas::apply_map(MAP_EDITOR_STATE.map);
            },
            onchange: fn(input: string) {
                MAP_EDITOR_CLUSTER_Z_LABEL.text = "Initial Z";
                match input.parse_float() {
                    Result::Ok(num) => cluster.z = num,
                    Result::Err(e) => MAP_EDITOR_CLUSTER_Z_LABEL.text = f"Initial Z (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: MAP_EDITOR_CLUSTER_SPEED_LABEL,
            input: f"{cluster.rise_speed:.1}",
            onclick: fn(input: string) {
                Tas::save_map(MAP_EDITOR_STATE.map_name, MAP_EDITOR_STATE.map);
                Tas::apply_map(MAP_EDITOR_STATE.map);
            },
            onchange: fn(input: string) {
                MAP_EDITOR_CLUSTER_SPEED_LABEL.text = "Rise Speed";
                match input.parse_float() {
                    Result::Ok(num) => cluster.rise_speed = num,
                    Result::Err(e) => MAP_EDITOR_CLUSTER_SPEED_LABEL.text = f"Rise Speed (invalid value)",
                }
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) {
                MAP_EDITOR_CLUSTER_Z_LABEL.text = "Initial Z";
                MAP_EDITOR_CLUSTER_SPEED_LABEL.text = "Rise Speed";
                leave_ui();
            },
        }),
    ))
}

fn create_map_editor_element_ui(mut element: Element, index: ElementIndex, selected: int) -> Ui {
    let submit = fn() {
        let selected = match UI_STACK.last() {
            Option::Some(ui) => ui.selected,
            Option::None => panic("we are currently in a UI"),
        };
        Tas::save_map(MAP_EDITOR_STATE.map_name, MAP_EDITOR_STATE.map);
        leave_ui();
        Tas::apply_map(MAP_EDITOR_STATE.map);
        enter_ui(create_map_editor_element_ui(element, index, selected));
    };

    static mut MAP_EDITOR_X_LABEL = Text { text: "X" };
    static mut MAP_EDITOR_Y_LABEL = Text { text: "Y" };
    static mut MAP_EDITOR_Z_LABEL = Text { text: "Z" };
    static mut MAP_EDITOR_PITCH_LABEL = Text { text: "Pitch" };
    static mut MAP_EDITOR_YAW_LABEL = Text { text: "Yaw" };
    static mut MAP_EDITOR_ROLL_LABEL = Text { text: "Roll" };
    static mut MAP_EDITOR_SIZEX_LABEL = Text { text: "SizeX" };
    static mut MAP_EDITOR_SIZEY_LABEL = Text { text: "SizeY" };
    static mut MAP_EDITOR_SIZEZ_LABEL = Text { text: "SizeZ" };
    Ui::new_with_selected(f"Map Editor - Edit Cluster {index.cluster_index + 1} {index.element_type} {index.element_index + 1}", selected, List::of(
        UiElement::Button(UiButton {
            label: Text { text: "Set to player location" },
            onclick: fn(label: Text) {
                let loc = Tas::get_location();
                let bounds = match index.element_type {
                    ElementType::Platform => Tas::get_element_bounds(index),
                    ElementType::Cube => Bounds { originx: 0., originy: 0., originz: 0., extentx: 0., extenty: 0., extentz: 0. },
                    ElementType::Button => Bounds { originx: 0., originy: 0., originz: 0., extentx: 0., extenty: 0., extentz: 0. },
                    ElementType::Lift => Tas::get_element_bounds(index),
                    ElementType::Pipe => Bounds { originx: 0., originy: 0., originz: 0., extentx: 0., extenty: 0., extentz: 0. },
                    ElementType::Springpad => {
                        let mut bounds = Tas::get_element_bounds(index);
                        bounds.extentz -= 112.;
                        bounds
                    },
                };
                element.x = loc.x - bounds.extentx;
                element.y = loc.y - bounds.extenty;
                element.z = (loc.z - 89.15) - bounds.extentz * 2.;
                submit();
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Set to player rotation" },
            onclick: fn(label: Text) {
                let rot = Tas::get_rotation();
                element.pitch = rot.pitch;
                element.yaw = rot.yaw;
                element.roll = rot.roll;
                submit();
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: MAP_EDITOR_X_LABEL,
            input: f"{element.x:.1}",
            onclick: fn(input: string) { submit() },
            onchange: fn(input: string) {
                MAP_EDITOR_X_LABEL.text = "X";
                match input.parse_float() {
                    Result::Ok(num) => element.x = num,
                    Result::Err(e) => MAP_EDITOR_X_LABEL.text = f"X (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: MAP_EDITOR_Y_LABEL,
            input: f"{element.y:.1}",
            onclick: fn(input: string) { submit() },
            onchange: fn(input: string) {
                MAP_EDITOR_Y_LABEL.text = "Y";
                match input.parse_float() {
                    Result::Ok(num) => element.y = num,
                    Result::Err(e) => MAP_EDITOR_Y_LABEL.text = f"Y (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: MAP_EDITOR_Z_LABEL,
            input: f"{element.z:.1}",
            onclick: fn(input: string) { submit() },
            onchange: fn(input: string) {
                MAP_EDITOR_Z_LABEL.text = "Z";
                match input.parse_float() {
                    Result::Ok(num) => element.z = num,
                    Result::Err(e) => MAP_EDITOR_Z_LABEL.text = f"Z (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: MAP_EDITOR_PITCH_LABEL,
            input: f"{element.pitch:.1}",
            onclick: fn(input: string) { submit() },
            onchange: fn(input: string) {
                MAP_EDITOR_PITCH_LABEL.text = "Pitch";
                match input.parse_float() {
                    Result::Ok(num) => element.pitch = num,
                    Result::Err(e) => MAP_EDITOR_PITCH_LABEL.text = f"Pitch (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: MAP_EDITOR_YAW_LABEL,
            input: f"{element.yaw:.1}",
            onclick: fn(input: string) { submit() },
            onchange: fn(input: string) {
                MAP_EDITOR_YAW_LABEL.text = "Yaw";
                match input.parse_float() {
                    Result::Ok(num) => element.yaw = num,
                    Result::Err(e) => MAP_EDITOR_YAW_LABEL.text = f"Yaw (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: MAP_EDITOR_ROLL_LABEL,
            input: f"{element.roll:.1}",
            onclick: fn(input: string) { submit() },
            onchange: fn(input: string) {
                MAP_EDITOR_ROLL_LABEL.text = "Roll";
                match input.parse_float() {
                    Result::Ok(num) => element.roll = num,
                    Result::Err(e) => MAP_EDITOR_ROLL_LABEL.text = f"Roll (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: MAP_EDITOR_SIZEX_LABEL,
            input: f"{element.sizex:.1}",
            onclick: fn(input: string) { submit() },
            onchange: fn(input: string) {
                MAP_EDITOR_SIZEX_LABEL.text = "SizeX";
                match input.parse_float() {
                    Result::Ok(num) => element.sizex = num,
                    Result::Err(e) => MAP_EDITOR_SIZEX_LABEL.text = f"SizeX (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: MAP_EDITOR_SIZEY_LABEL,
            input: f"{element.sizey:.1}",
            onclick: fn(input: string) { submit() },
            onchange: fn(input: string) {
                MAP_EDITOR_SIZEY_LABEL.text = "SizeY";
                match input.parse_float() {
                    Result::Ok(num) => element.sizey = num,
                    Result::Err(e) => MAP_EDITOR_SIZEY_LABEL.text = f"SizeY (invalid value)",
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: MAP_EDITOR_SIZEZ_LABEL,
            input: f"{element.sizez:.1}",
            onclick: fn(input: string) { submit() },
            onchange: fn(input: string) {
                MAP_EDITOR_SIZEZ_LABEL.text = "SizeZ";
                match input.parse_float() {
                    Result::Ok(num) => element.sizez = num,
                    Result::Err(e) => MAP_EDITOR_SIZEZ_LABEL.text = f"SizeZ (invalid value)",
                }
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Reset to original rotation" },
            onclick: fn(label: Text) {
                let original_map = Tas::original_map();
                let cluster = original_map.clusters.get(index.cluster_index).unwrap();
                let element_list = match index.element_type {
                    ElementType::Platform => cluster.platforms,
                    ElementType::Cube => cluster.cubes,
                    ElementType::Button => cluster.buttons,
                    ElementType::Lift => cluster.lifts,
                    ElementType::Pipe => cluster.pipes,
                    ElementType::Springpad => cluster.springpads,
                };
                let original_element = element_list.get(index.element_index).unwrap();
                element.pitch = original_element.pitch;
                element.yaw = original_element.yaw;
                element.roll = original_element.roll;
                submit();
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Reset to original values" },
            onclick: fn(label: Text) {
                let original_map = Tas::original_map();
                let cluster = original_map.clusters.get(index.cluster_index).unwrap();
                let element_list = match index.element_type {
                    ElementType::Platform => cluster.platforms,
                    ElementType::Cube => cluster.cubes,
                    ElementType::Button => cluster.buttons,
                    ElementType::Lift => cluster.lifts,
                    ElementType::Pipe => cluster.pipes,
                    ElementType::Springpad => cluster.springpads,
                };
                let original_element = element_list.get(index.element_index).unwrap();
                element.x = original_element.x;
                element.y = original_element.y;
                element.z = original_element.z;
                element.pitch = original_element.pitch;
                element.yaw = original_element.yaw;
                element.roll = original_element.roll;
                element.sizex = original_element.sizex;
                element.sizey = original_element.sizey;
                element.sizez = original_element.sizez;
                submit();
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) {
                MAP_EDITOR_X_LABEL.text = "X";
                MAP_EDITOR_Y_LABEL.text = "Y";
                MAP_EDITOR_Z_LABEL.text = "Z";
                MAP_EDITOR_PITCH_LABEL.text = "Pitch";
                MAP_EDITOR_YAW_LABEL.text = "Yaw";
                MAP_EDITOR_ROLL_LABEL.text = "Roll";
                MAP_EDITOR_SIZEX_LABEL.text = "SizeX";
                MAP_EDITOR_SIZEY_LABEL.text = "SizeY";
                MAP_EDITOR_SIZEZ_LABEL.text = "SizeZ";
                leave_ui();
            },
        }),
    ))
}
