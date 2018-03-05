use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use std::thread;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashSet;

use lua::{Lua, LuaInterface, LuaEvents, Event, IfaceResult, IfaceError, RLua, LuaResult};
use failure::Fail;

use threads::{StreamToLua, LuaToStream, LuaToUe, UeToLua, Config};
use native::{AMyCharacter, AController, FApp};

struct Tas {
    iface: Rc<GameInterface>,
    lua: Lua<GameInterface>,
    working_dir: Option<String>,
}

pub fn run(stream_lua_rx: Receiver<StreamToLua>, lua_stream_tx: Sender<LuaToStream>,
           lua_ue_tx: Sender<LuaToUe>, ue_lua_rx: Receiver<UeToLua>) {
    thread::spawn(move|| {
        let iface = Rc::new(GameInterface {
            pressed_keys: RefCell::new(HashSet::new()),
            stream_lua_rx,
            lua_stream_tx,
            lua_ue_tx,
            ue_lua_rx,
            config: RefCell::new(Config::default()),
            should_exit: RefCell::new(false),
        });
        let mut tas = Tas {
            iface: iface.clone(),
            lua: Lua::new(iface),
            working_dir: None,
        };

        loop {
            tas.handle_rx();
        }
    });
}

impl Tas {
    fn handle_rx(&mut self) {
        let res = self.iface.stream_lua_rx.recv().unwrap();
        match res {
            StreamToLua::Stop => {},
            StreamToLua::Config(config) => {
                log!("Set config before running");
                *self.iface.config.borrow_mut() = config;
            },
            StreamToLua::WorkingDir(dir) => {
                log!("Set working dir");
                self.working_dir = Some(dir);
            }
            StreamToLua::Start(s) => {
                log!("Starting lua...");
                log!("Cleaning ue_lua_rx...");
                let mut count = 0;
                while let Ok(_) = self.iface.ue_lua_rx.try_recv() {
                    count += 1;
                }
                log!("Removed {} messages", count);
                self.lua = Lua::new(self.iface.clone());
                if let Some(dir) = self.working_dir.as_ref() {
                    log!("Add {} to package.path.", dir);
                    let dir = format!(r#"package.path = package.path .. ";{}/?.lua""#, dir.replace('\\', "\\\\"));
                    self.lua.execute(&dir).unwrap();
                    log!("Added");
                }
                self.iface.lua_ue_tx.send(LuaToUe::Stop).unwrap();
                log!("Executing Lua code.");
                if let Err(e) = self.lua.execute(&s) {
                    let mut err = format!("Lua error'd: {}\n", e);
                    let mut e: &Fail = &e;
                    while let Some(cause) = e.cause() {
                        err += &format!("caused by: {}\n", cause);
                        e = cause;
                    }
                    log!("{}", err);
                    self.iface.lua_stream_tx.send(LuaToStream::Print(err)).unwrap();
                }
                log!("Lua execution done. Starting cleanup...");
                self.iface.reset();
                self.iface.lua_ue_tx.send(LuaToUe::Resume).unwrap();
                self.iface.lua_stream_tx.send(LuaToStream::MiDone).unwrap();
                log!("Cleanup finished.");
            }
        }
    }
}

pub struct GameInterface {
    pressed_keys: RefCell<HashSet<i32>>,
    stream_lua_rx: Receiver<StreamToLua>,
    lua_stream_tx: Sender<LuaToStream>,
    lua_ue_tx: Sender<LuaToUe>,
    ue_lua_rx: Receiver<UeToLua>,
    config: RefCell<Config>,
    should_exit: RefCell<bool>,
}

impl GameInterface {
    /// Check internal state and channel to see if we should stop.
    /// Returns an Error if Lua should exit.
    fn syscall(&self) -> IfaceResult<()> {
        if *self.should_exit.borrow() {
            return Err(IfaceError::ExitPlease);
        }
        match self.stream_lua_rx.try_recv() {
            Ok(res) => match res {
                StreamToLua::Config(cfg) => {
                    log!("Set Config while running");
                    *self.config.borrow_mut() = cfg;
                }
                StreamToLua::WorkingDir(_) => {
                    log!("Got WorkingDir, but can't set it during execution");
                    panic!()
                }
                StreamToLua::Start(_) => {
                    log!("Got StreamToLua::Start but lua is already running");
                    panic!()
                }
                StreamToLua::Stop => {
                    log!("Should Exit");
                    *self.should_exit.borrow_mut() = true;
                    return Err(IfaceError::ExitPlease);
                }
            }
            Err(TryRecvError::Empty) => {},
            Err(e) => {
                log!("Error stream_lua_rx.try_recv: {:?}", e);
                panic!();
            }
        }
        Ok(())
    }

    fn reset(&self) {
        let mut pressed_keys = self.pressed_keys.borrow_mut();
        for key in pressed_keys.drain() {
            self.lua_ue_tx.send(LuaToUe::ReleaseKey(key)).unwrap();
        }
        *self.should_exit.borrow_mut() = false;
    }

