use std::{fs, fs::File};
use std::collections::HashSet;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};
use livesplit_core::{Segment, TimeSpan, TimingMethod, Timer as LiveSplitTimer, Run as LiveSplitRun, Layout as LiveSplitLayout, run::{saver::livesplit, parser::composite}, GeneralLayoutSettings, Component, analysis::total_playtime::TotalPlaytime as LiveSplitTotalPlaytime};
use livesplit_core::component::{CurrentPace as LiveSplitCurrentPace, DetailedTimer, PbChance, PossibleTimeSave, PreviousSegment, Splits, SumOfBest, Title};
use livesplit_core::{comparison::{personal_best, best_segments, best_split_times, average_segments, median_segments, worst_segments, balanced_pb, latest_run}, Segment, TimeSpan, TimingMethod, Timer as LiveSplitTimer, Run as LiveSplitRun, Layout as LiveSplitLayout, run::{saver::livesplit, parser::composite}, GeneralLayoutSettings, Component, analysis::total_playtime::TotalPlaytime as LiveSplitTotalPlaytime};
use livesplit_core::component::{CurrentPace as LiveSplitCurrentPace, DetailedTimer, PbChance, PossibleTimeSave, PreviousSegment, Splits, SumOfBest, Title};
use livesplit_core::settings::Color as LiveSplitColor;
use livesplit_core::timing::formatter::{Accuracy as LiveSplitAccuracy, DigitsFormat as LiveSplitDigitsFormat, TimeFormatter, timer::TimeWithFraction};
use super::rebo_init::Color;

static LIVESPLIT_STATE: LazyLock<Mutex<LiveSplit>> = LazyLock::new(|| Mutex::new(LiveSplit::new()));
static VALID_SPLITS_PATHS: LazyLock<Mutex<HashSet<PathBuf>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

pub struct LiveSplit {
    pub timer: LiveSplitTimer,
    pub layout: LiveSplitLayout,
    total_playtime_formatter: TimeWithFraction,
    sum_of_best_formatter: TimeWithFraction,
    current_pace_formatter: TimeWithFraction,
}
pub struct Timer {}
pub struct Run {}
pub struct Layout {}
pub struct TotalPlaytime {}
pub struct SumOfBestSegments {}
pub struct CurrentPace {}

#[derive(rebo::ExternalType)]
pub enum DigitsFormat {
    SingleDigitSeconds,
    DoubleDigitSeconds,
    SingleDigitMinutes,
    DoubleDigitMinutes,
    SingleDigitHours,
    DoubleDigitHours,
}
#[derive(rebo::ExternalType)]
pub enum Accuracy {
    Seconds,
    Tenths,
    Hundredths,
    Milliseconds,
}
#[derive(rebo::ExternalType)]
pub enum Comparison {
    PersonalBest,
    BestSegments,
    BestSplitTimes,
    AverageSegments,
    MedianSegments,
    WorstSegments,
    BalancedPB,
    LatestRun,
}

