extern crate hlua;

use std::rc::Rc;
use std::cell::RefCell;

use hlua::{Lua as HLua, LuaFunction};

pub struct Lua<'lua> {
    lua: HLua<'lua>,
}

pub enum Response {
    Stopped,
    NewGame,
}

pub trait Tas {
    fn stop(&mut self);
    fn step(&mut self) -> Response;
    fn press_key(&mut self, key: String);
    fn release_key(&mut self, key: String);
    fn move_mouse(&mut self, x: i32, y: i32);
    fn get_delta(&mut self) -> f64;
    fn set_delta(&mut self, delta: f64);
    fn get_location(&mut self) -> (f32, f32, f32);
    fn set_location(&mut self, x: f32, y: f32, z: f32);
    fn get_rotation(&mut self) -> (f32, f32, f32);
    fn set_rotation(&mut self, x: f32, y: f32, z: f32);
    fn get_velocity(&mut self) -> (f32, f32, f32);
    fn set_velocity(&mut self, x: f32, y: f32, z: f32);
    fn get_acceleration(&mut self) -> (f32, f32, f32);
    fn set_acceleration(&mut self, x: f32, y: f32, z: f32);
    fn wait_for_new_game(&mut self);
}

impl<'lua> Lua<'lua> {
    pub fn new<T: Tas>(outer: Rc<RefCell<Tas>>) -> Lua<'lua> {
        let mut lua = HLua::new();
        lua.openlibs();

        let tas = outer.clone();
        lua.set("__stop", hlua::function0(move || {
            tas.borrow_mut().stop()
        }));

        let tas = outer.clone();
        lua.set("__step", hlua::function0(move || {
            match tas.borrow_mut().step() {
                Response::Stopped => "stopped",
                Response::NewGame => "newgame",
            }
        }));

        let tas = outer.clone();
        lua.set("__press_key", hlua::function1(move |key: String| {
            tas.borrow_mut().press_key(key)
        }));

        let tas = outer.clone();
        lua.set("__release_key", hlua::function1(move |key: String| {
            tas.borrow_mut().release_key(key)
        }));

        let tas = outer.clone();
        lua.set("__move_mouse", hlua::function2(move |x: i32, y: i32| {
            tas.borrow_mut().move_mouse(x, y)
        }));

        let tas = outer.clone();
        lua.set("__get_delta", hlua::function0(move || {
            tas.borrow_mut().get_delta()
        }));
        let tas = outer.clone();
        lua.set("__set_delta", hlua::function1(move |delta: f64| {
            tas.borrow_mut().set_delta(delta)
        }));

        let tas = outer.clone();
        lua.set("__get_location", hlua::function0(move || {
            tas.borrow_mut().get_location()
        }));
        let tas = outer.clone();
        lua.set("__set_location", hlua::function3(move |x: f32, y: f32, z: f32| {
            tas.borrow_mut().set_location(x, y, z)
        }));

        let tas = outer.clone();
        lua.set("__get_rotation", hlua::function0(move || {
            tas.borrow_mut().get_rotation()
        }));
        let tas = outer.clone();
        lua.set("__set_rotation", hlua::function3(move |pitch: f32, yaw: f32, roll: f32| {
            tas.borrow_mut().set_rotation(pitch, yaw, roll)
        }));

        let tas = outer.clone();
        lua.set("__get_velocity", hlua::function0(move || {
            tas.borrow_mut().get_velocity()
        }));
        let tas = outer.clone();
        lua.set("__set_velocity", hlua::function3(move |x: f32, y: f32, z: f32| {
            tas.borrow_mut().set_velocity(x, y, z)
        }));

        let tas = outer.clone();
        lua.set("__get_acceleration", hlua::function0(move || {
            tas.borrow_mut().get_acceleration()
        }));
        let tas = outer.clone();
        lua.set("__set_acceleration", hlua::function3(move |x: f32, y: f32, z: f32| {
            tas.borrow_mut().set_acceleration(x, y, z)
        }));

        let tas = outer.clone();
        lua.set("__wait_for_new_game", hlua::function0(move || {
            tas.borrow_mut().wait_for_new_game()
        }));

        Lua {
            lua,
        }
    }

    pub fn execute(&mut self, code: &str) {
        let mut function = LuaFunction::load(&mut self.lua, code).unwrap();
        function.call::<()>().unwrap();
    }
}

//fn to_key(key: &str, cfg: &Config) -> i32 {
//    match key {
//        "forward" => cfg.forward.into(),
//        "backward" => cfg.backward.into(),
//        "left" => cfg.left.into(),
//        "right" => cfg.right.into(),
//        "jump" => cfg.jump.into(),
//        "crouch" => cfg.crouch.into(),
//        "menu" => cfg.menu.into(),
//        s => panic!("Unknown key {}", s)
//    }
//}

