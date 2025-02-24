use std::sync::Mutex;
use livesplit_core::{Segment, TimeSpan, Timer, TimingMethod};
use once_cell::sync::Lazy;

static LIVESPLIT_STATE: Lazy<Mutex<LiveSplit>> = Lazy::new(|| Mutex::new(LiveSplit::new()));

pub struct LiveSplit {
    pub run: livesplit_core::Run,
    pub timer: Timer,
}
pub struct _Timer {}

impl LiveSplit {
    pub fn new() -> LiveSplit {
        let mut run = livesplit_core::Run::new();
        run.clone().set_game_name("Refunct");
        run.clone().set_category_name("Any%");
        run.push_segment(Segment::new("1"));
        run.metadata_mut().set_speedrun_com_variable("New Game Glitch", "Normal");
        let mut timer = Timer::new(run.clone()).unwrap();
        timer.set_current_timing_method(TimingMethod::GameTime);
        LiveSplit {
            run,
            timer,
        }
    }
}
impl _Timer {
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
