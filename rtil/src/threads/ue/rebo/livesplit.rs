use std::{fs, fs::File};
use std::collections::HashSet;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};
use livesplit_core::{Segment, TimeSpan, TimingMethod, Timer as LiveSplitTimer, Run as LiveSplitRun, Layout as LiveSplitLayout, run::{saver::livesplit, parser::composite}, GeneralLayoutSettings, Component};
use livesplit_core::component::{CurrentPace, DetailedTimer, PbChance, PossibleTimeSave, PreviousSegment, Splits, SumOfBest, Title};
use livesplit_core::settings::Color as LiveSplitColor;
use super::rebo_init::Color;

static LIVESPLIT_STATE: LazyLock<Mutex<LiveSplit>> = LazyLock::new(|| Mutex::new(LiveSplit::new()));
static VALID_SPLITS_PATHS: LazyLock<Mutex<HashSet<PathBuf>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

pub struct LiveSplit {
    pub timer: LiveSplitTimer,
    pub layout: LiveSplitLayout,
}
pub struct Timer {}
pub struct Run {}
pub struct Layout {}

#[derive(rebo::ExternalType)]
pub enum Game {
    Refunct,
    RefunctCategoryExtensions,
    RefunctMultiplayer,
    RefunctRandomizer,
}
#[derive(rebo::ExternalType)]
pub enum NewGameGlitch {
    Yes,
    No,
}
#[derive(rebo::ExternalType)]
pub enum SplitsSaveError {
    /// filename, error description
    CreationFailed(String, String),
    /// filename, error description
    SaveFailed(String, String),
    /// path
    DisallowedFilePath(String),
}
#[derive(rebo::ExternalType)]
pub enum SplitsLoadError {
    /// filename, error description
    OpenFailed(String, String),
    /// filename, error description
    ParseFailed(String, String),
}

