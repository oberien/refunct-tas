use std::{fs, fs::File};
use std::collections::HashSet;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};
use livesplit_core::{comparison::{personal_best, best_segments, best_split_times, average_segments, median_segments, worst_segments, balanced_pb, latest_run}, Segment as LiveSplitSegment, TimeSpan, TimingMethod, Timer as LiveSplitTimer, Run as LiveSplitRun, Layout as LiveSplitLayout, run::{saver::livesplit, parser::composite}, Component, analysis::total_playtime::TotalPlaytime as LiveSplitTotalPlaytime};
use livesplit_core::component::{CurrentPace as LiveSplitCurrentPace, DetailedTimer, PbChance, PossibleTimeSave, PreviousSegment, Splits, SumOfBest, Title};
use livesplit_core::settings::Color as LiveSplitColor;
use livesplit_core::timing::formatter::{Accuracy as LiveSplitAccuracy, DigitsFormat as LiveSplitDigitsFormat, TimeFormatter, timer::TimeWithFraction};
use livesplit_core::analysis::{sum_of_segments, current_pace, pb_chance};
use super::rebo_init::{Color, Segment};

static LIVESPLIT_STATE: LazyLock<Mutex<LiveSplit>> = LazyLock::new(|| Mutex::new(LiveSplit::new()));
static VALID_SPLITS_PATHS: LazyLock<Mutex<HashSet<PathBuf>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

pub struct LiveSplit {
    pub layout: LiveSplitLayout,
    pub timer: Timer,
}
pub struct Timer {
    livesplit_timer: LiveSplitTimer,
    total_playtime_formatter: TimeWithFraction,
    sum_of_best_formatter: TimeWithFraction,
    current_pace_formatter: TimeWithFraction,
}
pub struct Run<'a> {
    timer: &'a mut LiveSplitTimer,
}
pub struct Layout {}

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

#[derive(rebo::ExternalType)]
pub enum LiveSplitLayoutColor {
    BestSegment,
    AheadGainingTime,
    AheadLosingTime,
    BehindGainingTime,
    BehindLosingTime,
    NotRunning,
    PersonalBest,
    Paused,
    Text,
}