impl From<DigitsFormat> for LiveSplitDigitsFormat {
    fn from(format: DigitsFormat) -> LiveSplitDigitsFormat {
        match format {
            DigitsFormat::SingleDigitSeconds => LiveSplitDigitsFormat::SingleDigitSeconds,
            DigitsFormat::DoubleDigitSeconds => LiveSplitDigitsFormat::DoubleDigitSeconds,
            DigitsFormat::SingleDigitMinutes => LiveSplitDigitsFormat::SingleDigitMinutes,
            DigitsFormat::DoubleDigitMinutes => LiveSplitDigitsFormat::DoubleDigitMinutes,
            DigitsFormat::SingleDigitHours => LiveSplitDigitsFormat::SingleDigitHours,
            DigitsFormat::DoubleDigitHours => LiveSplitDigitsFormat::DoubleDigitHours,
        }
    }
}
impl From<LiveSplitDigitsFormat> for DigitsFormat {
    fn from(format: LiveSplitDigitsFormat) -> Self {
        match format {
            LiveSplitDigitsFormat::SingleDigitSeconds => DigitsFormat::SingleDigitSeconds,
            LiveSplitDigitsFormat::DoubleDigitSeconds => DigitsFormat::DoubleDigitSeconds,
            LiveSplitDigitsFormat::SingleDigitMinutes => DigitsFormat::SingleDigitMinutes,
            LiveSplitDigitsFormat::DoubleDigitMinutes => DigitsFormat::DoubleDigitMinutes,
            LiveSplitDigitsFormat::SingleDigitHours => DigitsFormat::SingleDigitHours,
            LiveSplitDigitsFormat::DoubleDigitHours => DigitsFormat::DoubleDigitHours,
        }
    }
}
impl From<Accuracy> for LiveSplitAccuracy {
    fn from(accuracy: Accuracy) -> LiveSplitAccuracy {
        match accuracy {
            Accuracy::Seconds => LiveSplitAccuracy::Seconds,
            Accuracy::Tenths => LiveSplitAccuracy::Tenths,
            Accuracy::Hundredths => LiveSplitAccuracy::Hundredths,
            Accuracy::Milliseconds => LiveSplitAccuracy::Milliseconds,
        }
    }
}
impl From<LiveSplitAccuracy> for Accuracy {
    fn from(accuracy: LiveSplitAccuracy) -> Self {
        match accuracy {
            LiveSplitAccuracy::Seconds => Accuracy::Seconds,
            LiveSplitAccuracy::Tenths => Accuracy::Tenths,
            LiveSplitAccuracy::Hundredths => Accuracy::Hundredths,
            LiveSplitAccuracy::Milliseconds => Accuracy::Milliseconds,
        }
    }
}

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
        let total_playtime_formatter = TimeWithFraction::new(LiveSplitDigitsFormat::SingleDigitSeconds, LiveSplitAccuracy::Hundredths);
        let sum_of_best_formatter = TimeWithFraction::new(LiveSplitDigitsFormat::SingleDigitSeconds, LiveSplitAccuracy::Hundredths);
        let current_pace_formatter = TimeWithFraction::new(LiveSplitDigitsFormat::SingleDigitSeconds, LiveSplitAccuracy::Hundredths);
        LiveSplit { timer, layout, total_playtime_formatter, sum_of_best_formatter, current_pace_formatter }
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
    pub fn total_playtime_digits_format() -> LiveSplitDigitsFormat {
        LIVESPLIT_STATE.lock().unwrap().total_playtime_formatter.time.digits_format()
    }
    pub fn set_total_playtime_digits_format(total_playtime_digits_format: LiveSplitDigitsFormat) {
        LIVESPLIT_STATE.lock().unwrap().total_playtime_formatter.time.set_digits_format(total_playtime_digits_format);
    }
    pub fn total_playtime_accuracy() -> LiveSplitAccuracy {
        LIVESPLIT_STATE.lock().unwrap().total_playtime_formatter.fraction.accuracy()
    }
    pub fn set_total_playtime_accuracy(accuracy: LiveSplitAccuracy) {
        LIVESPLIT_STATE.lock().unwrap().total_playtime_formatter.fraction.set_accuracy(accuracy);
    }
    pub fn sum_of_best_digits_format() -> LiveSplitDigitsFormat {
        LIVESPLIT_STATE.lock().unwrap().sum_of_best_formatter.time.digits_format()
    }
    pub fn set_sum_of_best_digits_format(sum_of_best_digits_format: LiveSplitDigitsFormat) {
        LIVESPLIT_STATE.lock().unwrap().sum_of_best_formatter.time.set_digits_format(sum_of_best_digits_format);
    }
    pub fn sum_of_best_accuracy() -> LiveSplitAccuracy {
        LIVESPLIT_STATE.lock().unwrap().sum_of_best_formatter.fraction.accuracy()
    }
    pub fn set_sum_of_best_accuracy(accuracy: LiveSplitAccuracy) {
        LIVESPLIT_STATE.lock().unwrap().sum_of_best_formatter.fraction.set_accuracy(accuracy);
    }
    pub fn current_pace_digits_format() -> LiveSplitDigitsFormat {
        LIVESPLIT_STATE.lock().unwrap().current_pace_formatter.time.digits_format()
    }
    pub fn set_current_pace_digits_format(current_pace_digits_format: LiveSplitDigitsFormat) {
        LIVESPLIT_STATE.lock().unwrap().current_pace_formatter.time.set_digits_format(current_pace_digits_format);
    }
    pub fn current_pace_accuracy() -> LiveSplitAccuracy {
        LIVESPLIT_STATE.lock().unwrap().current_pace_formatter.fraction.accuracy()
    }
    pub fn set_current_pace_accuracy(accuracy: LiveSplitAccuracy) {
        LIVESPLIT_STATE.lock().unwrap().current_pace_formatter.fraction.set_accuracy(accuracy);
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
        layout.push(Component::CurrentPace(LiveSplitCurrentPace::new()));
        layout.push(Component::PbChance(PbChance::new()));
        layout.push(Component::SumOfBest(SumOfBest::new()));
        layout
    }
    fn settings() -> GeneralLayoutSettings {
        LIVESPLIT_STATE.lock().unwrap().layout.general_settings().clone()
    }
}
impl TotalPlaytime {
    pub fn total_playtime() -> String {
        let livesplit_state = LIVESPLIT_STATE.lock().unwrap();
        livesplit_state.total_playtime_formatter.format(livesplit_state.timer.total_playtime()).to_string()
    }
}
impl SumOfBestSegments {
    pub fn sum_of_best() -> String {
        let livesplit_state = LIVESPLIT_STATE.lock().unwrap();
        let segments = livesplit_state.timer.run().segments();
        let sum_of_best = livesplit_core::analysis::sum_of_segments::calculate_best(segments, true, true, TimingMethod::GameTime)
            .unwrap_or(TimeSpan::zero());
        livesplit_state.sum_of_best_formatter.format(sum_of_best).to_string()
    }
}
impl CurrentPace {
    pub fn current_pace(comparison: Comparison) -> String {
        let comparison = match comparison {
            Comparison::PersonalBest => "Personal Best",
            Comparison::BestSegments => "Best Segments",
            Comparison::BestSplitTimes => "Best Split Times",
            Comparison::AverageSegments => "Average Segments",
            Comparison::MedianSegments => "Median Segments",
            Comparison::WorstSegments => "Worst Segments",
            Comparison::BalancedPB => "Balanced PB",
            Comparison::LatestRun => "Latest Run",
        };
        let livesplit_state = LIVESPLIT_STATE.lock().unwrap();
        let foo = livesplit_core::analysis::current_pace::calculate(&livesplit_state.timer.snapshot(), comparison);
        livesplit_state.current_pace_formatter.format(foo.0.unwrap_or(TimeSpan::zero())).to_string()
    }
}
impl CurrentPace {
    pub fn current_pace(comparison: Comparison) -> String {
        let comparison = match comparison {
            Comparison::PersonalBest => personal_best::NAME,
            Comparison::BestSegments => best_segments::NAME,
            Comparison::BestSplitTimes => best_split_times::NAME,
            Comparison::AverageSegments => average_segments::NAME,
            Comparison::MedianSegments => median_segments::NAME,
            Comparison::WorstSegments => worst_segments::NAME,
            Comparison::BalancedPB => balanced_pb::NAME,
            Comparison::LatestRun => latest_run::NAME,
        };
        let livesplit_state = LIVESPLIT_STATE.lock().unwrap();
        let current_pace = livesplit_core::analysis::current_pace::calculate(&livesplit_state.timer.snapshot(), comparison);
        livesplit_state.current_pace_formatter.format(current_pace.0.unwrap_or(TimeSpan::zero())).to_string()
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
#[rebo::function("LiveSplit::get_total_playtime")]
pub fn livesplit_get_total_playtime() -> String {
    TotalPlaytime::total_playtime()
}
#[rebo::function("LiveSplit::get_total_playtime_digits_format")]
pub fn livesplit_get_total_playtime_digits_format() -> DigitsFormat {
    Timer::total_playtime_digits_format().into()
}
#[rebo::function("LiveSplit::set_total_playtime_digits_format")]
pub fn livesplit_set_total_playtime_digits_format(digits_format: DigitsFormat) {
    Timer::set_total_playtime_digits_format(digits_format.into());
}
#[rebo::function("LiveSplit::get_total_playtime_accuracy")]
pub fn livesplit_get_total_playtime_accuracy() -> Accuracy {
    Timer::total_playtime_accuracy().into()
}
#[rebo::function("LiveSplit::set_total_playtime_accuracy")]
pub fn livesplit_set_total_playtime_accuracy(total_playtime_accuracy: Accuracy) {
    Timer::set_total_playtime_accuracy(total_playtime_accuracy.into());
}
#[rebo::function("LiveSplit::get_sum_of_best_segments")]
pub fn livesplit_get_sum_of_best_segments() -> String {
    SumOfBestSegments::sum_of_best()
}
#[rebo::function("LiveSplit::get_sum_of_best_digits_format")]
pub fn livesplit_get_sum_of_best_digits_format() -> DigitsFormat {
    Timer::sum_of_best_digits_format().into()
}
#[rebo::function("LiveSplit::set_sum_of_best_digits_format")]
pub fn livesplit_set_sum_of_best_digits_format(digits_format: DigitsFormat) {
    Timer::set_sum_of_best_digits_format(digits_format.into());
}
#[rebo::function("LiveSplit::get_sum_of_best_accuracy")]
pub fn livesplit_get_sum_of_best_accuracy() -> Accuracy {
    Timer::sum_of_best_accuracy().into()
}
#[rebo::function("LiveSplit::set_sum_of_best_accuracy")]
pub fn livesplit_set_sum_of_best_accuracy(sum_of_best_accuracy: Accuracy) {
    Timer::set_sum_of_best_accuracy(sum_of_best_accuracy.into());
}
#[rebo::function("LiveSplit::get_current_pace")]
pub fn livesplit_get_current_pace(comparison: Comparison) -> String {
    CurrentPace::current_pace(comparison)
}
#[rebo::function("LiveSplit::get_current_pace_digits_format")]
pub fn livesplit_get_current_pace_digits_format() -> DigitsFormat {
    Timer::current_pace_digits_format().into()
}
#[rebo::function("LiveSplit::set_current_pace_digits_format")]
pub fn livesplit_set_current_pace_digits_format(digits_format: DigitsFormat) {
    Timer::set_current_pace_digits_format(digits_format.into());
}
#[rebo::function("LiveSplit::get_current_pace_accuracy")]
pub fn livesplit_get_current_pace_accuracy() -> Accuracy {
    Timer::current_pace_accuracy().into()
}
#[rebo::function("LiveSplit::set_current_pace_accuracy")]
pub fn livesplit_set_current_pace_accuracy(current_pace_accuracy: Accuracy) {
    Timer::set_current_pace_accuracy(current_pace_accuracy.into());
}
