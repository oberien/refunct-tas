use std::sync::mpsc::TryRecvError;

use libc;

#[cfg(unix)]
mod linux;
#[cfg(windows)]
mod windows;


#[cfg(unix)]
use self::linux::*;
#[cfg(windows)]
use self::windows::*;
use error::*;
use consts;
use loops::{Event, Response};
use statics::{Static, SENDER, RECEIVER};

#[cfg(unix)]
pub use self::linux::INITIALIZE_CTOR;

struct State {
    typ: StateType,
    delta: Option<f64>,
}

enum StateType {
    Running,
    Stopping,
}

lazy_static! {
    static ref SLATEAPP: Static<usize> = Static::new();
    static ref STATE: Static<State> = Static::from(State { typ: StateType::Running, delta: None });
}

pub fn init() {
    hook_slateapp();
    hook_newgame();
    hook_tick();
}

pub struct FSlateApplication;

impl FSlateApplication {
    pub unsafe fn on_key_down(key_code: i32, character_code: u32, is_repeat: bool) {
        let fun: unsafe extern fn(this: libc::uintptr_t, key_code: libc::int32_t, character_code: libc::uint32_t, is_repeat: libc::uint32_t) = ::std::mem::transmute(consts::FSLATEAPPLICATION_ONKEYDOWN);
        fun(*SLATEAPP.get(), key_code, character_code, is_repeat as u32)
    }
    pub unsafe fn on_key_up(key_code: i32, character_code: u32, is_repeat: bool) {
        let fun: unsafe extern fn(this: libc::uintptr_t, key_code: libc::int32_t, character_code: libc::uint32_t, is_repeat: libc::uint32_t) = ::std::mem::transmute(consts::FSLATEAPPLICATION_ONKEYUP);
        fun(*SLATEAPP.get(), key_code, character_code, is_repeat as u32)
    }

    pub unsafe fn on_raw_mouse_move(x: i32, y: i32) {
        let fun: unsafe extern fn(this: libc::uintptr_t, x: libc::int32_t, y: libc::int32_t) = ::std::mem::transmute(consts::FSLATEAPPLICATION_ONRAWMOUSEMOVE);
        fun(*SLATEAPP.get(), x, y)
    }
}

unsafe fn set_delta(d: f64) {
    let mut delta = consts::APP_DELTATIME as *mut u8 as *mut f64;
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
    loop {
        let event = match state.typ {
            StateType::Running => {
                match RECEIVER.get().try_recv() {
                    Ok(evt) => evt,
                    Err(TryRecvError::Empty) => {
                        if let Some(delta) = state.delta {
                            set_delta(delta);
                        }
                        break;
                    },
                    err => err.chain_err(|| "Receiver is disconnected")?
                }
            },
            StateType::Stopping => {
                SENDER.get().send(Response::Stopped).chain_err(|| "Error during send")?;
                RECEIVER.get().recv().chain_err(|| "Cannot receive")?
            },
        };
        
        match event {
            Event::Stop => {
                log!("Received stop");
                state.typ = StateType::Stopping
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
        }
    }
    if let Some(delta) = state.delta {
        set_delta(delta);
    }
    Ok(())
}
// TODO: detect game starts
