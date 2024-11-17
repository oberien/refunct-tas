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
        UiElement::FloatInput(FloatInput {
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
        UiElement::FloatInput(FloatInput {
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
        UiElement::FloatInput(FloatInput {
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
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Log Message Duration (s)" },
            input: f"{SETTINGS.log_message_duration.to_float() / 1000.}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(val) => {
                        if 0.0 <= val {
                            let milliseconds = val * 1000.;
                            SETTINGS.log_message_duration = milliseconds.round(0).to_int();
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
    lighting_casts_shadows: bool,
    time_dilation: float,
    gravity: float,
    kill_z: float,
    outro_dilated_duration: float,
    outro_time_dilation: float,
    time_of_day: float,
    sky_time_speed: float,
    sky_light_enabled: bool,
    sky_light_brightness: float,
    sky_light_intensity: float,
    sun_redness: float,
    cloud_redness: float,
    cloud_speed: float,
    fog_enabled: bool,
    reflection_render_scale: int,
    display_gamma: float,
    day_stars_brightness: float,
    night_stars_brightness: float,
    screen_percentage: float,
    reticle_w: float,
    reticle_h: float,
    reticle_scale: float,
    reticle_scale_position: bool,
    log_message_duration: int,
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
        let get_int = fn(key: string, default: int) -> int {
            match map.get(key) {
                Option::Some(val) => val.parse_int().unwrap(),
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
            ui_scale: get_float("ui_scale", 1.),
            show_character_stats: get_bool("show_character_stats", false),
            show_game_stats: get_bool("show_game_stats", false),
            minimap_enabled: get_bool("minimap_enabled", false),
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
            lighting_casts_shadows: get_bool("lighting_casts_shadows", true),
            time_dilation: get_float("time_dilation", 1.),
            gravity: get_float("gravity", -980.),
            kill_z: get_float("kill_z", -2550.),
            outro_dilated_duration: get_float("outro_dilated_duration", 0.6),
            outro_time_dilation: get_float("outro_time_dilation", 0.03),
            time_of_day: get_float("time_of_day", 0.),
            sky_time_speed: get_float("sky_time_speed", 120.),
            sky_light_enabled: get_bool("sky_light_enabled", true),
            sky_light_brightness: get_float("sky_light_brightness", 3.141592741),
            sky_light_intensity: get_float("sky_light_intensity", 10.),
            sun_redness: get_float("sun_redness", 0.21960786),
            cloud_redness: get_float("cloud_redness", 0.23137257),
            cloud_speed: get_float("cloud_speed", 2.),
            fog_enabled: get_bool("fog_enabled", true),
            reflection_render_scale: get_int("reflection_render_scale", 100),
            display_gamma: get_float("display_gamma", 2.2),
            day_stars_brightness: get_float("day_stars_brightness", 0.),
            night_stars_brightness: get_float("night_stars_brightness", 5.),
            screen_percentage: get_float("screen_percentage", 100.),
            reticle_w: get_float("reticle_w", 6.),
            reticle_h: get_float("reticle_h", 6.),
            reticle_scale: get_float("reticle_scale", 1.),
            reticle_scale_position: get_bool("reticle_scale_position", false),
            log_message_duration: get_int("log_message_duration", 10000),
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
        map.insert("lighting_casts_shadows", f"{SETTINGS.lighting_casts_shadows}");
        map.insert("time_dilation", f"{SETTINGS.time_dilation}");
        map.insert("gravity", f"{SETTINGS.gravity}");
        map.insert("kill_z", f"{SETTINGS.kill_z}");
        map.insert("outro_dilated_duration", f"{SETTINGS.outro_dilated_duration}");
        map.insert("outro_time_dilation", f"{SETTINGS.outro_time_dilation}");
        map.insert("time_of_day", f"{SETTINGS.time_of_day}");
        map.insert("sky_time_speed", f"{SETTINGS.sky_time_speed}");
        map.insert("sky_light_enabled", f"{SETTINGS.sky_light_enabled}");
        map.insert("sky_light_brightness", f"{SETTINGS.sky_light_brightness}");
        map.insert("sky_light_intensity", f"{SETTINGS.sky_light_intensity}");
        map.insert("sun_redness", f"{SETTINGS.sun_redness}");
        map.insert("cloud_redness", f"{SETTINGS.cloud_redness}");
        map.insert("cloud_speed", f"{SETTINGS.cloud_speed}");
        map.insert("fog_enabled", f"{SETTINGS.fog_enabled}");
        map.insert("reflection_render_scale", f"{SETTINGS.reflection_render_scale}");
        map.insert("display_gamma", f"{SETTINGS.display_gamma}");
        map.insert("day_stars_brightness", f"{SETTINGS.day_stars_brightness}");
        map.insert("night_stars_brightness", f"{SETTINGS.night_stars_brightness}");
        map.insert("screen_percentage", f"{SETTINGS.screen_percentage}");
        map.insert("reticle_w", f"{SETTINGS.reticle_w}");
        map.insert("reticle_h", f"{SETTINGS.reticle_h}");
        map.insert("reticle_scale", f"{SETTINGS.reticle_scale}");
        map.insert("reticle_scale_position", f"{SETTINGS.reticle_scale_position}");
        map.insert("log_message_duration", f"{SETTINGS.log_message_duration}");
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
    fn toggle_lighting_casts_shadows(mut self) {
        self.lighting_casts_shadows = !self.lighting_casts_shadows;
        self.store();
    }
    fn toggle_sky_light_enabled(mut self) {
        self.sky_light_enabled = !self.sky_light_enabled;
        self.store();
    }
    fn toggle_fog_enabled(mut self) {
        self.fog_enabled = !self.fog_enabled;
        self.store();
    }
}