    fn to_key(&self, key: &str) -> i32 {
        match key {
            "forward" => self.config.borrow().forward,
            "backward" => self.config.borrow().backward,
            "left" => self.config.borrow().left,
            "right" => self.config.borrow().right,
            "jump" => self.config.borrow().jump,
            "crouch" => self.config.borrow().crouch,
            "menu" => self.config.borrow().menu,
            _ => {
                log!("Invalid Key: {}", key);
                panic!()
            }
        }
    }
}

impl LuaInterface for GameInterface {
    fn step(&self, lua: &RLua) -> LuaResult<Event> {
        self.lua_ue_tx.send(LuaToUe::AdvanceFrame).unwrap();
        loop {
            self.syscall()?;
            match self.ue_lua_rx.recv().unwrap() {
                UeToLua::Tick => return Ok(Event::Stopped),
                UeToLua::NewGame => return Ok(Event::NewGame),
                UeToLua::KeyDown(key, char, repeat) => lua.on_key_down(key, char, repeat)?,
                UeToLua::KeyUp(key, char, repeat) => lua.on_key_up(key, char, repeat)?,
                UeToLua::DrawHud => lua.draw_hud()?,
            }
            // We aren't actually advancing a frame, but just returning from the
            // key event interceptor.
            self.lua_ue_tx.send(LuaToUe::AdvanceFrame).unwrap();
        }
    }

    fn press_key(&self, key: String) -> IfaceResult<()> {
        self.syscall()?;
        let key = self.to_key(&key);
        self.pressed_keys.borrow_mut().insert(key);
        self.lua_ue_tx.send(LuaToUe::PressKey(key)).unwrap();
        Ok(())
    }

    fn release_key(&self, key: String) -> IfaceResult<()> {
        self.syscall()?;
        let key = self.to_key(&key);
        self.pressed_keys.borrow_mut().remove(&key);
        self.lua_ue_tx.send(LuaToUe::ReleaseKey(key)).unwrap();
        Ok(())
    }

    fn move_mouse(&self, x: i32, y: i32) -> IfaceResult<()> {
        self.syscall()?;
        self.lua_ue_tx.send(LuaToUe::MoveMouse(x, y)).unwrap();
        Ok(())
    }

    fn get_delta(&self) -> IfaceResult<f64> {
        self.syscall()?;
        Ok(FApp::delta())
    }

    fn set_delta(&self, delta: f64) -> IfaceResult<()> {
        self.syscall()?;
        FApp::set_delta(delta);
        Ok(())
    }

    fn get_location(&self) -> IfaceResult<(f32, f32, f32)> {
        self.syscall()?;
        Ok(AMyCharacter::location())
    }

    fn set_location(&self, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        self.syscall()?;
        AMyCharacter::set_location(x, y, z);
        Ok(())
    }

    fn get_rotation(&self) -> IfaceResult<(f32, f32, f32)> {
        self.syscall()?;
        Ok(AController::rotation())
    }

    fn set_rotation(&self, pitch: f32, yaw: f32, roll: f32) -> IfaceResult<()> {
        self.syscall()?;
        AController::set_rotation(pitch, yaw, roll);
        Ok(())
    }

    fn get_velocity(&self) -> IfaceResult<(f32, f32, f32)> {
        self.syscall()?;
        Ok(AMyCharacter::velocity())
    }

    fn set_velocity(&self, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        self.syscall()?;
        AMyCharacter::set_velocity(x, y, z);
        Ok(())
    }

    fn get_acceleration(&self) -> IfaceResult<(f32, f32, f32)> {
        self.syscall()?;
        Ok(AMyCharacter::acceleration())
    }

    fn set_acceleration(&self, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        self.syscall()?;
        AMyCharacter::set_acceleration(x, y, z);
        Ok(())
    }

    fn wait_for_new_game(&self, lua: &RLua) -> LuaResult<()> {
        loop {
            match self.step(lua)? {
                Event::Stopped => continue,
                Event::NewGame => return Ok(()),
            }
        }
    }

    fn draw_line(&self, startx: f32, starty: f32, endx: f32, endy: f32, color: (f32, f32, f32, f32), thickness: f32) -> IfaceResult<()> {
        self.syscall()?;
        self.lua_ue_tx.send(LuaToUe::DrawLine(startx, starty, endx, endy, color, thickness)).unwrap();
        Ok(())
    }

    fn draw_text(&self, text: String, color: (f32, f32, f32, f32), x: f32, y: f32, scale: f32, scale_position: bool) -> IfaceResult<()> {
        self.syscall()?;
        self.lua_ue_tx.send(LuaToUe::DrawText(text, color, x, y, scale, scale_position)).unwrap();
        Ok(())
    }

    fn print(&self, s: String) -> IfaceResult<()> {
        self.syscall()?;
        self.lua_stream_tx.send(LuaToStream::Print(s)).unwrap();
        Ok(())
    }
}