impl LiveSplit {
    pub fn new() -> LiveSplit {
        let mut run = LiveSplitRun::new();
        let layout = Layout::new();
        run.set_game_name("Refunct");
        run.set_category_name("Any%");
        run.push_segment(Segment::new("1"));
        run.metadata_mut().set_speedrun_com_variable("New Game Glitch", "Normal");
        let mut timer = LiveSplitTimer::new(run).unwrap();
        timer.set_current_timing_method(TimingMethod::GameTime);
        LiveSplit { timer, layout }
    }
    fn splits_path() -> PathBuf {
        let appdata_path = super::rebo_init::data_path();
        let splits_path = appdata_path.join("splits/");
        if !splits_path.is_dir() {
            fs::create_dir(&splits_path).unwrap();
        }
        splits_path
    }
}
impl Timer {
    pub fn start() {
        Self::reset(true);
        let mut state = LIVESPLIT_STATE.lock().unwrap();
        state.timer.start();
        state.timer.initialize_game_time();
    }
    pub fn split() {
        LIVESPLIT_STATE.lock().unwrap().timer.split();
    }
    pub fn reset(update_splits: bool) {
        LIVESPLIT_STATE.lock().unwrap().timer.reset(update_splits);
    }
    pub fn get_game_time() -> f64 {
        LIVESPLIT_STATE.lock().unwrap().timer.snapshot().current_time().game_time.unwrap().total_seconds()
    }
    pub fn pause_game_time() {
        LIVESPLIT_STATE.lock().unwrap().timer.pause_game_time();
    }
    pub fn set_game_time(time: f64) {
        LIVESPLIT_STATE.lock().unwrap().timer.set_game_time(TimeSpan::from_seconds(time));
    }
}
impl Run {
    pub fn game_name() -> String {
        LIVESPLIT_STATE.lock().unwrap().timer.run().clone().game_name().to_owned()
    }
    pub fn set_game_info(game: Game, category: String, new_game_glitch: NewGameGlitch) {
        let mut state = LIVESPLIT_STATE.lock().unwrap();
        let mut run = state.timer.run().clone();
        let game = match game {
            Game::Refunct => "Refunct",
            Game::RefunctCategoryExtensions => "Refunct Category Extensions",
            Game::RefunctMultiplayer => "Refunct Multiplayer",
            Game::RefunctRandomizer => "Refunct Randomizer",
        };
        run.set_game_name(game);
        run.set_category_name(category);
        if run.metadata_mut().speedrun_com_variables.iter().any(|(name, _)| name == "New Game Glitch") {
            let cat = match new_game_glitch {
                NewGameGlitch::Yes => "New Game Glitch",
                NewGameGlitch::No => "Normal",
            };
            run.metadata_mut().set_speedrun_com_variable("New Game Glitch", cat);
        }
        state.timer.set_run(run).unwrap();
    }
    pub fn category_name() -> String {
        LIVESPLIT_STATE.lock().unwrap().timer.run().category_name().to_owned()
    }
    pub fn segments() -> Vec<Segment> {
        LIVESPLIT_STATE.lock().unwrap().timer.run().segments().to_owned()
    }
    pub fn create_segment(name: String) {
        let mut state = LIVESPLIT_STATE.lock().unwrap();
        let mut run = state.timer.run().clone();
        run.push_segment(Segment::new(name));
        state.timer.set_run(run).unwrap();
    }
    pub fn attempt_count() -> u32 {
        LIVESPLIT_STATE.lock().unwrap().timer.run().clone().attempt_count()
    }
    pub fn save_splits(user_input: &str) -> Result<(), SplitsSaveError> {
        let state = LIVESPLIT_STATE.lock().unwrap();
        let valid_splits_paths = VALID_SPLITS_PATHS.lock().unwrap();
        let path = Path::new(user_input);
        let (path, path_display) = match (path.is_relative(), valid_splits_paths.contains(path)) {
            (true, _) => {
                let sanitized = sanitize_filename::sanitize(user_input);
                (LiveSplit::splits_path().join(&sanitized), sanitized)
            },
            (false, true) => (path.to_owned(), user_input.to_owned()),
            (false, false) => return Err(SplitsSaveError::DisallowedFilePath(user_input.to_string())),
        };
        let file = File::create(&path)
            .map_err(|e| SplitsSaveError::CreationFailed(path_display.clone(), e.to_string()))?;
        livesplit::save_timer(&state.timer, livesplit::IoWrite(BufWriter::new(file)))
            .map_err(|e| SplitsSaveError::SaveFailed(path_display.clone(), e.to_string()))?;
        Ok(())
    }
    pub fn load_splits(user_input: &str) -> Result<(), SplitsLoadError> {
        let path = Path::new(user_input);
        let (path, path_display) = match path.is_relative() {
            true => {
                let sanitized = sanitize_filename::sanitize(user_input);
                let path = LiveSplit::splits_path().join(sanitized);
                let path_display = path.to_str().expect("Could not convert to &str").to_owned();
                (path, path_display)
            },
            false => (path.to_owned(), user_input.to_owned()),
        };
        let file = fs::read(&path)
            .map_err(|e| SplitsLoadError::OpenFailed(path_display.to_owned(), e.to_string()))?;
        let parsed_run = composite::parse(&file, None)
            .map_err(|e| SplitsLoadError::ParseFailed(path_display.to_owned(), e.to_string()))?;
        let mut state = LIVESPLIT_STATE.lock().unwrap();
        state.timer.reset(true);
        state.timer.set_run(parsed_run.run).unwrap();
        VALID_SPLITS_PATHS.lock().unwrap().insert(path);
        Ok(())
    }
}
impl Layout {
    pub fn new() -> LiveSplitLayout {
        let mut layout = LiveSplitLayout::new();
        layout.push(Component::Title(Title::new()));
        layout.push(Component::Splits(Splits::new()));
        layout.push(Component::DetailedTimer(Box::new(DetailedTimer::new())));
        layout.push(Component::PreviousSegment(PreviousSegment::new()));
        layout.push(Component::PossibleTimeSave(PossibleTimeSave::new()));
        layout.push(Component::CurrentPace(CurrentPace::new()));
        layout.push(Component::PbChance(PbChance::new()));
        layout.push(Component::SumOfBest(SumOfBest::new()));
        layout
    }
    fn settings() -> GeneralLayoutSettings {
        LIVESPLIT_STATE.lock().unwrap().layout.general_settings().clone()
    }
}
#[rebo::function("LiveSplit::get_best_segment_color")]
pub fn livesplit_get_best_segment_color() -> Color {
    let color = Layout::settings().best_segment_color;
    Color { red: color.red, green: color.green, blue: color.blue, alpha: color.alpha }
}
#[rebo::function("LiveSplit::set_best_segment_color")]
pub fn livesplit_set_best_segment_color(color: Color) {
    let color = LiveSplitColor::rgba(color.red, color.green, color.blue, color.alpha);
    let mut state = LIVESPLIT_STATE.lock().unwrap();
    state.layout.general_settings_mut().best_segment_color = color;
}
#[rebo::function("LiveSplit::get_ahead_gaining_time_color")]
pub fn livesplit_get_ahead_gaining_time_color() -> Color {
    let color = Layout::settings().ahead_gaining_time_color;
    Color { red: color.red, green: color.green, blue: color.blue, alpha: color.alpha }
}
#[rebo::function("LiveSplit::set_ahead_gaining_time_color")]
pub fn livesplit_set_ahead_gaining_time_color(color: Color) {
    let color = LiveSplitColor::rgba(color.red, color.green, color.blue, color.alpha);
    let mut state = LIVESPLIT_STATE.lock().unwrap();
    state.layout.general_settings_mut().ahead_gaining_time_color = color;
}
#[rebo::function("LiveSplit::get_behind_gaining_time_color")]
pub fn livesplit_get_behind_gaining_time_color() -> Color {
    let color = Layout::settings().behind_gaining_time_color;
    Color { red: color.red, green: color.green, blue: color.blue, alpha: color.alpha }
}
#[rebo::function("LiveSplit::set_behind_gaining_time_color")]
pub fn livesplit_set_behind_gaining_time_color(color: Color) {
    let color = LiveSplitColor::rgba(color.red, color.green, color.blue, color.alpha);
    let mut state = LIVESPLIT_STATE.lock().unwrap();
    state.layout.general_settings_mut().behind_gaining_time_color = color;
}
#[rebo::function("LiveSplit::get_behind_losing_time_color")]
pub fn livesplit_get_behind_losing_time_color() -> Color {
    let color = Layout::settings().behind_losing_time_color;
    Color { red: color.red, green: color.green, blue: color.blue, alpha: color.alpha }
}
#[rebo::function("LiveSplit::set_behind_losing_time_color")]
pub fn livesplit_set_behind_losing_time_color(color: Color) {
    let color = LiveSplitColor::rgba(color.red, color.green, color.blue, color.alpha);
    let mut state = LIVESPLIT_STATE.lock().unwrap();
    state.layout.general_settings_mut().behind_losing_time_color = color;
}
#[rebo::function("LiveSplit::get_not_running_color")]
pub fn livesplit_get_not_running_color() -> Color {
    let color = Layout::settings().not_running_color;
    Color { red: color.red, green: color.green, blue: color.blue, alpha: color.alpha }
}
#[rebo::function("LiveSplit::set_not_running_color")]
pub fn livesplit_set_not_running_color(color: Color) {
    let color = LiveSplitColor::rgba(color.red, color.green, color.blue, color.alpha);
    let mut state = LIVESPLIT_STATE.lock().unwrap();
    state.layout.general_settings_mut().not_running_color = color;
}
#[rebo::function("LiveSplit::get_personal_best_color")]
pub fn livesplit_get_personal_best_color() -> Color {
    let color = Layout::settings().personal_best_color;
    Color { red: color.red, green: color.green, blue: color.blue, alpha: color.alpha }
}
#[rebo::function("LiveSplit::livesplit_set_personal_best_color")]
pub fn livesplit_set_personal_best_color(color: Color) {
    let color = LiveSplitColor::rgba(color.red, color.green, color.blue, color.alpha);
    let mut state = LIVESPLIT_STATE.lock().unwrap();
    state.layout.general_settings_mut().personal_best_color = color;
}
#[rebo::function("LiveSplit::get_paused_color")]
pub fn livesplit_get_paused_color() -> Color {
    let color = Layout::settings().paused_color;
    Color { red: color.red, green: color.green, blue: color.blue, alpha: color.alpha }
}
#[rebo::function("LiveSplit::set_paused_color")]
pub fn livesplit_set_paused_color(color: Color) {
    let color = LiveSplitColor::rgba(color.red, color.green, color.blue, color.alpha);
    let mut state = LIVESPLIT_STATE.lock().unwrap();
    state.layout.general_settings_mut().paused_color = color;
}
#[rebo::function("LiveSplit::get_text_color")]
pub fn livesplit_get_text_color() -> Color {
    let color = Layout::settings().text_color;
    Color { red: color.red, green: color.green, blue: color.blue, alpha: color.alpha }
}
#[rebo::function("LiveSplit::set_text_color")]
pub fn livesplit_set_text_color(color: Color) {
    let color = LiveSplitColor::rgba(color.red, color.green, color.blue, color.alpha);
    let mut state = LIVESPLIT_STATE.lock().unwrap();
    state.layout.general_settings_mut().text_color = color;
}
