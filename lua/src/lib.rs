extern crate rlua;

pub mod stub;

use std::rc::Rc;

use rlua::{Value, ToLua, UserData, UserDataMethods, Error as LuaError, Function};
pub use rlua::{Result as LuaResult, Lua as RLua};

#[derive(Debug)]
pub enum IfaceError {
    ExitPlease,
}

impl std::fmt::Display for IfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error;
        writeln!(f, "{}", self.description())
    }
}

impl std::error::Error for IfaceError {
    fn description(&self) -> &str {
        match *self {
            IfaceError::ExitPlease => "Lua should Exit",
        }
    }
}

impl From<IfaceError> for LuaError {
    fn from(err: IfaceError) -> Self {
        LuaError::external(err)
    }
}

pub type IfaceResult<T> = Result<T, IfaceError>;

pub struct Lua<T: LuaInterface> {
    lua: RLua,
    iface: Rc<T>,
}

pub enum Event {
    Stopped,
    NewGame,
}

impl<'lua> ToLua<'lua> for Event {
    fn to_lua(self, lua: &'lua RLua) -> LuaResult<Value<'lua>> {
        match self {
            Event::Stopped => "stopped".to_lua(lua),
            Event::NewGame => "newgame".to_lua(lua),
        }
    }
}

pub trait LuaInterface {
    fn step(&self, lua: &RLua) -> LuaResult<Event>;
    fn press_key(&self, key: String) -> IfaceResult<()>;
    fn release_key(&self, key: String) -> IfaceResult<()>;
    fn key_down(&self, key_code: i32, character_code: u32, is_repeat: bool) -> IfaceResult<()>;
    fn key_up(&self, key_code: i32, character_code: u32, is_repeat: bool) -> IfaceResult<()>;
    fn move_mouse(&self, x: i32, y: i32) -> IfaceResult<()>;
    fn get_delta(&self) -> IfaceResult<f64>;
    fn set_delta(&self, delta: f64) -> IfaceResult<()>;
    fn get_location(&self) -> IfaceResult<(f32, f32, f32)>;
    fn set_location(&self, x: f32, y: f32, z: f32) -> IfaceResult<()>;
    fn get_rotation(&self) -> IfaceResult<(f32, f32, f32)>;
    fn set_rotation(&self, pitch: f32, yaw: f32, roll: f32) -> IfaceResult<()>;
    fn get_velocity(&self) -> IfaceResult<(f32, f32, f32)>;
    fn set_velocity(&self, x: f32, y: f32, z: f32) -> IfaceResult<()>;
    fn get_acceleration(&self) -> IfaceResult<(f32, f32, f32)>;
    fn set_acceleration(&self, x: f32, y: f32, z: f32) -> IfaceResult<()>;
    fn wait_for_new_game(&self, lua: &RLua) -> LuaResult<()>;

    fn draw_line(&self, startx: f32, starty: f32, endx: f32, endy: f32, color: (f32, f32, f32, f32), thickness: f32) -> IfaceResult<()>;
    fn draw_text(&self, text: String, color: (f32, f32, f32, f32), x: f32, y: f32, scale: f32, scale_position: bool) -> IfaceResult<()>;

    fn print(&self, s: String) -> IfaceResult<()>;

    fn working_dir(&self) -> IfaceResult<String>;

