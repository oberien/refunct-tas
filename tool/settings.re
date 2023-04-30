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
        Settings {
            ui_scale: get_float("ui_scale", 2.),
            show_character_stats: get_bool("show_character_stats", false),
            show_game_stats: get_bool("show_game_stats", false),
            minimap_enabled: get_bool("minimap_enabled", true),
            minimap_size: get_float("minimap_size", 0.35),
            minimap_alpha: get_float("minimap_alpha", 0.4),
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
