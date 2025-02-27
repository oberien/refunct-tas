use std::sync::Mutex;
use livesplit_core::{Segment, TimeSpan, TimingMethod, Timer as LiveSplitTimer, Run as LiveSplitRun};
use once_cell::sync::Lazy;

static LIVESPLIT_STATE: Lazy<Mutex<LiveSplit>> = Lazy::new(|| Mutex::new(LiveSplit::new()));

pub struct LiveSplit {
    pub timer: LiveSplitTimer,
}
pub struct Timer {}
pub struct Run {}

#[derive(rebo::ExternalType)]
pub enum Games {
    Refunct,
    RefunctCategoryExtensions,
    RefunctMultiplayer,
    RefunctRandomizer,
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
}
impl Timer {
    pub fn start() {
        Self::reset(true);
        LIVESPLIT_STATE.lock().unwrap().timer.start();
        LIVESPLIT_STATE.lock().unwrap().timer.initialize_game_time();
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
        LIVESPLIT_STATE.lock().unwrap().timer.run().clone().game_name().to_string()
    }
    pub fn set_game_name(game: Games) {
        let state = LIVESPLIT_STATE.lock().unwrap();
        let mut run = state.timer.run().clone();
        let game = match game {
            Games::Refunct => "Refunct",
            Games::RefunctCategoryExtensions => "Refunct Category Extensions",
            Games::RefunctMultiplayer => "Refunct Multiplayer",
            Games::RefunctRandomizer => "Refunct Randomizer",
        };
        run.set_game_name(game);
        LIVESPLIT_STATE.lock().unwrap().timer.set_run(run).unwrap();
    }
    pub fn category_name() -> String {
        LIVESPLIT_STATE.lock().unwrap().timer.run().clone().category_name().to_string()
    }
    pub fn set_category_name(category_name: String) {
        let state = LIVESPLIT_STATE.lock().unwrap();
        let mut run = state.timer.run().clone();
        run.set_category_name(category_name);
        LIVESPLIT_STATE.lock().unwrap().timer.set_run(run).unwrap();
    }
    pub fn get_segments() -> Vec<Segment> {
        Vec::from(LIVESPLIT_STATE.lock().unwrap().timer.run().segments())
    }
    pub fn create_segment(name: String) {
        let mut state = LIVESPLIT_STATE.lock().unwrap();
        let mut run = state.timer.run().clone();
        run.push_segment(Segment::new(name));
        state.timer.set_run(run).unwrap();
    }
    pub fn remove_segment(index: i32) {
        let state = LIVESPLIT_STATE.lock().unwrap();
        let mut run = state.timer.run().clone();
        if run.segments().len() < 2 {
            return;
        }
        run.segments_mut().remove(index as usize);
        LIVESPLIT_STATE.lock().unwrap().timer.set_run(run).unwrap();
    }
    pub fn attempt_count() -> u32 {
        LIVESPLIT_STATE.lock().unwrap().timer.run().clone().attempt_count()
    }
    pub fn set_attempt_count(count: u32) {
        let mut state = LIVESPLIT_STATE.lock().unwrap();
        let mut run = state.timer.run().clone();
        run.set_attempt_count(count);
        state.timer.set_run(run).unwrap();
    }
}
