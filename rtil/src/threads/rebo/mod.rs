use std::thread;
use std::collections::{HashSet, HashMap};
use std::net::TcpStream;
use std::sync::Mutex;

use protocol::Message;
use crossbeam_channel::{Sender, Receiver};

use threads::{StreamToRebo, ReboToStream, ReboToUe, UeToRebo, Config};
use native::AMyCharacter;

mod rebo_init;

lazy_static! {
    static ref STATE: Mutex<Option<State>> = Mutex::new(None);
}

struct State {
    delta: Option<f64>,
    stream_rebo_rx: Receiver<StreamToRebo>,
    rebo_stream_tx: Sender<ReboToStream>,
    rebo_ue_tx: Sender<ReboToUe>,
    ue_rebo_rx: Receiver<UeToRebo>,
    config: Config,
    working_dir: Option<String>,
    pressed_keys: HashSet<i32>,
    tcp_stream: Option<(TcpStream, Receiver<Message>)>,
    pawns: HashMap<u32, AMyCharacter>,
    pawn_id: u32,
}

pub fn run(stream_rebo_rx: Receiver<StreamToRebo>, rebo_stream_tx: Sender<ReboToStream>,
           rebo_ue_tx: Sender<ReboToUe>, ue_rebo_rx: Receiver<UeToRebo>) {
    log!("starting rebo thread");
    thread::spawn(move || {
        *STATE.lock().unwrap() = Some(State {
            delta: None,
            stream_rebo_rx,
            rebo_stream_tx,
            rebo_ue_tx,
            ue_rebo_rx,
            config: Config::default(),
            working_dir: None,
            pressed_keys: HashSet::new(),
            tcp_stream: None,
            pawns: HashMap::new(),
            pawn_id: 0,
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
        StreamToRebo::Config(config) => {
            log!("Set config before running");
            STATE.lock().unwrap().as_mut().unwrap().config = config;
        },
        StreamToRebo::WorkingDir(dir) => {
            log!("Set working dir");
            STATE.lock().unwrap().as_mut().unwrap().working_dir = Some(dir);
        }
        StreamToRebo::Start(s) => {
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
            rebo::run_with_config("file.re".to_string(), s, config);
            log!("Rebo execution done. Starting cleanup...");

            // reset STATE
            let mut state = STATE.lock().unwrap();
            let state = state.as_mut().unwrap();
            state.delta = None;
            state.tcp_stream.take();
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
