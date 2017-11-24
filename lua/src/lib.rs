extern crate hlua;

pub mod stub;

use std::rc::Rc;
use std::cell::RefCell;

use hlua::{Lua as HLua, LuaFunction, AnyLuaValue, Void, AsMutLua, PushGuard, Push, LuaError};

pub struct Lua<'lua> {
    lua: HLua<'lua>,
}

pub trait IntoAnyLuaValue {
    fn into_any_lua_value(self) -> AnyLuaValue;
}

pub enum Event {
    Stopped,
    NewGame,
}

impl IntoAnyLuaValue for Event {
    fn into_any_lua_value(self) -> AnyLuaValue {
        match self {
            Event::Stopped => AnyLuaValue::LuaString("stopped".to_string()),
            Event::NewGame => AnyLuaValue::LuaString("newgame".to_string()),
        }
    }
}

pub enum Response<T> {
    ExitPlease,
    Result(T),
}

impl<T: IntoAnyLuaValue> IntoAnyLuaValue for Response<T> {
    fn into_any_lua_value(self) -> AnyLuaValue {
        let variant = AnyLuaValue::LuaString("variant".to_string());
        let data = AnyLuaValue::LuaString("data".to_string());
        match self {
            Response::ExitPlease => {
                AnyLuaValue::LuaArray(vec![
                    (variant, AnyLuaValue::LuaString("exit".to_string())),
                    (data, AnyLuaValue::LuaNil),
                ])
            }
            Response::Result(res) => {
                AnyLuaValue::LuaArray(vec![
                    (variant, AnyLuaValue::LuaString("result".to_string())),
                    (data, res.into_any_lua_value()),
                ])
            }
        }
    }
}

impl IntoAnyLuaValue for () {
    fn into_any_lua_value(self) -> AnyLuaValue {
        AnyLuaValue::LuaNil
    }
}

impl IntoAnyLuaValue for f32 {
    fn into_any_lua_value(self) -> AnyLuaValue {
        AnyLuaValue::LuaNumber(self as f64)
    }
}

impl IntoAnyLuaValue for f64 {
    fn into_any_lua_value(self) -> AnyLuaValue {
        AnyLuaValue::LuaNumber(self)
    }
}

impl<A, B, C> IntoAnyLuaValue for (A, B, C)
        where A: IntoAnyLuaValue, B: IntoAnyLuaValue, C: IntoAnyLuaValue {
    fn into_any_lua_value(self) -> AnyLuaValue {
        let (a, b, c) = self;
        AnyLuaValue::LuaArray(vec![
            (AnyLuaValue::LuaNumber(1.0), a.into_any_lua_value()),
            (AnyLuaValue::LuaNumber(2.0), b.into_any_lua_value()),
            (AnyLuaValue::LuaNumber(3.0), c.into_any_lua_value()),
        ])
    }
}

impl<'lua, T, L> Push<L> for Response<T> where L: AsMutLua<'lua>, T: IntoAnyLuaValue {
    type Err = Void;

    fn push_to_lua(self, lua: L) -> Result<PushGuard<L>, (Self::Err, L)> {
        self.into_any_lua_value().push_to_lua(lua)
    }
}

pub trait LuaInterface {
    fn step(&mut self) -> Response<Event>;
    fn press_key(&mut self, key: String) -> Response<()>;
    fn release_key(&mut self, key: String) -> Response<()>;
    fn move_mouse(&mut self, x: i32, y: i32) -> Response<()>;
    fn get_delta(&mut self) -> Response<f64>;
    fn set_delta(&mut self, delta: f64) -> Response<()>;
    fn get_location(&mut self) -> Response<(f32, f32, f32)>;
    fn set_location(&mut self, x: f32, y: f32, z: f32) -> Response<()>;
    fn get_rotation(&mut self) -> Response<(f32, f32, f32)>;
    fn set_rotation(&mut self, pitch: f32, yaw: f32, roll: f32) -> Response<()>;
    fn get_velocity(&mut self) -> Response<(f32, f32, f32)>;
    fn set_velocity(&mut self, x: f32, y: f32, z: f32) -> Response<()>;
    fn get_acceleration(&mut self) -> Response<(f32, f32, f32)>;
    fn set_acceleration(&mut self, x: f32, y: f32, z: f32) -> Response<()>;
    fn wait_for_new_game(&mut self) -> Response<()>;

    fn print(&mut self, s: String) -> Response<()>;
}

impl<'lua> Lua<'lua> {
    pub fn new<T: 'lua + LuaInterface>(outer: Rc<RefCell<T>>) -> Lua<'lua> {
        let mut lua = HLua::new();
        lua.openlibs();

        let tas = outer.clone();
        lua.set("__step", hlua::function0(move || {
            tas.borrow_mut().step()
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

        let tas = outer.clone();
        lua.set("__print", hlua::function1(move |s: String| {
            tas.borrow_mut().print(s)
        }));

        Lua {
            lua,
        }
    }

    pub fn execute(&mut self, code: &str) -> Result<(), LuaError> {
        let mut function = LuaFunction::load(&mut self.lua, code)?;
        function.call::<()>()?;
        Ok(())
    }
}
