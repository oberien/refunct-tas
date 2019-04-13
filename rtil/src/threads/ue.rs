use std::sync::mpsc::{Sender, Receiver, TryRecvError};

use statics::Static;
use threads::{UeToLua, LuaToUe};
use native::{
    FSlateApplication,
    unhook_fslateapplication_onkeydown,
    hook_fslateapplication_onkeydown,
    unhook_fslateapplication_onkeyup,
    hook_fslateapplication_onkeyup,
    AMyHud,
};
use native::UWorld;

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

pub fn key_down(key_code: i32, character_code: u32, is_repeat: bool) {
    handle(UeToLua::KeyDown(key_code, character_code, is_repeat));
}

pub fn key_up(key_code: i32, character_code: u32, is_repeat: bool) {
    handle(UeToLua::KeyUp(key_code, character_code, is_repeat));
}

pub fn draw_hud() {
    handle(UeToLua::DrawHud);
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
            evt @ LuaToUe::PressKey(..) | evt @ LuaToUe::ReleaseKey(..) | evt @ LuaToUe::MoveMouse(..)
                    | evt @ LuaToUe::DrawLine(..) | evt @ LuaToUe::DrawText(..) => {
                // Release STATE lock, as events can trigger a new game,
                // which needs to acquire the lock.
                drop(state);
                match evt {
                    LuaToUe::PressKey(key, code, repeat) => {
                        // we don't want to trigger our keyevent handler for emulated presses
                        unhook_fslateapplication_onkeydown();
                        FSlateApplication::press_key(key, code, repeat);
                        hook_fslateapplication_onkeydown();
                    },
                    LuaToUe::ReleaseKey(key, code, repeat) => {
                        // we don't want to trigger our keyevent handler for emulated presses
                        unhook_fslateapplication_onkeyup();
                        FSlateApplication::release_key(key, code, repeat);
                        hook_fslateapplication_onkeyup();
                    },
                    LuaToUe::MoveMouse(x, y) => FSlateApplication::move_mouse(x, y),
                    LuaToUe::DrawLine(startx, starty, endx, endy, color, thickness) =>
                        AMyHud::draw_line(startx, starty, endx, endy, color, thickness),
                    LuaToUe::DrawText(text, color, x, y, scale, scale_position) =>
                        AMyHud::draw_text(text, color, x, y, scale, scale_position),
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
            LuaToUe::SpawnActor => {
                let ptr = UWorld::spawn_pawn();
                log!("{:p}", ptr);
                UWorld::destroy_pawn(ptr);
//                let root = unsafe { *((ptr as usize + 0x168) as *const *mut crate::native::linux::character::USceneComponent) };
//                unsafe { (*root).location = crate::native::ue::FVector { x: -1000.0, y: -1000.0, z: 732.0 } };
            }
        }
    }
}
