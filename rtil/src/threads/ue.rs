use std::sync::mpsc::{Sender, Receiver, TryRecvError};

use statics::Static;
use threads::{UeToLua, LuaToUe};
use native::FSlateApplication;

lazy_static! {
    static ref STATE: Static<State> = Static::new();
}

struct State {
    typ: StateType,
    lua_ue_rx: Receiver<LuaToUe>,
    ue_lua_tx: Sender<UeToLua>,
}

#[derive(PartialEq, Eq)]
enum StateType {
    Running,
    Stopping,
}

pub fn run(lua_ue_rx: Receiver<LuaToUe>, ue_lua_tx: Sender<UeToLua>) {
    STATE.set(State {
        typ: StateType::Running,
        lua_ue_rx,
        ue_lua_tx,
    });
}

pub fn new_game() {
    log!("New Game");
    handle(UeToLua::NewGame)
}

pub fn tick() {
    handle(UeToLua::Tick);
}

fn handle(event: UeToLua) {
    // not yet initialized
    if STATE.is_none() {
        return
    }

    let mut state = STATE.get();

    if state.typ == StateType::Running {
        match state.lua_ue_rx.try_recv() {
            Ok(LuaToUe::Stop) => state.typ = StateType::Stopping,
            Err(TryRecvError::Empty) => return,
            val => {
                log!("Error lua_ue_rx.try_recv: {:?}", val);
                panic!();
            }
        }
    } else {
        state.ue_lua_tx.send(event).unwrap();
    }

    loop {
        match state.lua_ue_rx.recv().unwrap() {
            LuaToUe::Stop => {
                log!("Got LuaToUe::Stop, but state is Stopping");
                panic!()
            }
            evt @ LuaToUe::PressKey(_) | evt @ LuaToUe::ReleaseKey(_) | evt @ LuaToUe::MoveMouse(..) => {
                // Release STATE lock, as events can trigger a new game,
                // which needs to acquire the lock.
                drop(state);
                match evt {
                    LuaToUe::PressKey(key) => FSlateApplication::press_key(key),
                    LuaToUe::ReleaseKey(key) => FSlateApplication::release_key(key),
                    LuaToUe::MoveMouse(x, y) => FSlateApplication::move_mouse(x, y),
                    _ => unreachable!()
                }
                state = STATE.get();
            },
            LuaToUe::Resume => {
                log!("Resuming");
                state.typ = StateType::Running;
                break;
            },
            LuaToUe::AdvanceFrame => break,
        }
    }
}
