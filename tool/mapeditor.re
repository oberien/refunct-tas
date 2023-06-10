static mut MAP_EDITOR_STATE = MapEditorState {
    map_name: "",
    map: Tas::current_map(),
};

struct MapEditorState {
    map_name: string,
    map: RefunctMap,
}

static MAP_EDITOR_COMPONENT = Component {
    id: MAP_EDITOR_COMPONENT_ID,
    conflicts_with: List::of(MAP_EDITOR_COMPONENT_ID),
    draw_hud_text: fn(text: string) -> string {
        f"{text}\nMap Editor - editing map {MAP_EDITOR_STATE.map_name:?}\n    <TAB> edit an element    <e> select looked-at element"
     },
    draw_hud_always: fn() {},
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {},
    on_yield: fn() {},
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_element_pressed: fn(index: ElementIndex) {},
    on_element_released: fn(index: ElementIndex) {},
    on_key_down: fn(key: KeyCode, is_repeat: bool) {
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
    on_key_up: fn(key: KeyCode) {},
    on_mouse_move: fn(x: int, y: int) {},
    on_component_exit: fn() {},
    on_resolution_change: fn() {},
    on_menu_open: fn() {},
};

static mut MAP_EDITOR_LABEL = Text { text: if CURRENT_COMPONENTS.contains(MAP_EDITOR_COMPONENT) { "Stop Map Editor" } else { "Edit Map" } };
fn create_map_editor_menu() -> Ui {
    Ui::new("Map Editor", List::of(
        UiElement::Button(UiButton {
            label: MAP_EDITOR_LABEL,
            onclick: fn(label: Text) {
                if CURRENT_COMPONENTS.contains(MAP_EDITOR_COMPONENT) {
                    remove_component(MAP_EDITOR_COMPONENT);
                    MAP_EDITOR_LABEL.text = "Edit Map";
                    MAP_EDITOR_STATE.map = Tas::original_map();
                    Tas::apply_map(MAP_EDITOR_STATE.map);
                } else {
                    let map_list = Tas::list_maps();
                    enter_ui(Ui::new_filechooser("Map to edit", map_list, fn(input: string) {
                        MAP_EDITOR_STATE.map_name = input;
                        if map_list.contains(input) {
                            MAP_EDITOR_STATE.map = Tas::load_map(input);
                            Tas::apply_map(MAP_EDITOR_STATE.map);
                        } else {
                            MAP_EDITOR_STATE.map = Tas::current_map();
                        };
                        MAP_EDITOR_LABEL.text = "Stop Map Editor";
                        add_component(MAP_EDITOR_COMPONENT);
                        leave_ui();
                        leave_ui();
                        leave_ui();
                    }));
                }
            },
        }),
        UiElement::Button(UiButton {
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
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui() },
        }),
    ))
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
    Ui::new("Map Editor - What do you want to modify? (format: <cluster>pl/b/p/l/pi/s<num>, ex: 14pl2 or 2b1 or 4c1 or 9l1 or 5pi1 or 25s1)", List::of(
        UiElement::Input(Input {
            label: MAP_EDITOR_INPUT_LABEL,
            input: "",
            onclick: fn(input: string) {
                let indexes = input.find_matches("\\d+");
                MAP_EDITOR_INPUT_LABEL.text = "Input";
                if indexes.len() != 2 {
                    MAP_EDITOR_INPUT_LABEL.text = "Input (ERROR: need 2 numbers)";
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
                    MAP_EDITOR_INPUT_LABEL.text = "Input (ERROR: must contain pl / c / b / l / pi / s)";
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
    static mut MAP_EDITOR_XSCALE_LABEL = Text { text: "XScale" };
    static mut MAP_EDITOR_YSCALE_LABEL = Text { text: "YScale" };
    static mut MAP_EDITOR_ZSCALE_LABEL = Text { text: "ZScale" };
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
        UiElement::Input(Input {
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
        UiElement::Input(Input {
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
        UiElement::Input(Input {
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
        UiElement::Input(Input {
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
        UiElement::Input(Input {
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
        UiElement::Input(Input {
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
        UiElement::Input(Input {
            label: MAP_EDITOR_XSCALE_LABEL,
            input: f"{element.xscale:.1}",
            onclick: fn(input: string) { submit() },
            onchange: fn(input: string) {
                MAP_EDITOR_XSCALE_LABEL.text = "XScale";
                match input.parse_float() {
                    Result::Ok(num) => element.xscale = num,
                    Result::Err(e) => MAP_EDITOR_XSCALE_LABEL.text = f"XScale (invalid value)",
                }
            },
        }),
        UiElement::Input(Input {
            label: MAP_EDITOR_YSCALE_LABEL,
            input: f"{element.yscale:.1}",
            onclick: fn(input: string) { submit() },
            onchange: fn(input: string) {
                MAP_EDITOR_YSCALE_LABEL.text = "YScale";
                match input.parse_float() {
                    Result::Ok(num) => element.yscale = num,
                    Result::Err(e) => MAP_EDITOR_YSCALE_LABEL.text = f"YScale (invalid value)",
                }
            },
        }),
        UiElement::Input(Input {
            label: MAP_EDITOR_ZSCALE_LABEL,
            input: f"{element.zscale:.1}",
            onclick: fn(input: string) { submit() },
            onchange: fn(input: string) {
                MAP_EDITOR_ZSCALE_LABEL.text = "ZScale";
                match input.parse_float() {
                    Result::Ok(num) => element.zscale = num,
                    Result::Err(e) => MAP_EDITOR_ZSCALE_LABEL.text = f"ZScale (invalid value)",
                }
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
                element.xscale = original_element.xscale;
                element.yscale = original_element.yscale;
                element.zscale = original_element.zscale;
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
                MAP_EDITOR_XSCALE_LABEL.text = "XScale";
                MAP_EDITOR_YSCALE_LABEL.text = "YScale";
                MAP_EDITOR_ZSCALE_LABEL.text = "ZScale";
                leave_ui();
            },
        }),
    ))
}
