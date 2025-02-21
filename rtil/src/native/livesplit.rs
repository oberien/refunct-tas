use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Mutex;
use livesplit_core::{Layout, Segment, Timer};
use livesplit_core::run::parser::composite;
use livesplit_core::run::saver::livesplit::IoWrite;
use once_cell::sync::Lazy;
use speedrun_api::api::leaderboards::FullGameLeaderboard;
use speedrun_api::api::variables::Variable;
use speedrun_api::SpeedrunApiBuilder;
use speedrun_api::types::{Category, Game};

pub struct LiveSplit {
    layout: Layout,
    run: livesplit_core::Run,
}
pub struct Run {}
pub struct _Timer {}

static LIVESPLIT_STATE: Lazy<Mutex<LiveSplit>> = Lazy::new(|| Mutex::new(LiveSplit::new()));

impl LiveSplit {
    pub fn new() -> LiveSplit {
        LiveSplit {
            layout: Layout::new(),
            run: Run::new(),
        }
    }

    pub fn create_layout() -> Layout {
        Layout::new()
    }

    pub fn open_layout(path: String) -> Layout {
        livesplit_core::layout::parser::parse(path.as_str()).expect("Failed to parse layout")
    }

    pub fn add_component(comp: livesplit_core::Component) {
        LIVESPLIT_STATE.lock().unwrap().layout.components.push(comp);
    }

    pub fn remove_component(index: usize) {
        LIVESPLIT_STATE.lock().unwrap().layout.components.remove(index);
    }

    pub fn set_layout(layout: Layout) {
        LIVESPLIT_STATE.lock().unwrap().layout = layout;
    }
}

impl Run {
    pub fn new() -> livesplit_core::Run {
        livesplit_core::Run::new()
    }
    pub fn attempt_count() -> u32 {
        LIVESPLIT_STATE.lock().unwrap().run.attempt_count()
    }
    pub fn set_attempt_count(count: u32) {
        LIVESPLIT_STATE.lock().unwrap().run.set_attempt_count(count);
    }
    pub fn category_name() -> String {
        LIVESPLIT_STATE.lock().unwrap().run.category_name().to_string()
    }
    pub fn set_category_name(name: String) {
        LIVESPLIT_STATE.lock().unwrap().run.set_category_name(name);
    }
    pub fn segments() -> Vec<Segment> {
        let mut segments = Vec::new();
        for seg in LIVESPLIT_STATE.lock().unwrap().run.segments() {
            segments.push(seg.clone());
        }
        segments
    }
    pub fn fix_splits() {
        LIVESPLIT_STATE.lock().unwrap().run.fix_splits();
    }
    pub fn open_splits(file: String) -> livesplit_core::Run {
        let path = Path::new(&file);
        let file = fs::read(path).expect("Failed reading the file.");
        let result = composite::parse(&file, Some(path));
        let parsed = result.expect("Not a valid splits file.");
        parsed.run
    }
    pub fn save_splits(run: livesplit_core::Run, file: String) {
        let file = File::create(file);
        let writer = BufWriter::new(file.expect("Failed creating the file"));
        livesplit_core::run::saver::livesplit::save_run(&run, IoWrite(writer)).expect("Couldn't save the splits file");
    }
    pub fn world_record(category: String, variable: String) -> String {
        let client = SpeedrunApiBuilder::new().build();
        let cat = match category.as_str() {
            "Any%" => "w20pq58k",
            "100%" => "wdmz5752",
            "Low%" => "zdnxyjx2",
            "All Cubes" => "ndx8zvjk",
            _ => "UNKNOWN",
        };
        let var_choice = match cat {
            "w20pq58k" => match variable.as_str() {
                "Normal" => "21d6v8pq",
                "New Game Glitch" => "klrxjkol",
                _ => "UNKNOWN",
            },
            "wdmz5752" => match variable.as_str() {
                "Normal" => "21d6v8pq",
                "New Game Glitch" => "klrxjkol",
                _ => "UNKNOWN",
            },
        };
        if cat == "UNKNOWN" || var_choice == "UNKNOWN" || var == "UNKNOWN" {
            return "Unknown World Record".to_string();
        }
        let endpoint = FullGameLeaderboard::builder()
            .game("xldev513") // Refunct
            .category(cat)
            .variable(variable)
            .build()
            .unwrap();
        let leaderboard: speedrun_api::types::Leaderboard = endpoint.query_async(&client).await?;
    }
}

impl _Timer {
    pub fn start(mut timer: Timer) {
        timer.run();
    }
    pub fn pause(mut timer: Timer, pause: bool) {
        match pause {
            true => timer.pause(),
            false => timer.resume()
        }
    }
    pub fn split(mut timer: Timer) {
        timer.split();
    }
    pub fn reset(mut timer: Timer, update_splits: bool) {
        timer.reset(update_splits);
    }
}