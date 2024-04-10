use std::{ptr, thread};
use std::collections::{HashSet, HashMap, VecDeque};
use std::error::Error;
use std::sync::Mutex;
use std::time::Duration;
use std::cell::{Cell, RefCell};
use corosensei::{CoroutineResult, Yielder};

use crossbeam_channel::{Sender, Receiver};
use image::{Rgba, RgbaImage};
use once_cell::sync::Lazy;
use websocket::sync::Client;
use websocket::stream::sync::NetworkStream;

use crate::threads::{StreamToRebo, ReboToStream};
use crate::native::{AMyCharacter, FPlatformMisc, FSlateApplication, hook_fslateapplication_onkeyup, ObjectWrapper, REBO_DOESNT_START_SEMAPHORE, unhook_fslateapplication_onkeyup, UObject, UTexture2D, UWorld};
use crate::threads::ue::{Suspend, UeEvent};

mod rebo_init;

type Coroutine = corosensei::Coroutine<UeEvent, Suspend, ()>;

static STATE: Lazy<Mutex<Option<State>>> = Lazy::new(|| Mutex::new(None));

thread_local! {
    static YIELDER: Cell<*const Yielder<UeEvent, Suspend>> = Cell::new(ptr::null());
    static COROUTINE: RefCell<Option<Coroutine>> = RefCell::new(None);
}

struct State {
    is_semaphore_acquired: bool,
    event_queue: VecDeque<UeEvent>,

    new_version_string: Option<String>,
    delta: Option<f64>,
    stream_rebo_rx: Receiver<StreamToRebo>,
    rebo_stream_tx: Sender<ReboToStream>,
    working_dir: Option<String>,
    pressed_keys: HashSet<i32>,
    websocket: Option<Client<Box<dyn NetworkStream + Send>>>,
    local_time_offset: i32,
    pawns: HashMap<u32, AMyCharacter>,
    pawn_id: u32,
    minimap_texture: Option<UTexture2D>,
    minimap_image: RgbaImage,
    player_minimap_image: RgbaImage,
    // will keep textures forever, even if the player doesn't exist anymore, but each texture is only a few MB
    player_minimap_textures: HashMap<Rgba<u8>, UTexture2D>,
}

pub(super) fn poll(event: UeEvent) {
    // check if we have acquired the semaphore
    {
        let mut state = STATE.lock().unwrap();
        let state = state.as_mut().unwrap();
        if !state.is_semaphore_acquired {
            if !REBO_DOESNT_START_SEMAPHORE.try_acquire() {
                return
            }
            state.is_semaphore_acquired = true;
            state.minimap_texture = Some(UTexture2D::create(&state.minimap_image));
            log!("rebo continuing as all this* have been acquired");
        }
    }

    // check if we can resume the coroutine
    {
        enum ShouldReturn {
            No,
            Yes,
            CleanupAndYes,
        }
        let should_return = COROUTINE.with(|co| {
            let mut co = match co.try_borrow_mut() {
                Ok(co) => co,
                Err(_) => {
                    // we are currently already within the coroutine
                    // queue the event and return
                    let mut state = STATE.lock().unwrap();
                    let state = state.as_mut().unwrap();
                    log!("needed to enqueue an event, queue length {}, enqueued event {:?}", state.event_queue.len(), event);
                    state.event_queue.push_back(event.clone());
                    return ShouldReturn::Yes;
                }
            };
            if let Some(co) = co.as_mut() {
                STATE.lock().unwrap().as_mut().unwrap().event_queue.push_back(event.clone());
                while let Some(evt) = { let foo = STATE.lock().unwrap().as_mut().unwrap().event_queue.pop_front(); foo } {
                    match co.resume(evt) {
                        CoroutineResult::Yield(Suspend::Return) => (),
                        CoroutineResult::Yield(Suspend::Yield) => {
                            // don't return to UE, let input events be handled
                            FPlatformMisc::pump_messages();
                            if STATE.lock().unwrap().as_ref().unwrap().event_queue.is_empty() {
                                thread::sleep(Duration::from_millis(5));
                                STATE.lock().unwrap().as_mut().unwrap().event_queue.push_back(UeEvent::NothingHappened);
                            }
                        },
                        CoroutineResult::Return(_) => {
                            return ShouldReturn::CleanupAndYes;
                        },
                    }
                }
                return ShouldReturn::Yes;
            }
            ShouldReturn::No
        });
        match should_return {
            ShouldReturn::No => (),
            ShouldReturn::CleanupAndYes => { cleanup_after_rebo(); return },
            ShouldReturn::Yes => return,
        }
    }

    // check if we should execute new rebo code
    {
        if let Some(coroutine) = poll_tool() {
            COROUTINE.with(|co| *co.borrow_mut() = Some(coroutine));
            // actually start the coroutine
            poll(event);
            return;
        }
    }
}

