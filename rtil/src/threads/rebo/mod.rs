use std::thread;
use std::collections::{HashSet, HashMap};
use std::sync::Mutex;

use crossbeam_channel::{Sender, Receiver};
use image::{Rgba, RgbaImage};
use once_cell::sync::Lazy;
use websocket::sync::Client;
use websocket::stream::sync::NetworkStream;

use crate::threads::{StreamToRebo, ReboToStream, ReboToUe, UeToRebo};
use crate::native::{AMyCharacter, REBO_DOESNT_START_SEMAPHORE, UTexture2D, UWorld};

mod rebo_init;

static STATE: Lazy<Mutex<Option<State>>> = Lazy::new(|| Mutex::new(None));

struct State {
    delta: Option<f64>,
    stream_rebo_rx: Receiver<StreamToRebo>,
    rebo_stream_tx: Sender<ReboToStream>,
    rebo_ue_tx: Sender<ReboToUe>,
    ue_rebo_rx: Receiver<UeToRebo>,
    working_dir: Option<String>,
    pressed_keys: HashSet<i32>,
    websocket: Option<Client<Box<dyn NetworkStream + Send>>>,
    local_time_offset: i32,
    pawns: HashMap<u32, AMyCharacter>,
    pawn_id: u32,
    minimap_texture: UTexture2D,
    minimap_image: RgbaImage,
    player_minimap_image: RgbaImage,
    // will keep textures forever, even if the player doesn't exist anymore, but each texture is only a few MB
    player_minimap_textures: HashMap<Rgba<u8>, UTexture2D>,
}

pub fn run(stream_rebo_rx: Receiver<StreamToRebo>, rebo_stream_tx: Sender<ReboToStream>,
           rebo_ue_tx: Sender<ReboToUe>, ue_rebo_rx: Receiver<UeToRebo>) {
    log!("starting rebo thread");
    thread::spawn(move || {
        log!("rebo thread waiting until all this* have been acquired");
        REBO_DOESNT_START_SEMAPHORE.acquire();
        log!("rebo thread continuing as all this* have been acquired");

        const MINIMAP: &'static [u8] = include_bytes!("../../../minimap.png");
        const PLAYER_MINIMAP: &'static [u8] = include_bytes!("../../../player_minimap.png");

        let mut minimap_image = image::load_from_memory(MINIMAP).unwrap().to_rgba8();
        for pixel in minimap_image.pixels_mut() {
            pixel.0[3] = 100;
        }
        let player_minimap_image = image::load_from_memory(PLAYER_MINIMAP).unwrap().to_rgba8();

        *STATE.lock().unwrap() = Some(State {
            delta: None,
            stream_rebo_rx,
            rebo_stream_tx,
            rebo_ue_tx,
            ue_rebo_rx,
            working_dir: None,
            pressed_keys: HashSet::new(),
            websocket: None,
            local_time_offset: 0,
            pawns: HashMap::new(),
            pawn_id: 0,
            minimap_texture: UTexture2D::create(&minimap_image),
            minimap_image,
            player_minimap_image,
            player_minimap_textures: HashMap::new(),
        });

        loop {
            handle_rx();
        }
    });
}

fn handle_rx() {
    let res = STATE.lock().unwrap().as_ref().unwrap().stream_rebo_rx.recv().unwrap();
    match res {
        StreamToRebo::Stop => {},
        StreamToRebo::WorkingDir(dir) => {
            log!("Set working dir");
            STATE.lock().unwrap().as_mut().unwrap().working_dir = Some(dir);
        }
        StreamToRebo::Start(filename, code) => {
            log!("Starting rebo...");
            log!("Cleaning ue_rebo_rx...");
            let mut count = 0;
            while let Ok(_) = STATE.lock().unwrap().as_ref().unwrap().ue_rebo_rx.try_recv() {
                count += 1;
            }
            log!("Removed {} messages", count);

            STATE.lock().unwrap().as_ref().unwrap().rebo_ue_tx.send(ReboToUe::Stop).unwrap();
            let rebo_stream_tx = STATE.lock().unwrap().as_ref().unwrap().rebo_stream_tx.clone();
            let config = rebo_init::create_config(rebo_stream_tx);
            log!("Executing rebo code.");
            rebo::run_with_config(filename, code, config);
            log!("Rebo execution done. Starting cleanup...");

            // reset STATE
            let mut state = STATE.lock().unwrap();
            let state = state.as_mut().unwrap();
            state.delta = None;
            state.websocket.take();
            for (_id, my_character) in state.pawns.drain() {
                UWorld::destroy_amycharaccter(my_character);
            }
            state.pawns.clear();
            state.pawn_id = 0;
            for key in state.pressed_keys.drain() {
                state.rebo_ue_tx.send(ReboToUe::ReleaseKey(key, key as u32, false)).unwrap();
            }

            state.rebo_ue_tx.send(ReboToUe::Resume).unwrap();
            state.rebo_stream_tx.send(ReboToStream::MiDone).unwrap();
            log!("Cleanup finished.");
        }
    }
}
