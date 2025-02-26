use livesplit_core::{Segment, TimeSpan, TimingMethod};
use crate::threads::ue::rebo::STATE;

pub struct LiveSplit {
    pub timer: livesplit_core::Timer,
}
pub struct Timer {}

impl LiveSplit {
    pub fn new() -> LiveSplit {
        let mut run = livesplit_core::Run::new();
        run.set_game_name("Refunct");
        run.set_category_name("Any%");
        run.push_segment(Segment::new("1"));
        run.metadata_mut().set_speedrun_com_variable("New Game Glitch", "Normal");
        let mut timer = livesplit_core::Timer::new(run).unwrap();
        timer.set_current_timing_method(TimingMethod::GameTime);
        LiveSplit {
            timer,
        }
    }
}
impl Timer {
    pub fn start() {
        Self::reset(true);
        STATE.lock().unwrap().as_mut().unwrap().livesplit_state.timer.start();
        STATE.lock().unwrap().as_mut().unwrap().livesplit_state.timer.initialize_game_time();
    }
    pub fn split() {
        STATE.lock().unwrap().as_mut().unwrap().livesplit_state.timer.split();
    }
    pub fn reset(update_splits: bool) {
        STATE.lock().unwrap().as_mut().unwrap().livesplit_state.timer.reset(update_splits);
    }
    pub fn get_game_time() -> f64 {
        STATE.lock().unwrap().as_mut().unwrap().livesplit_state.timer.snapshot().current_time().game_time.unwrap().total_seconds()
    }
    pub fn pause_game_time() {
        STATE.lock().unwrap().as_mut().unwrap().livesplit_state.timer.pause_game_time();
    }
    pub fn set_game_time(time: f64) {
        STATE.lock().unwrap().as_mut().unwrap().livesplit_state.timer.set_game_time(TimeSpan::from_seconds(time));
    }
}
