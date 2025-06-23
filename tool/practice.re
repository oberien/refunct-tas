fn create_practice_menu() -> Ui {
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
    Ui::new("Practice:", buttons)
}

static mut CURRENT_PRACTICE = Practice {
    name: "none",
    cluster: 0,
    button: 0,
    location: Location { x: 0., y: 0., z: 0. },
    rotation: Rotation { pitch: 0., yaw: 0., roll: 0. },
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

static PRACTICE_COMPONENT = Component {
    id: PRACTICE_COMPONENT_ID,
    conflicts_with: List::of(MULTIPLAYER_COMPONENT_ID, NEW_GAME_100_PERCENT_COMPONENT_ID, PRACTICE_COMPONENT_ID, RANDOMIZER_COMPONENT_ID),
    draw_hud_text: fn(text: string) -> string {
        f"{text}\nPracticing: {CURRENT_PRACTICE.name}"
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
        Tas::set_velocity(Velocity { x: 0., y: 0., z: 0. });
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
        Tas::set_velocity(Velocity { x: 0., y: 0., z: 0. });
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

struct Practice {
    name: string,
    /// cluster to be risen already
    cluster: int,
    /// buttons to be pressed already
    button: int,
    location: Location,
    rotation: Rotation,
}

static PRACTICE_POINTS = List::of(
    Practice { name: "5 Turn & 6 Elevator", cluster: 4, button: 3, rotation: Rotation { pitch: 0., yaw:  180., roll:  0. }, location: Location { x: -4284., y: -806., z: 840. } },
    Practice { name: "Ls Jump", cluster: 6, button: 5, rotation: Rotation { pitch: 0., yaw:  180., roll:  0. }, location: Location { x: -4265., y: -2989., z: 90. } },
    Practice { name: "Faster 9", cluster: 7, button: 7, rotation: Rotation { pitch: 355., yaw: 0., roll: 0. }, location: Location { x: -3010., y: -3657., z: 1589. } },
    Practice { name: "Dive Skip", cluster: 8, button: 8, rotation: Rotation { pitch: 343., yaw: 180., roll: 0. }, location: Location { x: -244., y: -3374., z: 1589. } },
    Practice { name: "Pit", cluster: 10, button: 11, rotation: Rotation { pitch: 0., yaw:  90., roll:  0. }, location: Location { x: 1859., y: -869., z: 89. } },
    Practice { name: "LoF & Spiral Skip", cluster: 18, button: 20, rotation: Rotation { pitch: 353., yaw: 7., roll: 0. }, location: Location { x: -1180., y: -3920., z: 464. } },
    Practice { name: "21", cluster: 19, button: 21, rotation: Rotation { pitch: 0., yaw:  35., roll:  0. }, location: Location { x: 4015., y: -2743., z: 589. } },
    Practice { name: "Pillars Superjump", cluster: 25, button: 29, rotation: Rotation { pitch: 356., yaw:  204., roll:  0. }, location: Location { x: 832., y: 6302., z: 89. } },
    Practice { name: "Pillars", cluster: 25, button: 30, rotation: Rotation { pitch: 0., yaw:  256., roll:  0. }, location: Location { x: -847., y: 5589., z: 231. } },
    // same as Button 30
    Practice { name: "Final Climb / Hdnoftr", cluster: 27, button: 33, rotation: Rotation { pitch: 355., yaw: 22., roll: 0. }, location: Location { x: 2802., y: 1779., z: 89. } },
    Practice { name: "Button 2 & 3", cluster: 0, button: -1, rotation: Rotation { pitch: 353., yaw: 146., roll: 0. }, location: Location { x: -820., y: -975., z: 714. } },
    Practice { name: "Button 4", cluster: 2, button: 1, rotation: Rotation { pitch: 339., yaw: 187., roll: 0. }, location: Location { x: 2074., y: -260., z: 1107. } },
    Practice { name: "Button 5", cluster: 3, button: 2, rotation: Rotation { pitch: 355., yaw: 185., roll: 0. }, location: Location { x: -1429., y: -727., z: 89. } },
    Practice { name: "Button 6", cluster: 4, button: 3, rotation: Rotation { pitch: 350., yaw: 180., roll: 0. }, location: Location { x: -4345., y: -807., z: 839. } },
    Practice { name: "Button 7", cluster: 5, button: 4, rotation: Rotation { pitch: 305., yaw: 204., roll: 0. }, location: Location { x: -3241., y: -2295., z: 1607. } },
    Practice { name: "Button 8", cluster: 6, button: 5, rotation: Rotation { pitch: 350., yaw: 197., roll: 0. }, location: Location { x: -4434., y: -2816., z: 89. } },
    Practice { name: "Button 9", cluster: 7, button: 7, rotation: Rotation { pitch: 0., yaw: 0., roll: 0. }, location: Location { x: -2994., y: -3846., z: 1589. } },
    Practice { name: "Button 10", cluster: 8, button: 8, rotation: Rotation { pitch: 0., yaw: 0., roll: 0. }, location: Location { x: -1065., y: -3842., z: 464. } },
    Practice { name: "Button 11", cluster: 9, button: 10, rotation: Rotation { pitch: 341., yaw: 355., roll: 0. }, location: Location { x: 1509., y: -2263., z: 214. } },
    Practice { name: "Button 12", cluster: 10, button: 11, rotation: Rotation { pitch: 358., yaw: 90., roll: 0. }, location: Location { x: 1875., y: 0., z: 89. } },
    Practice { name: "Button 13", cluster: 11, button: 12, rotation: Rotation { pitch: 345., yaw: 90., roll: 0. }, location: Location { x: 2382., y: -431., z: 107. } },
    Practice { name: "Button 14 & 15", cluster: 12, button: 13, rotation: Rotation { pitch: 347., yaw: 152., roll: 0. }, location: Location { x: 782., y: 2361., z: 214. } },
    Practice { name: "Button 16", cluster: 15, button: 16, rotation: Rotation { pitch: 330., yaw: 203., roll: 0. }, location: Location { x: -730., y: 1534., z: 839. } },
    Practice { name: "Button 17", cluster: 15, button: 16, rotation: Rotation { pitch: 334., yaw: 31., roll: 0. }, location: Location { x: -2969., y: 1226., z: 839. } },
    Practice { name: "Button 18", cluster: 17, button: 18, rotation: Rotation { pitch: 352., yaw: 230., roll: 0. }, location: Location { x: -2905., y: 0., z: 89. } },
    Practice { name: "Button 18.2", cluster: 17, button: 19, rotation: Rotation { pitch: 357., yaw: 319., roll: 0. }, location: Location { x: -4490., y: -2761., z: 89. } },
    Practice { name: "Button 19", cluster: 18, button: 20, rotation: Rotation { pitch: 341., yaw: 358., roll: 0. }, location: Location { x: -4148., y: -4008., z: 1607. } },
    Practice { name: "Button 20", cluster: 18, button: 20, rotation: Rotation { pitch: 337., yaw: 1., roll: 0. }, location: Location { x: 1732., y: -3711., z: 1214. } },
    Practice { name: "Button 21", cluster: 19, button: 21, rotation: Rotation { pitch: 337., yaw: 185., roll: 0. }, location: Location { x: 4507., y: -2205., z: 1089. } },
    Practice { name: "Button 22", cluster: 20, button: 22, rotation: Rotation { pitch: 358., yaw: 81., roll: 0. }, location: Location { x: 2777., y: -3962., z: 53. } },
    Practice { name: "Button 23", cluster: 21, button: 23, rotation: Rotation { pitch: 351., yaw: 101., roll: 0. }, location: Location { x: 3143., y: -1298., z: 214. } },
    Practice { name: "Button 24", cluster: 22, button: 24, rotation: Rotation { pitch: 331., yaw: 87., roll: 0. }, location: Location { x: 2346., y: 1976., z: 589. } },
    Practice { name: "Button 25", cluster: 23, button: 26, rotation: Rotation { pitch: 340., yaw: 355., roll: 0. }, location: Location { x: 622., y: 4773., z: 1339. } },
    Practice { name: "Button 26", cluster: 24, button: 27, rotation: Rotation { pitch: 351., yaw: 138., roll: 0. }, location: Location { x: 4122., y: 5066., z: 214. } },
    Practice { name: "Button 27", cluster: 25, button: 30, rotation: Rotation { pitch: 357., yaw: 257., roll: 0. }, location: Location { x: -1137., y: 4550., z: 89. } },
    Practice { name: "Button 28", cluster: 26, button: 31, rotation: Rotation { pitch: 349., yaw: 232., roll: 0. }, location: Location { x: -1504., y: 3045., z: 971. } },
    Practice { name: "Button 29", cluster: 27, button: 33, rotation: Rotation { pitch: 347., yaw: 15., roll: 0. }, location: Location { x: -4130., y: -3., z: 589. } },
    Practice { name: "Button 30", cluster: 27, button: 33, rotation: Rotation { pitch: 355., yaw: 22., roll: 0. }, location: Location { x: 2802., y: 1779., z: 89. } },
    Practice { name: "Button 31", cluster: 29, button: 34, rotation: Rotation { pitch: 359., yaw: 253., roll: 0. }, location: Location { x: 4123., y: 977., z: 89. } },
    Practice { name: "Button 32", cluster: 30, button: 35, rotation: Rotation { pitch: 300., yaw: 108., roll: 0. }, location: Location { x: 2625., y: -2250., z: 1357. } },
);
