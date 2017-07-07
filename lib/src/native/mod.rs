use std::sync::mpsc::TryRecvError;

#[cfg(unix)]
mod linux;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
use self::linux::*;
#[cfg(windows)]
use self::windows::*;
use error::*;
#[cfg(unix)]
use consts;
use loops::{Event, Response};
use statics::{Static, SENDER, RECEIVER};

#[cfg(unix)]
pub use self::linux::{INITIALIZE_CTOR, AController};
#[cfg(windows)]
pub use self::windows::{DllMain, AController};

struct State {
    typ: StateType,
    delta: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StateType {
    Running,
    Stopping,
}

lazy_static! {
    static ref SLATEAPP: Static<usize> = Static::new();
    static ref CONTROLLER: Static<usize> = Static::new();
    static ref STATE: Static<State> = Static::from(State { typ: StateType::Running, delta: None });
}

pub fn init() {
    #[cfg(windows)] windows::init();
    #[cfg(unix)] linux::init();
    hook_slateapp();
    hook_newgame();
    hook_tick();
    hook_controller();
}

unsafe fn set_delta(d: f64) {
    #[cfg(unix)] 
    let mut delta = consts::APP_DELTATIME as *mut u8 as *mut f64;
    #[cfg(windows)]
    let mut delta = windows::APP_DELTATIME as *mut u8 as *mut f64;
    *delta = d;
}

pub fn new_game() {
    log!("New Game detected");
    SENDER.get().send(Response::NewGame).unwrap();
}

pub unsafe fn tick_intercept() {
    if let Err(err) = tick_internal() {
        log!("Error in tick_intercept: {:?}", err);
    }
}

unsafe fn tick_internal() -> Result<()> {
    let mut state = STATE.get();
    if state.typ == StateType::Stopping {
        SENDER.get().send(Response::Stopped).chain_err(|| "Error during send")?;
    }
    loop {
        let event = match state.typ {
            StateType::Running => {
                match RECEIVER.get().try_recv() {
                    Ok(evt) => evt,
                    Err(TryRecvError::Empty) => {
                        if let Some(delta) = state.delta {
                            set_delta(delta);
                        }
                        return Ok(());
                    },
                    err => err.chain_err(|| "Receiver is disconnected")?
                }
            },
            StateType::Stopping => {
                RECEIVER.get().recv().chain_err(|| "Cannot receive")?
            },
        };
        
        match event {
            Event::Stop => {
                log!("Received stop");
                state.typ = StateType::Stopping;
                break;
            },
            Event::Step => {
                log!("Received step");
                break;
            },
            Event::Continue => {
                log!("Received continue");
                state.typ = StateType::Running;
                break;
            },
            Event::Press(key) => {
                log!("Received press {}", key);
                FSlateApplication::on_key_down(key, key as u32, false)
            },
            Event::Release(key) => {
                log!("Received release {}", key);
                FSlateApplication::on_key_up(key, key as u32, false)
            },
            Event::Mouse(x, y) => {
                log!("Received mouse {}:{}", x, y);
                FSlateApplication::on_raw_mouse_move(x, y)
            },
            Event::SetDelta(delta) => {
                log!("Received setDelta {}", delta);
                if delta == 0.0 {
                    state.delta = None;
                } else {
                    state.delta = Some(delta);
                }
            },
            Event::SetRotation(pitch, yaw, roll) => {
                log!("Received setRotation {} {} {}", pitch, yaw, roll);
                AController::set_rotation(pitch, yaw, roll);
            }
        }
    }
    if let Some(delta) = state.delta {
        set_delta(delta);
    }
    //::std::thread::sleep(::std::time::Duration::from_secs(5000));
    Ok(())
}
