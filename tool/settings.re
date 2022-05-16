struct Settings {
    ui_scale: float,
    show_character_stats: bool,
    show_game_stats: bool,
}
enum SettingsManuallySet {
    UiScale,
    ShowCharacterStats,
    ShowGameStats,
}
impl SettingsManuallySet {
    fn to_string(self) -> string {
        match self {
            SettingsManuallySet::UiScale => SETTINGS_UI_SCALE,
            SettingsManuallySet::ShowCharacterStats => SETTINGS_SHOW_CHARACTER_STATS,
            SettingsManuallySet::ShowGameStats => SETTINGS_SHOW_GAME_STATS,
        }
    }

    fn access(self, settings: Settings) -> string {
        match self {
            SettingsManuallySet::UiScale => f"{settings.ui_scale:?}",
            SettingsManuallySet::ShowCharacterStats => f"{settings.show_character_stats}",
            SettingsManuallySet::ShowGameStats => f"{settings.show_game_stats}",
        }
    }
}

static SETTINGS_UI_SCALE = "ui_scale";
static SETTINGS_SHOW_CHARACTER_STATS = "show_character_stats";
static SETTINGS_SHOW_GAME_STATS = "show_game_stats";
static mut SETTINGS_MANUALLY_SET = Set::new();
static mut SETTINGS = Settings::load();

impl Settings {
    fn load() -> Settings {
        let settings = Tas::load_settings();
        let map = settings.unwrap_or(Map::new());
        let ui_scale = match map.get("ui_scale") {
            Option::Some(scale) => {
                SETTINGS_MANUALLY_SET.insert(SettingsManuallySet::UiScale);
                let list = scale.split("\\.");
                let num = list.get(0).unwrap().parse_int().unwrap();
                let decimal = list.get(1).unwrap().parse_int().unwrap();
                num.to_float() + decimal.to_float() / 10.
            },
            Option::None => 2.,
        };
        let show_character_stats = match map.get(SETTINGS_SHOW_CHARACTER_STATS) {
            Option::Some(char_stats) => {
                SETTINGS_MANUALLY_SET.insert(SettingsManuallySet::ShowCharacterStats);
                char_stats == "true"
            },
            Option::None => false,
        };
        let show_game_stats = match map.get("show_game_stats") {
            Option::Some(game_stats) => {
                SETTINGS_MANUALLY_SET.insert(SettingsManuallySet::ShowGameStats);
                game_stats == "true"
            },
            Option::None => false,
        };
        Settings {
            ui_scale: ui_scale,
            show_character_stats: show_character_stats,
            show_game_stats: show_game_stats,
        }
    }

    fn store(self) {
        let mut map = Map::new();
        for setting in SETTINGS_MANUALLY_SET.values() {
            map.insert(setting.to_string(), setting.access(SETTINGS));
        }
        Tas::store_settings(map);
    }

    fn increase_ui_scale(mut self) {
        SETTINGS_MANUALLY_SET.insert(SettingsManuallySet::UiScale);
        self.ui_scale += 0.5;
        self.ui_scale = self.ui_scale.min(10.);
        self.store();
    }
    fn decrease_ui_scale(mut self) {
        SETTINGS_MANUALLY_SET.insert(SettingsManuallySet::UiScale);
        self.ui_scale -= 0.5;
        self.ui_scale = self.ui_scale.max(0.5);
        self.store();
    }
    fn toggle_show_character_stats(mut self) {
        SETTINGS_MANUALLY_SET.insert(SettingsManuallySet::ShowCharacterStats);
        self.show_character_stats = !self.show_character_stats;
        self.store();
    }
    fn toggle_show_game_stats(mut self) {
        SETTINGS_MANUALLY_SET.insert(SettingsManuallySet::ShowGameStats);
        self.show_game_stats = !self.show_game_stats;
        self.store();
    }
}
