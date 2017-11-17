use std::sync::mpsc::{Sender, Receiver};

use lua::{LuaInterface, Response};

use native::{AMyCharacter, AController, FSlateApplication};

pub struct GameInterface {
    tx: Sender<()>,
    rx: Receiver<Response>,
}

impl LuaInterface for GameInterface {
    fn step(&mut self) -> Response {
        self.tx.send(());
        self.rx.recv()
    }

    fn press_key(&mut self, key: String) {
        FSlateApplication::onk
    }

    fn release_key(&mut self, key: String) {
        unimplemented!()
    }

    fn move_mouse(&mut self, x: i32, y: i32) {
        unimplemented!()
    }

    fn get_delta(&mut self) -> f64 {
        unimplemented!()
    }

    fn set_delta(&mut self, delta: f64) {
        unimplemented!()
    }

    fn get_location(&mut self) -> (f32, f32, f32) {
        unimplemented!()
    }

    fn set_location(&mut self, x: f32, y: f32, z: f32) {
        unimplemented!()
    }

    fn get_rotation(&mut self) -> (f32, f32, f32) {
        unimplemented!()
    }

    fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        unimplemented!()
    }

    fn get_velocity(&mut self) -> (f32, f32, f32) {
        unimplemented!()
    }

    fn set_velocity(&mut self, x: f32, y: f32, z: f32) {
        unimplemented!()
    }

    fn get_acceleration(&mut self) -> (f32, f32, f32) {
        unimplemented!()
    }

    fn set_acceleration(&mut self, x: f32, y: f32, z: f32) {
        unimplemented!()
    }

    fn wait_for_new_game(&mut self) {
        unimplemented!()
    }

    fn print(&mut self, s: String) {
        unimplemented!()
    }
}