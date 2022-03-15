use crossbeam_channel::{Sender, Receiver, TryRecvError};
use once_cell::sync::Lazy;

use crate::statics::Static;
use crate::threads::{UeToRebo, ReboToUe};
use crate::native::{
    FSlateApplication,
    unhook_fslateapplication_onkeydown,
    hook_fslateapplication_onkeydown,
    unhook_fslateapplication_onkeyup,
    hook_fslateapplication_onkeyup,
    AMyHud,
    UWorld,
};

static STATE: Lazy<Static<State>> = Lazy::new(Static::new);

struct State {
    typ: StateType,
    rebo_ue_rx: Receiver<ReboToUe>,
    ue_rebo_tx: Sender<UeToRebo>,
}

#[derive(PartialEq, Eq)]
enum StateType {
    Running,
    Stopping,
}

pub fn run(rebo_ue_rx: Receiver<ReboToUe>, ue_rebo_tx: Sender<UeToRebo>) {
    log!("\"starting\" ue thread");
    STATE.set(State {
        typ: StateType::Running,
        rebo_ue_rx,
        ue_rebo_tx,
    });
}

pub fn new_game() {
    log!("New Game");
    handle(UeToRebo::NewGame)
}

pub fn tick() {
    handle(UeToRebo::Tick);
}

pub fn key_down(key_code: i32, character_code: u32, is_repeat: bool) {
    handle(UeToRebo::KeyDown(key_code, character_code, is_repeat));
}

pub fn key_up(key_code: i32, character_code: u32, is_repeat: bool) {
    handle(UeToRebo::KeyUp(key_code, character_code, is_repeat));
}

pub fn draw_hud() {
    handle(UeToRebo::DrawHud);
}

fn handle(event: UeToRebo) {
    // not yet initialized
    if STATE.is_none() {
        return
    }

    let mut state = STATE.get();

    if state.typ == StateType::Running {
        match state.rebo_ue_rx.try_recv() {
            Ok(ReboToUe::Stop) => state.typ = StateType::Stopping,
            Err(TryRecvError::Empty) => return,
            val => {
                log!("Error rebo_ue_rx.try_recv: {:?}", val);
                panic!();
            }
        }
    } else {
        state.ue_rebo_tx.send(event).unwrap();
    }

    loop {
        match state.rebo_ue_rx.recv().unwrap() {
            ReboToUe::Stop => {
                log!("Got ReboToUe::Stop, but state is Stopping");
                panic!()
            }
            evt @ ReboToUe::PressKey(..)
            | evt @ ReboToUe::ReleaseKey(..)
            | evt @ ReboToUe::MoveMouse(..)
            | evt @ ReboToUe::DrawLine(..)
            | evt @ ReboToUe::DrawText(..)
            | evt @ ReboToUe::SpawnAMyCharacter => {
                // Release STATE lock, as events can trigger a new game,
                // which needs to acquire the lock.
                drop(state);
                match evt {
                    ReboToUe::PressKey(key, code, repeat) => {
                        // we don't want to trigger our keyevent handler for emulated presses
                        unhook_fslateapplication_onkeydown();
                        FSlateApplication::press_key(key, code, repeat);
                        hook_fslateapplication_onkeydown();
                    },
                    ReboToUe::ReleaseKey(key, code, repeat) => {
                        // we don't want to trigger our keyevent handler for emulated presses
                        unhook_fslateapplication_onkeyup();
                        FSlateApplication::release_key(key, code, repeat);
                        hook_fslateapplication_onkeyup();
                    },
                    ReboToUe::MoveMouse(x, y) => FSlateApplication::move_mouse(x, y),
                    ReboToUe::DrawLine(startx, starty, endx, endy, color, thickness) =>
                        AMyHud::draw_line(startx, starty, endx, endy, color, thickness),
                    ReboToUe::DrawText(text, color, x, y, scale, scale_position) =>
                        AMyHud::draw_text(text, color, x, y, scale, scale_position),
                    ReboToUe::SpawnAMyCharacter => {
                        let my_character = UWorld::spawn_amycharacter();
                        STATE.get().ue_rebo_tx.send(UeToRebo::AMyCharacterSpawned(my_character)).unwrap();
                    },
                    _ => unreachable!()
                }
                state = STATE.get();
            },
            ReboToUe::Resume => {
                log!("Resuming");
                state.typ = StateType::Running;
                break;
            },
            ReboToUe::AdvanceFrame => break,
        }
    }
}
