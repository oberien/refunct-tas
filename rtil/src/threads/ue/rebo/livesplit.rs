use std::{fs, fs::File};
use std::collections::HashSet;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use livesplit_core::{Segment, TimeSpan, TimingMethod, Timer as LiveSplitTimer, Run as LiveSplitRun, run::{saver::livesplit::{IoWrite, save_timer}, parser::composite}};
use once_cell::sync::Lazy;

static LIVESPLIT_STATE: Lazy<Mutex<LiveSplit>> = Lazy::new(|| Mutex::new(LiveSplit::new()));
static VALID_SPLITS: Lazy<Mutex<HashSet<PathBuf>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub struct LiveSplit {
    pub timer: LiveSplitTimer,
}
pub struct Timer {}
pub struct Run {}

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
    /// filename, io::Error.to_string()
    CreationFailed(String, String),
    /// filename, io::Error.to_string()
    SaveFailed(String, String),
    /// path, io::Error.to_string()
    DisallowedFilePath(String, String),
}
#[derive(rebo::ExternalType)]
pub enum SplitsLoadError {
    /// filename, io::Error.to_string()
    OpenFailed(String, String),
    /// filename, io::Error.to_string()
    ParseFailed(String, String),
}

impl LiveSplit {
    pub fn new() -> LiveSplit {
        let mut run = LiveSplitRun::new();
        run.set_game_name("Refunct");
        run.set_category_name("Any%");
        run.push_segment(Segment::new("1"));
        run.metadata_mut().set_speedrun_com_variable("New Game Glitch", "Normal");
        let mut timer = LiveSplitTimer::new(run).unwrap();
        timer.set_current_timing_method(TimingMethod::GameTime);
        LiveSplit { timer }
    }
    fn splits_path() -> PathBuf {
        let appdata_path = crate::threads::ue::rebo::rebo_init::data_path();
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
    pub fn save_splits(path: impl AsRef<Path>) -> Result<(), SplitsSaveError> {
        let state = LIVESPLIT_STATE.lock().unwrap();
        let path = path.as_ref();
        let filename = path
            .file_name().expect("Could not get filename")
            .to_str().expect("Could not convert filename to &str")
            .to_owned();
        let splits_path = LiveSplit::splits_path();
        let parent = match path.is_absolute() {
            true => path.parent().expect("Could not get parent directory")
                .to_str().expect("Could not convert parent dir to &str"),
            false => splits_path
                .to_str().expect("Could not convert filename to &str"),
        };
        let filename = sanitize_filename::sanitize(filename);
        let path = match path.is_relative() {
            true => LiveSplit::splits_path().join(&filename),
            false => path.to_path_buf(),
        };
        if parent != LiveSplit::splits_path().to_str().unwrap() && !VALID_SPLITS.lock().unwrap().contains(&path.to_path_buf()) {
            return Err(SplitsSaveError::DisallowedFilePath(filename, "Disallowed file path".to_owned()));
        }
        let file = match File::create(path) {
            Ok(file) => file,
            Err(e) => return Err(SplitsSaveError::CreationFailed(filename, e.to_string())),
        };
        let writer = BufWriter::new(file);
        match save_timer(&state.timer, IoWrite(writer)) {
            Ok(_) => (),
            Err(e) => return Err(SplitsSaveError::SaveFailed(filename, e.to_string())),
        };
        Ok(())
    }
    pub fn load_splits(path: impl AsRef<Path>) -> Result<(), SplitsLoadError> {
        let filename = path
            .as_ref()
            .file_name().expect("Could not get filename")
            .to_str().expect("Could not convert filename to &str")
            .to_owned();
        let file = match fs::read(path.as_ref()) {
            Ok(file) => file,
            Err(e) => return Err(SplitsLoadError::OpenFailed(filename.to_owned(), e.to_string())),
        };
        let parsed_run = match composite::parse(&file, None) {
            Ok(parsed_run) => parsed_run,
            Err(e) => return Err(SplitsLoadError::ParseFailed(filename.to_owned(), e.to_string())),
        };
        let mut state = LIVESPLIT_STATE.lock().unwrap();
        state.timer.reset(true);
        state.timer.set_run(parsed_run.run).unwrap();
        VALID_SPLITS.lock().unwrap().to_owned().insert(path.as_ref().to_path_buf());
        Ok(())
    }
}