pub fn init(stream_rebo_rx: Receiver<StreamToRebo>, rebo_stream_tx: Sender<ReboToStream>) {
    log!("init rebo state");
    log!("checking for a new refunct-tas release");
    let new_version = check_for_new_version();
    log!("rebo waiting until all this* have been acquired");

    const MINIMAP: &'static [u8] = include_bytes!("../../../../minimap.png");
    const PLAYER_MINIMAP: &'static [u8] = include_bytes!("../../../../player_minimap.png");
    let mut minimap_image = image::load_from_memory(MINIMAP).unwrap().to_rgba8();
    for pixel in minimap_image.pixels_mut() {
        pixel.0[3] = 100;
    }
    let player_minimap_image = image::load_from_memory(PLAYER_MINIMAP).unwrap().to_rgba8();


    *STATE.lock().unwrap() = Some(State {
        is_semaphore_acquired: false,
        event_queue: VecDeque::new(),
        new_version_string: new_version.clone(),
        delta: None,
        stream_rebo_rx,
        rebo_stream_tx,
        working_dir: None,
        pressed_keys: HashSet::new(),
        websocket: None,
        local_time_offset: 0,
        pawns: HashMap::new(),
        pawn_id: 0,
        minimap_texture: None,
        minimap_image,
        player_minimap_image,
        player_minimap_textures: HashMap::new(),
    });
}

fn poll_tool() -> Option<Coroutine> {
    while let Ok(msg) = { let foo = STATE.lock().unwrap().as_ref().unwrap().stream_rebo_rx.try_recv(); foo } {
        // can't use while let because of borrow extension of the lock
        match msg {
            StreamToRebo::Stop => {},
            StreamToRebo::WorkingDir(dir) => {
                log!("Set working dir");
                STATE.lock().unwrap().as_mut().unwrap().working_dir = Some(dir);
                log!("Working dir set");
            }
            StreamToRebo::Start(filename, code) => {
                log!("Starting rebo...");
                return Some(Coroutine::new(|yielder, _| {
                    YIELDER.with(|y| y.set(yielder as *const _));
                    let rebo_stream_tx = STATE.lock().unwrap().as_ref().unwrap().rebo_stream_tx.clone();
                    let config = rebo_init::create_config(rebo_stream_tx);
                    log!("Executing rebo code.");
                    rebo::run_with_config(filename, code, config);
                    log!("Rebo execution done.");
                }));
            }
        }
    }
    None
}

fn cleanup_after_rebo() {
    log!("Starting rebo cleanup...");
    // reset STATE
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    YIELDER.with(|yielder| yielder.set(ptr::null()));
    COROUTINE.with(|co| *co.borrow_mut() = None);
    state.event_queue.clear();
    state.delta = None;
    drop(state.websocket.take());
    for (_id, my_character) in state.pawns.drain() {
        let character = unsafe { ObjectWrapper::new(my_character.as_ptr() as *mut UObject) };
        UWorld::destroy_amycharacter(character.as_ptr());
    }
    state.pawn_id = 0;
    // we don't want to trigger our keyevent handler for emulated presses
    unhook_fslateapplication_onkeyup();
    for key in state.pressed_keys.drain() {
        FSlateApplication::release_key(key, key as u32, false);
    }
    hook_fslateapplication_onkeyup();
    rebo_init::apply_map_internal(&rebo_init::ORIGINAL_MAP);
    state.rebo_stream_tx.send(ReboToStream::MiDone).unwrap();
    log!("Cleanup finished.");
}

fn check_for_new_version() -> Option<String> {
    const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
    let res = ureq::AgentBuilder::new()
        .redirects(0)
        .timeout(Duration::from_secs(3))
        .build()
        .get("https://github.com/oberien/refunct-tas/releases/latest")
        .call();
    match res {
        Ok(response) => {
            assert_eq!(response.status(), 302);
            let loc = response.header("Location").unwrap();
            let pos = loc.rfind("/v").unwrap();
            let version = &loc[pos+2..];
            if version != CURRENT_VERSION {
                let new_version = format!("New version available: v{CURRENT_VERSION} -> v{version}");
                log!("VERSION: {new_version}");
                Some(new_version)
            } else {
                log!("VERSION: rtil version v{CURRENT_VERSION} is up to date");
                None
            }
        },
        Err(err) => {
            log!("VERSION: Error checking for new version: err");
            match err {
                ureq::Error::Status(status, _) => {
                    Some(format!("Error checking for new version: Got status {status}"))
                },
                ureq::Error::Transport(transport) => {
                    let kind = transport.kind().to_string();
                    let message = transport.message().map(|m| format!(": {m}"));
                    let source = transport.source().map(|s| format!(": {s}"));
                    let mut res = kind;
                    if let Some(message) = message {
                        res += &message;
                    }
                    if let Some(source) = source {
                        res += &source;
                    }
                    Some(format!("Error checking for new version: {res}"))
                }
            }
        }
    }
}
