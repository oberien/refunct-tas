use crossbeam_channel::{Receiver, Sender};
use crate::native::{ALiftBaseUE, ElementIndex, EMouseButtonsType, Hooks, try_find_element_index, UObject};
use crate::threads::{ReboToStream, StreamToRebo};
use crate::threads::ue::iced_ui::Key;

mod rebo;
mod iced_ui;

#[derive(Debug, Clone)]
enum UeEvent {
    Tick,
    ElementPressed(ElementIndex),
    ElementReleased(ElementIndex),
    /// Response to `Yield` if no new event happened
    NothingHappened,
    NewGame,
    KeyDown(Key, bool),
    KeyUp(Key, bool),
    MouseMove(i32, i32),
    MouseButtonDown(EMouseButtonsType),
    MouseButtonUp(EMouseButtonsType),
    MouseWheel(f32),
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

pub fn run(stream_rebo_rx: Receiver<StreamToRebo>, rebo_stream_tx: Sender<ReboToStream>, hooks: Hooks) {
    rebo::init(stream_rebo_rx, rebo_stream_tx, hooks);
    log!("\"starting\" ue thread");
}

pub fn new_game() {
    log!("New Game");
    handle(UeEvent::NewGame)
}

pub fn tick() {
    handle(UeEvent::Tick);
}

pub fn add_based_character(ptr: *mut ALiftBaseUE) {
    // TODO: remove once we added pipes to the map editor
    let element_index = match try_find_element_index(ptr as *mut UObject) {
        Some(i) => i,
        None => return,
    };
    handle(UeEvent::ElementPressed(element_index));
}
pub fn remove_based_character(ptr: *mut ALiftBaseUE) {
    // TODO: remove once we added pipes to the map editor
    let element_index = match try_find_element_index(ptr as *mut UObject) {
        Some(i) => i,
        None => return,
    };
    handle(UeEvent::ElementReleased(element_index));
}

fn codes_to_key(key_code: i32, character_code: u32) -> Key {
    #[cfg(unix)] {
        Key::try_from_linux(key_code, character_code)
    }
    #[cfg(windows)] {
        Key::try_from_windows(key_code, character_code)
    }
}

pub fn key_down(key_code: i32, character_code: u32, is_repeat: bool) {
    handle(UeEvent::KeyDown(codes_to_key(key_code, character_code), is_repeat));
}
pub fn key_up(key_code: i32, character_code: u32, is_repeat: bool) {
    handle(UeEvent::KeyUp(codes_to_key(key_code, character_code), is_repeat));
}

pub fn mouse_move(x: i32, y: i32) {
    handle(UeEvent::MouseMove(x, y));
}
pub fn mouse_button_down(button: EMouseButtonsType) {
    handle(UeEvent::MouseButtonDown(button));
}
pub fn mouse_button_up(button: EMouseButtonsType) {
    handle(UeEvent::MouseButtonUp(button));
}
pub fn mouse_wheel(delta: f32) {
    handle(UeEvent::MouseWheel(delta));
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
