use crossbeam_channel::{Sender, Receiver, TryRecvError};
use once_cell::sync::Lazy;

use crate::statics::Static;
use crate::threads::{UeToRebo, ReboToUe};
use crate::native::{FSlateApplication, unhook_fslateapplication_onkeydown, hook_fslateapplication_onkeydown, unhook_fslateapplication_onkeyup, hook_fslateapplication_onkeyup, unhook_fslateapplication_onrawmousemove, hook_fslateapplication_onrawmousemove, AMyHud, UWorld, FPlatformMisc, UMyGameInstance};

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

pub fn mouse_move(x: i32, y: i32) {
    handle(UeToRebo::MouseMove(x, y));
}

pub fn draw_hud() {
    handle(UeToRebo::DrawHud);
}

pub fn apply_resolution_settings() {
    handle(UeToRebo::ApplyResolutionSettings);
}

pub fn add_to_screen() {
    handle(UeToRebo::AddToScreen);
}

fn handle(event: UeToRebo) {
    // not yet initialized
    if STATE.is_none() {
        return
    }


    if STATE.get().typ == StateType::Running {
        let msg = STATE.get().rebo_ue_rx.try_recv();
        match msg {
            Ok(ReboToUe::Stop) => STATE.get().typ = StateType::Stopping,
            Err(TryRecvError::Empty) => return,
            val => {
                log!("Error rebo_ue_rx.try_recv: {:?}", val);
                panic!();
            }
        }
    } else {
        STATE.get().ue_rebo_tx.send(event).unwrap();
    }

    loop {
        let msg = STATE.get().rebo_ue_rx.recv().unwrap();
        match msg {
            ReboToUe::Stop => {
                log!("Got ReboToUe::Stop, but state is Stopping");
                panic!()
            }
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
            ReboToUe::MoveMouse(x, y) => {
                // we don't want to trigger our mouseevent handler for emulated mouse movements
                unhook_fslateapplication_onrawmousemove();
                FSlateApplication::move_mouse(x, y);
                hook_fslateapplication_onrawmousemove();
            },
            ReboToUe::DrawLine(startx, starty, endx, endy, color, thickness) =>
                AMyHud::draw_line(startx, starty, endx, endy, color, thickness),
            ReboToUe::DrawText(text, color, x, y, scale, scale_position) =>
                AMyHud::draw_text(text, color, x, y, scale, scale_position),
            ReboToUe::SpawnAMyCharacter(x, y, z, pitch, yaw, roll) => {
                let my_character = UWorld::spawn_amycharacter(x, y, z, pitch, yaw, roll);
                STATE.get().ue_rebo_tx.send(UeToRebo::AMyCharacterSpawned(my_character)).unwrap();
            },
            ReboToUe::PumpMessages => {
                FPlatformMisc::pump_messages();
                STATE.get().ue_rebo_tx.send(UeToRebo::PumpedMessages).unwrap();
            },
            ReboToUe::Resume => {
                log!("Resuming");
                STATE.get().typ = StateType::Running;
                break;
            },
            ReboToUe::AdvanceFrame => break,
            ReboToUe::TriggerNewGame => {
                UMyGameInstance::restart_game();
            }
        }
    }
}
