use crossbeam_channel::{Receiver, Sender};
use crate::native::{ElementIndex, try_find_element_index, UObject};
use crate::threads::{ReboToStream, StreamToRebo};

pub(crate) mod rebo;

#[derive(Debug, Clone)]
enum UeEvent {
    Tick,
    ElementPressed(ElementIndex),
    ElementReleased(ElementIndex),
    /// Response to `Yield` if no new event happened
    NothingHappened,
    NewGame,
    KeyDown(i32, u32, bool),
    KeyUp(i32, u32, bool),
    MouseMove(i32, i32),
    DrawHud,
    ApplyResolutionSettings,
    AddToScreen,
}
#[derive(Debug, Clone, Copy)]
enum Suspend {
    /// yield only without returning to the UE loop for the event-queue to be handled
    Yield,
    /// return back to the UE loop
    Return,
}

pub fn run(stream_rebo_rx: Receiver<StreamToRebo>, rebo_stream_tx: Sender<ReboToStream>) {
    rebo::init(stream_rebo_rx, rebo_stream_tx);
    log!("\"starting\" ue thread");
}

pub fn new_game() {
    log!("New Game");
    handle(UeEvent::NewGame)
}

pub fn tick() {
    handle(UeEvent::Tick);
}

pub fn add_based_character(ptr: *mut UObject) {
    // TODO: remove once we added pipes to the map editor
    let element_index = match try_find_element_index(ptr) {
        Some(i) => i,
        None => return,
    };
    handle(UeEvent::ElementPressed(element_index));
}
pub fn remove_based_character(ptr: *mut UObject) {
    // TODO: remove once we added pipes to the map editor
    let element_index = match try_find_element_index(ptr) {
        Some(i) => i,
        None => return,
    };
    handle(UeEvent::ElementReleased(element_index));
}

pub fn key_down(key_code: i32, character_code: u32, is_repeat: bool) {
    handle(UeEvent::KeyDown(key_code, character_code, is_repeat));
}

pub fn key_up(key_code: i32, character_code: u32, is_repeat: bool) {
    handle(UeEvent::KeyUp(key_code, character_code, is_repeat));
}

pub fn mouse_move(x: i32, y: i32) {
    handle(UeEvent::MouseMove(x, y));
}

pub fn draw_hud() {
    handle(UeEvent::DrawHud);
}

pub fn apply_resolution_settings() {
    handle(UeEvent::ApplyResolutionSettings);
}

pub fn add_to_screen() {
    handle(UeEvent::AddToScreen);
}

fn handle(event: UeEvent) {
    rebo::poll(event);
}