    // only Windows and Linux are supported
    fn is_windows(&self) -> IfaceResult<bool> {
        Ok(cfg!(windows))
    }
    fn is_linux(&self) -> IfaceResult<bool> {
        Ok(!self.is_windows()?)
    }
}

struct Wrapper<T>(T);

impl<T> std::ops::Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: 'static + LuaInterface> UserData for Wrapper<Rc<T>> {
    fn add_methods(methods: &mut UserDataMethods<Self>) {
        methods.add_method("step", |lua, this, _: ()| {
            this.step(lua)
        });
        methods.add_method("press_key", |_, this, key: String| {
            Ok(this.press_key(key)?)
        });
        methods.add_method("release_key", |_, this, key: String| {
            Ok(this.release_key(key)?)
        });
        methods.add_method("key_down", |_, this, (key, chr, rep): (i32, u32, bool)| {
            Ok(this.key_down(key, chr, rep)?)
        });
        methods.add_method("key_up", |_, this, (key, chr, rep): (i32, u32, bool)| {
            Ok(this.key_up(key, chr, rep)?)
        });
        methods.add_method("move_mouse", |_, this, (x, y): (i32, i32)| {
            Ok(this.move_mouse(x, y)?)
        });
        methods.add_method("get_delta", |_, this, _: ()| {
            Ok(this.get_delta()?)
        });
        methods.add_method("set_delta", |_, this, delta: f64| {
            Ok(this.set_delta(delta)?)
        });
        methods.add_method("get_location", |_, this, _: ()| {
            Ok(this.get_location()?)
        });
        methods.add_method("set_location", |_, this, (x, y, z): (f32, f32, f32)| {
            Ok(this.set_location(x, y, z)?)
        });
        methods.add_method("get_rotation", |_, this, _: ()| {
            Ok(this.get_rotation()?)
        });
        methods.add_method("set_rotation", |_, this, (pitch, yaw, roll): (f32, f32, f32)| {
            Ok(this.set_rotation(pitch, yaw, roll)?)
        });
        methods.add_method("get_velocity", |_, this, _: ()| {
            Ok(this.get_velocity()?)
        });
        methods.add_method("set_velocity", |_, this, (x, y, z): (f32, f32, f32)| {
            Ok(this.set_velocity(x, y, z)?)
        });
        methods.add_method("get_acceleration", |_, this, _: ()| {
            Ok(this.get_acceleration()?)
        });
        methods.add_method("set_acceleration", |_, this, (x, y, z): (f32, f32, f32)| {
            Ok(this.set_acceleration(x, y, z)?)
        });
        methods.add_method("wait_for_new_game", |lua, this, _: ()| {
            this.wait_for_new_game(lua)
        });

        methods.add_method("draw_line", |_, this, (startx, starty, endx, endy, red, green, blue, alpha, thickness): (f32, f32, f32, f32, f32, f32, f32, f32, f32)| {
            Ok(this.draw_line(startx, starty, endx, endy, (red, green, blue, alpha), thickness)?)
        });
        methods.add_method("draw_text", |_, this, (text, red, green, blue, alpha, x, y, scale, scale_position): (String, f32, f32, f32, f32, f32, f32, f32, bool)| {
            Ok(this.draw_text(text, (red, green, blue, alpha), x, y, scale, scale_position)?)
        });

        methods.add_method("print", |_, this, s: String| {
            Ok(this.print(s)?)
        });

        methods.add_method("working_dir", |_, this, _: ()| {
            Ok(this.working_dir()?)
        });

        methods.add_method("is_windows", |_, this, _: ()| {
            Ok(this.is_windows()?)
        });
        methods.add_method("is_linux", |_, this, _: ()| {
            Ok(this.is_linux()?)
        });
    }
}

pub trait LuaEvents {
    fn on_key_down(&self, key_code: i32, character_code: u32, is_repeat: bool) -> LuaResult<()>;
    fn on_key_up(&self, key_code: i32, character_code: u32, is_repeat: bool) -> LuaResult<()>;
    fn draw_hud(&self) -> LuaResult<()>;
}

impl<T: LuaInterface + 'static> Lua<T> {
    pub fn new(iface: Rc<T>) -> Lua<T> {
        let lua = RLua::new();
        Lua {
            lua,
            iface,
        }
    }

    pub fn execute(&mut self, code: &str) -> LuaResult<()> {
        self.lua.scope(|scope| {
            let iface = scope.create_userdata(Wrapper(self.iface.clone()))?;
            self.lua.globals().set("tas", iface)?;
            let function = self.lua.load(code, None)?;
            function.call::<_, ()>(())
        })
    }
}

impl LuaEvents for RLua {
    fn on_key_down(&self, key_code: i32, character_code: u32, is_repeat: bool) -> LuaResult<()> {
        let fun: LuaResult<Function> = self.globals().get("onkeydown");
        if let Ok(fun) = fun {
            let _: () = fun.call((key_code, character_code, is_repeat))?;
        }
        Ok(())
    }

    fn on_key_up(&self, key_code: i32, character_code: u32, is_repeat: bool) -> LuaResult<()> {
        let fun: LuaResult<Function> = self.globals().get("onkeyup");
        if let Ok(fun) = fun {
            let _: () = fun.call((key_code, character_code, is_repeat))?;
        }
        Ok(())
    }

    fn draw_hud(&self) -> LuaResult<()> {
        let fun: LuaResult<Function> = self.globals().get("drawhud");
        if let Ok(fun) = fun {
            let _: () = fun.call(())?;
        }
        Ok(())
    }
}