impl LiveSplit {
    pub fn new() -> LiveSplit {
        let mut run = LiveSplitRun::new();
        let layout = Layout::new();
        run.set_game_name("Refunct");
        run.set_category_name("Any%");
        run.push_segment(LiveSplitSegment::new("1"));
        run.metadata_mut().set_speedrun_com_variable("New Game Glitch", "Normal");
        let mut livesplit_timer = LiveSplitTimer::new(run).unwrap();
        livesplit_timer.set_current_timing_method(TimingMethod::GameTime);
        let total_playtime_formatter = TimeWithFraction::new(LiveSplitDigitsFormat::SingleDigitSeconds, LiveSplitAccuracy::Hundredths);
        let sum_of_best_formatter = TimeWithFraction::new(LiveSplitDigitsFormat::SingleDigitSeconds, LiveSplitAccuracy::Hundredths);
        let current_pace_formatter = TimeWithFraction::new(LiveSplitDigitsFormat::SingleDigitSeconds, LiveSplitAccuracy::Hundredths);
        let timer = Timer { livesplit_timer, total_playtime_formatter, sum_of_best_formatter, current_pace_formatter };
        LiveSplit { layout, timer }
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
    pub fn start(&mut self) {
        self.reset(true);
        let _ = self.livesplit_timer.start();
        let _ = self.livesplit_timer.initialize_game_time();
    }
    pub fn split(&mut self) {
        let _ = self.livesplit_timer.split();
    }
    pub fn reset(&mut self, update_splits: bool) {
        let _ = self.livesplit_timer.reset(update_splits);
    }
    pub fn get_game_time(&self) -> f64 {
        self.livesplit_timer.snapshot().current_time().game_time.unwrap().total_seconds()
    }
    pub fn pause_game_time(&mut self) {
        let _ = self.livesplit_timer.pause_game_time();
    }
    pub fn set_game_time(&mut self, time: f64) {
        let _ = self.livesplit_timer.set_game_time(TimeSpan::from_seconds(time));
    }
    pub fn run(&mut self) -> Run<'_> {
        Run { timer: &mut self.livesplit_timer }
    }
    pub fn total_playtime_digits_format(&self) -> LiveSplitDigitsFormat {
        self.total_playtime_formatter.time.digits_format()
    }
    pub fn set_total_playtime_digits_format(&mut self, total_playtime_digits_format: LiveSplitDigitsFormat) {
        self.total_playtime_formatter.time.set_digits_format(total_playtime_digits_format);
    }
    pub fn total_playtime_accuracy(&self) -> LiveSplitAccuracy {
        self.total_playtime_formatter.fraction.accuracy()
    }
    pub fn set_total_playtime_accuracy(&mut self, accuracy: LiveSplitAccuracy) {
        self.total_playtime_formatter.fraction.set_accuracy(accuracy);
    }
    pub fn sum_of_best_digits_format(&self) -> LiveSplitDigitsFormat {
        self.sum_of_best_formatter.time.digits_format()
    }
    pub fn set_sum_of_best_digits_format(&mut self, sum_of_best_digits_format: LiveSplitDigitsFormat) {
        self.sum_of_best_formatter.time.set_digits_format(sum_of_best_digits_format);
    }
    pub fn sum_of_best_accuracy(&self) -> LiveSplitAccuracy {
        self.sum_of_best_formatter.fraction.accuracy()
    }
    pub fn set_sum_of_best_accuracy(&mut self, accuracy: LiveSplitAccuracy) {
        self.sum_of_best_formatter.fraction.set_accuracy(accuracy); 
    }
    pub fn current_pace_digits_format(&self) -> LiveSplitDigitsFormat {
        self.current_pace_formatter.time.digits_format()
    }
    pub fn set_current_pace_digits_format(&mut self, current_pace_digits_format: LiveSplitDigitsFormat) {
        self.current_pace_formatter.time.set_digits_format(current_pace_digits_format);
    }
    pub fn current_pace_accuracy(&self) -> LiveSplitAccuracy {
        self.current_pace_formatter.fraction.accuracy()
    }
    pub fn set_current_pace_accuracy(&mut self, accuracy: LiveSplitAccuracy) {
        self.current_pace_formatter.fraction.set_accuracy(accuracy);
    }
    pub fn total_playtime(&self) -> String {
        self.total_playtime_formatter.format(self.livesplit_timer.total_playtime()).to_string()
    }
    pub fn sum_of_best(&self) -> String {
        let segments = self.livesplit_timer.run().segments();
        let sum_of_best = sum_of_segments::calculate_best(segments, true, true, TimingMethod::GameTime)
            .unwrap_or(TimeSpan::zero());
        self.sum_of_best_formatter.format(sum_of_best).to_string()
    }
    pub fn current_pace(&self, comparison: Comparison) -> String {
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
        let current_pace = current_pace::calculate(&self.livesplit_timer.snapshot(), comparison);
        self.current_pace_formatter.format(current_pace.0.unwrap_or(TimeSpan::zero())).to_string()
    }
    pub fn pb_chance(&self) -> f64 {
        pb_chance::for_timer(&self.livesplit_timer.snapshot()).0 * 100.
    }
}
impl Run<'_> {
    pub fn game_name(&self) -> String {
        self.timer.run().game_name().to_owned()
    }
    pub fn set_game_info(&mut self, game: Game, category: String, new_game_glitch: NewGameGlitch) {
        let mut run = self.timer.run().clone();
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
        self.timer.set_run(run).expect("The provided `Run` contains no segments");
    }
    pub fn category_name(&self) -> String {
        self.timer.run().category_name().to_owned()
    }
    pub fn segments(&self) -> Vec<LiveSplitSegment> {
        self.timer.run().segments().to_owned()
    }
    pub fn create_segment(&mut self, name: String) {
        let mut run = self.timer.run().clone();
        run.push_segment(LiveSplitSegment::new(name));
        self.timer.set_run(run).unwrap();
    }
    pub fn attempt_count(&self) -> u32 {
        self.timer.run().attempt_count()
    }
    pub fn save_splits(&self, user_input: &str) -> Result<(), SplitsSaveError> {
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
        livesplit::save_timer(self.timer, livesplit::IoWrite(BufWriter::new(file)))
            .map_err(|e| SplitsSaveError::SaveFailed(path_display.clone(), e.to_string()))?;
        Ok(())
    }
    pub fn load_splits(&mut self, user_input: &str) -> Result<(), SplitsLoadError> {
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
        let _ = self.timer.reset(true);
        let _ = self.timer.set_run(parsed_run.run);
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
}
#[rebo::function("LiveSplit::start")]
pub fn livesplit_start() {
    LIVESPLIT_STATE.lock().unwrap().timer.start();
}
#[rebo::function("LiveSplit::split")]
pub fn livesplit_split() {
    LIVESPLIT_STATE.lock().unwrap().timer.split();
}
#[rebo::function("LiveSplit::reset")]
pub fn livesplit_reset(update_splits: bool) {
    LIVESPLIT_STATE.lock().unwrap().timer.reset(update_splits);
}
#[rebo::function("LiveSplit::get_game_time")]
pub fn livesplit_get_game_time() -> f64 {
    LIVESPLIT_STATE.lock().unwrap().timer.get_game_time()
}
#[rebo::function("LiveSplit::set_game_time")]
pub fn livesplit_set_game_time(time: f64) {
    LIVESPLIT_STATE.lock().unwrap().timer.set_game_time(time);
}
#[rebo::function("LiveSplit::pause_game_time")]
pub fn livesplit_pause_game_time() {
    LIVESPLIT_STATE.lock().unwrap().timer.pause_game_time();
}
#[rebo::function("LiveSplit::create_segment")]
pub fn livesplit_create_segment(name: String) {
    Run::create_segment(&mut Timer::run(&mut LIVESPLIT_STATE.lock().unwrap().timer), name);
}
#[rebo::function("LiveSplit::get_game_name")]
pub fn livesplit_get_game_name() -> String {
    LIVESPLIT_STATE.lock().unwrap().timer.run().game_name()
}
#[rebo::function("LiveSplit::set_game_info")]
pub fn livesplit_set_game_info(game: Game, category: String, new_game_glitch: NewGameGlitch) {
    LIVESPLIT_STATE.lock().unwrap().timer.run().set_game_info(game, category, new_game_glitch);
}
#[rebo::function("LiveSplit::get_category_name")]
pub fn livesplit_get_category_name() -> String {
    LIVESPLIT_STATE.lock().unwrap().timer.run().category_name()
}
#[rebo::function("LiveSplit::get_segments")]
pub fn livesplit_get_segments() -> Vec<Segment> {
    let mut state = LIVESPLIT_STATE.lock().unwrap();
    let mut segments = Vec::new();
    for seg in state.timer.run().segments() {
        let segment = Segment {
            name: seg.name().to_owned(),
            time: seg.split_time().game_time.unwrap_or_else(|| TimeSpan::from_seconds(999_999.)).total_seconds(),
            pb_time: seg.personal_best_split_time().game_time.unwrap_or_else(|| TimeSpan::from_seconds(999_999.)).total_seconds(),
            best_time: seg.best_segment_time().game_time.unwrap_or_else(|| TimeSpan::from_seconds(999_999.)).total_seconds(),
        };
        segments.push(segment);
    }
    segments
}
#[rebo::function("LiveSplit::get_attempt_count")]   
pub fn livesplit_get_attempt_count() -> u32 {
    LIVESPLIT_STATE.lock().unwrap().timer.run().attempt_count()
}
#[rebo::function("LiveSplit::save_splits")]
pub fn livesplit_save_splits(path: String) -> Result<(), SplitsSaveError> {
    LIVESPLIT_STATE.lock().unwrap().timer.run().save_splits(&path)
}
#[rebo::function("LiveSplit::load_splits")]
pub fn livesplit_load_splits(path: String) -> Result<(), SplitsLoadError> {
    LIVESPLIT_STATE.lock().unwrap().timer.run().load_splits(&path)
}
#[rebo::function("LiveSplit::get_color")]
pub fn livesplit_get_color(color: LiveSplitLayoutColor) -> Color {
    let mut state = LIVESPLIT_STATE.lock().unwrap();
    let settings = state.layout.general_settings_mut();
    let color = match color {
        LiveSplitLayoutColor::BestSegment => settings.best_segment_color,
        LiveSplitLayoutColor::AheadGainingTime => settings.ahead_gaining_time_color,
        LiveSplitLayoutColor::AheadLosingTime => settings.ahead_losing_time_color,
        LiveSplitLayoutColor::BehindGainingTime => settings.behind_gaining_time_color,
        LiveSplitLayoutColor::BehindLosingTime => settings.behind_losing_time_color,
        LiveSplitLayoutColor::NotRunning => settings.not_running_color,
        LiveSplitLayoutColor::PersonalBest => settings.personal_best_color,
        LiveSplitLayoutColor::Paused => settings.paused_color,
        LiveSplitLayoutColor::Text => settings.text_color,
    };
    Color { red: color.red, green: color.green, blue: color.blue, alpha: color.alpha }
}
#[rebo::function("LiveSplit::set_color")]
pub fn livesplit_set_color(layout_color: LiveSplitLayoutColor, color: Color) {
    let mut state = LIVESPLIT_STATE.lock().unwrap();
    let settings = state.layout.general_settings_mut();
    let col = match layout_color {
        LiveSplitLayoutColor::BestSegment => &mut settings.best_segment_color,
        LiveSplitLayoutColor::AheadGainingTime => &mut settings.ahead_gaining_time_color,
        LiveSplitLayoutColor::AheadLosingTime => &mut settings.ahead_losing_time_color,
        LiveSplitLayoutColor::BehindGainingTime => &mut settings.behind_gaining_time_color,
        LiveSplitLayoutColor::BehindLosingTime => &mut settings.behind_losing_time_color,
        LiveSplitLayoutColor::NotRunning => &mut settings.not_running_color,
        LiveSplitLayoutColor::PersonalBest => &mut settings.personal_best_color,
        LiveSplitLayoutColor::Paused => &mut settings.paused_color,
        LiveSplitLayoutColor::Text => &mut settings.text_color,
    };
    *col = LiveSplitColor::rgba(color.red, color.green, color.blue, color.alpha);
}
#[rebo::function("LiveSplit::get_total_playtime")]
pub fn livesplit_get_total_playtime() -> String {
    LIVESPLIT_STATE.lock().unwrap().timer.total_playtime()
}
#[rebo::function("LiveSplit::get_total_playtime_digits_format")]
pub fn livesplit_get_total_playtime_digits_format() -> DigitsFormat {
    LIVESPLIT_STATE.lock().unwrap().timer.total_playtime_digits_format().into()
}
#[rebo::function("LiveSplit::set_total_playtime_digits_format")]
pub fn livesplit_set_total_playtime_digits_format(digits_format: DigitsFormat) {
    LIVESPLIT_STATE.lock().unwrap().timer.set_total_playtime_digits_format(digits_format.into());
}
#[rebo::function("LiveSplit::get_total_playtime_accuracy")]
pub fn livesplit_get_total_playtime_accuracy() -> Accuracy {
    LIVESPLIT_STATE.lock().unwrap().timer.total_playtime_accuracy().into()
}
#[rebo::function("LiveSplit::set_total_playtime_accuracy")]
pub fn livesplit_set_total_playtime_accuracy(total_playtime_accuracy: Accuracy) {
    LIVESPLIT_STATE.lock().unwrap().timer.set_total_playtime_accuracy(total_playtime_accuracy.into());
}
#[rebo::function("LiveSplit::get_sum_of_best_segments")]
pub fn livesplit_get_sum_of_best_segments() -> String {
    LIVESPLIT_STATE.lock().unwrap().timer.sum_of_best()
}
#[rebo::function("LiveSplit::get_sum_of_best_digits_format")]
pub fn livesplit_get_sum_of_best_digits_format() -> DigitsFormat {
    LIVESPLIT_STATE.lock().unwrap().timer.sum_of_best_digits_format().into()
}
#[rebo::function("LiveSplit::set_sum_of_best_digits_format")]
pub fn livesplit_set_sum_of_best_digits_format(digits_format: DigitsFormat) {
    LIVESPLIT_STATE.lock().unwrap().timer.set_sum_of_best_digits_format(digits_format.into());
}
#[rebo::function("LiveSplit::get_sum_of_best_accuracy")]
pub fn livesplit_get_sum_of_best_accuracy() -> Accuracy {
    LIVESPLIT_STATE.lock().unwrap().timer.sum_of_best_accuracy().into()
}
#[rebo::function("LiveSplit::set_sum_of_best_accuracy")]
pub fn livesplit_set_sum_of_best_accuracy(sum_of_best_accuracy: Accuracy) {
    LIVESPLIT_STATE.lock().unwrap().timer.set_sum_of_best_accuracy(sum_of_best_accuracy.into());
}
#[rebo::function("LiveSplit::get_current_pace")]
pub fn livesplit_get_current_pace(comparison: Comparison) -> String {
    LIVESPLIT_STATE.lock().unwrap().timer.current_pace(comparison)
}
#[rebo::function("LiveSplit::get_current_pace_digits_format")]
pub fn livesplit_get_current_pace_digits_format() -> DigitsFormat {
    LIVESPLIT_STATE.lock().unwrap().timer.current_pace_digits_format().into()
}
#[rebo::function("LiveSplit::set_current_pace_digits_format")]
pub fn livesplit_set_current_pace_digits_format(digits_format: DigitsFormat) {
    LIVESPLIT_STATE.lock().unwrap().timer.set_current_pace_digits_format(digits_format.into());
}
#[rebo::function("LiveSplit::get_current_pace_accuracy")]
pub fn livesplit_get_current_pace_accuracy() -> Accuracy {
    LIVESPLIT_STATE.lock().unwrap().timer.current_pace_accuracy().into()
}
#[rebo::function("LiveSplit::set_current_pace_accuracy")]
pub fn livesplit_set_current_pace_accuracy(current_pace_accuracy: Accuracy) {
    LIVESPLIT_STATE.lock().unwrap().timer.set_current_pace_accuracy(current_pace_accuracy.into());
}
#[rebo::function("LiveSplit::get_pb_chance")]
pub fn get_pb_chance() -> f64 {
    LIVESPLIT_STATE.lock().unwrap().timer.pb_chance()
}
