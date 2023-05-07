fn create_settings_menu() -> Ui {
    let mut ui_scale_text = Text { text: f"{SETTINGS.ui_scale}" };
    let mut show_character_stats_button_text = Text { text: f"Show Character Stats: {SETTINGS.show_character_stats}" };
    let mut show_game_stats_button_text = Text { text: f"Show Game Stats: {SETTINGS.show_game_stats}" };
    Ui::new("Settings:", List::of(
        UiElement::Slider(Slider {
            label: Text { text: "UI Scale" },
            content: ui_scale_text,
            onleft: fn() {
                SETTINGS.decrease_ui_scale();
                ui_scale_text.text = f"{SETTINGS.ui_scale}";
            },
            onright: fn() {
                SETTINGS.increase_ui_scale();
                ui_scale_text.text = f"{SETTINGS.ui_scale}";
            },
        }),
        UiElement::Button(UiButton {
            label: show_character_stats_button_text,
            onclick: fn(label: Text) {
                SETTINGS.toggle_show_character_stats();
                show_character_stats_button_text.text = f"Show Character Stats: {SETTINGS.show_character_stats}";
            },
        }),
        UiElement::Button(UiButton {
            label: show_game_stats_button_text,
            onclick: fn(label: Text) {
                SETTINGS.toggle_show_game_stats();
                show_game_stats_button_text.text = f"Show Game Stats: {SETTINGS.show_game_stats}";
            },
        }),
        UiElement::Input(Input {
            label: Text { text: "Player Color Red (0.0 - 1.0)" },
            input: f"{SETTINGS.player_color_red}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(val) => {
                        if 0.0 <= val && val <= 1.0 {
                            SETTINGS.player_color_red = val;
                            SETTINGS.store();
                        }
                    },
                    Result::Err(e) => (),
                }
            },
        }),
        UiElement::Input(Input {
            label: Text { text: "Player Color Green (0.0 - 1.0)" },
            input: f"{SETTINGS.player_color_green}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(val) => {
                        if 0.0 <= val && val <= 1.0 {
                            SETTINGS.player_color_green = val;
                            SETTINGS.store();
                        }
                    },
                    Result::Err(e) => (),
                }
            },
        }),
        UiElement::Input(Input {
            label: Text { text: "Player Color Blue (0.0 - 1.0)" },
            input: f"{SETTINGS.player_color_blue}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(val) => {
                        if 0.0 <= val && val <= 1.0 {
                            SETTINGS.player_color_blue = val;
                            SETTINGS.store();
                        }
                    },
                    Result::Err(e) => (),
                }
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Reset Game Stats" },
            onclick: fn(label: Text) { GAME_STATS.reset() },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui() },
        }),
    ))
}

struct Settings {
    ui_scale: float,
    show_character_stats: bool,
    show_game_stats: bool,
    minimap_enabled: bool,
    minimap_size: float,
    minimap_alpha: float,
    minimap_position: MinimapPosition,
    player_color_red: float,
    player_color_green: float,
    player_color_blue: float,
    flying_up_down_velocity: float,
    flying_forward_backward_velocity: float,
}
static mut SETTINGS = Settings::load();

impl Settings {
    fn load() -> Settings {
        let settings = Tas::load_settings();
        let map = settings.unwrap_or(Map::new());
        let get_float = fn(key: string, default: float) -> float {
            match map.get(key) {
                Option::Some(val) => val.parse_float().unwrap(),
//                let list = scale.split("\\.");
//                let num = list.get(0).unwrap().parse_int().unwrap();
//                let decimal = list.get(1).unwrap().parse_int().unwrap();
//                num.to_float() + decimal.to_float() / 10.
                Option::None => default,
            }
        };
        let get_bool = fn(key: string, default: bool) -> bool {
            match map.get(key) {
                Option::Some(val) => val == "true",
                Option::None => default,
            }
        };
        let get_string = fn(key: string, default: string) -> string {
            match map.get(key) {
                Option::Some(val) => val,
                Option::None => default,
            }
        };
        Settings {
            ui_scale: get_float("ui_scale", 2.),
            show_character_stats: get_bool("show_character_stats", false),
            show_game_stats: get_bool("show_game_stats", false),
            minimap_enabled: get_bool("minimap_enabled", true),
            minimap_size: get_float("minimap_size", 0.35),
            minimap_alpha: get_float("minimap_alpha", 0.4),
            minimap_position: match get_string("minimap_position", "BottomRight") {
                "TopLeft" => MinimapPosition::TopLeft,
                "TopCenter" => MinimapPosition::TopCenter,
                "TopRight" => MinimapPosition::TopRight,
                "CenterRight" => MinimapPosition::CenterRight,
                "BottomRight" => MinimapPosition::BottomRight,
                "BottomCenter" => MinimapPosition::BottomCenter,
                "BottomLeft" => MinimapPosition::BottomLeft,
                "CenterLeft" => MinimapPosition::CenterLeft,
                "Center" => MinimapPosition::Center,
                pos => panic(f"unknown minimap position {pos}"),
            },
            player_color_red: get_float("player_color_red", 0.),
            player_color_green: get_float("player_color_green", 0.),
            player_color_blue: get_float("player_color_blue", 0.),
            flying_up_down_velocity: get_float("flying_up_down_velocity", 600.),
            flying_forward_backward_velocity: get_float("flying_forward_backward_velocity", 1200.),
        }
    }

    fn store(self) {
        let mut map = Map::new();
        map.insert("ui_scale", f"{SETTINGS.ui_scale}");
        map.insert("show_character_stats", f"{SETTINGS.show_character_stats}");
        map.insert("show_game_stats", f"{SETTINGS.show_game_stats}");
        map.insert("minimap_enabled", f"{SETTINGS.minimap_enabled}");
        map.insert("minimap_size", f"{SETTINGS.minimap_size}");
        map.insert("minimap_alpha", f"{SETTINGS.minimap_alpha}");
        map.insert("minimap_position", f"{SETTINGS.minimap_position}");
        map.insert("player_color_red", f"{SETTINGS.player_color_red}");
        map.insert("player_color_green", f"{SETTINGS.player_color_green}");
        map.insert("player_color_blue", f"{SETTINGS.player_color_blue}");
        map.insert("flying_up_down_velocity", f"{SETTINGS.flying_up_down_velocity}");
        map.insert("flying_forward_backward_velocity", f"{SETTINGS.flying_forward_backward_velocity}");
        Tas::store_settings(map);
    }

    fn increase_ui_scale(mut self) {
        self.ui_scale += 0.5;
        self.ui_scale = self.ui_scale.min(10.);
        self.store();
    }
    fn decrease_ui_scale(mut self) {
        self.ui_scale -= 0.5;
        self.ui_scale = self.ui_scale.max(0.5);
        self.store();
    }
    fn toggle_show_character_stats(mut self) {
        self.show_character_stats = !self.show_character_stats;
        self.store();
    }
    fn toggle_show_game_stats(mut self) {
        self.show_game_stats = !self.show_game_stats;
        self.store();
    }
}
