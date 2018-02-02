use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use std::thread;
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashSet;

use lua::{Lua, LuaInterface, Response, Event};

use threads::{StreamToLua, LuaToStream, LuaToUe, UeToLua, Config};
use native::{AMyCharacter, AController, FApp};

struct Tas<'lua> {
    iface: Rc<RefCell<GameInterface>>,
    lua: Lua<'lua>,
    working_dir: Option<String>,
}

pub fn run(stream_lua_rx: Receiver<StreamToLua>, lua_stream_tx: Sender<LuaToStream>,
           lua_ue_tx: Sender<LuaToUe>, ue_lua_rx: Receiver<UeToLua>) {
    thread::spawn(move|| {
        let iface = Rc::new(RefCell::new(GameInterface {
            pressed_keys: HashSet::new(),
            stream_lua_rx,
            lua_stream_tx,
            lua_ue_tx,
            ue_lua_rx,
            config: Config::default(),
            should_exit: false,
        }));
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

impl<'lua> Tas<'lua> {
    fn handle_rx(&mut self) {
        let res = self.iface.borrow().stream_lua_rx.recv().unwrap();
        match res {
            StreamToLua::Stop => {},
            StreamToLua::Config(config) => {
                log!("Set config before running");
                self.iface.borrow_mut().config = config;
            },
            StreamToLua::WorkingDir(dir) => {
                log!("Set working dir");
                self.working_dir = Some(dir);
            }
            StreamToLua::Start(s) => {
                log!("Starting lua...");
                self.lua = Lua::new(self.iface.clone());
                if let Some(dir) = self.working_dir.as_ref() {
                    log!("Add {} to package.path.", dir);
                    let dir = format!(r#"package.path = package.path .. ";{}/?.lua""#, dir.replace('\\', "\\\\"));
                    self.lua.execute(&dir).unwrap();
                    log!("Added");
                }
                self.iface.borrow().lua_ue_tx.send(LuaToUe::Stop).unwrap();
                log!("Executing Lua code.");
                if let Err(e) = self.lua.execute(&s) {
                    log!("Lua error'd: {}", e);
                    self.iface.borrow().lua_stream_tx.send(LuaToStream::Print(format!("{}", e))).unwrap();
                }
                log!("Lua execution done. Starting cleanup...");
                self.iface.borrow_mut().reset();
                self.iface.borrow().lua_ue_tx.send(LuaToUe::Resume).unwrap();
                self.iface.borrow().lua_stream_tx.send(LuaToStream::MiDone).unwrap();
                log!("Cleanup finished.");
            }
        }
    }
}

pub struct GameInterface {
    pressed_keys: HashSet<i32>,
    stream_lua_rx: Receiver<StreamToLua>,
    lua_stream_tx: Sender<LuaToStream>,
    lua_ue_tx: Sender<LuaToUe>,
    ue_lua_rx: Receiver<UeToLua>,
    config: Config,
    should_exit: bool,
}

impl GameInterface {
    /// Check internal state and channel to see if we should stop.
    /// Returns true if lua should exit itself.
    fn syscall(&mut self) -> bool {
        if self.should_exit {
            return true;
        }
        match self.stream_lua_rx.try_recv() {
            Ok(res) => match res {
                StreamToLua::Config(cfg) => {
                    log!("Set Config while running");
                    self.config = cfg;
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
                    self.should_exit = true;
                    return true;
                }
            }
            Err(TryRecvError::Empty) => {},
            Err(e) => {
                log!("Error stream_lua_rx.try_recv: {:?}", e);
                panic!();
            }
        }
        false
    }

    fn reset(&mut self) {
        for key in self.pressed_keys.drain() {
            self.lua_ue_tx.send(LuaToUe::ReleaseKey(key)).unwrap();
        }
        self.should_exit = false;
    }

    fn to_key(&self, key: &str) -> i32 {
        match key {
            "forward" => self.config.forward,
            "backward" => self.config.backward,
            "left" => self.config.left,
            "right" => self.config.right,
            "jump" => self.config.jump,
            "crouch" => self.config.crouch,
            "menu" => self.config.menu,
            _ => {
                log!("Invalid Key: {}", key);
                panic!()
            }
        }
    }
}

impl LuaInterface for GameInterface {
    fn step(&mut self) -> Response<Event> {
        self.lua_ue_tx.send(LuaToUe::AdvanceFrame).unwrap();
        if self.syscall() { return Response::ExitPlease }
        match self.ue_lua_rx.recv().unwrap() {
            UeToLua::Tick => Response::Result(Event::Stopped),
            UeToLua::NewGame => Response::Result(Event::NewGame),
        }
    }

    fn press_key(&mut self, key: String) -> Response<()> {
        if self.syscall() { return Response::ExitPlease }
        let key = self.to_key(&key);
        self.pressed_keys.insert(key);
        self.lua_ue_tx.send(LuaToUe::PressKey(key)).unwrap();
        Response::Result(())
    }

    fn release_key(&mut self, key: String) -> Response<()> {
        if self.syscall() { return Response::ExitPlease }
        let key = self.to_key(&key);
        self.pressed_keys.remove(&key);
        self.lua_ue_tx.send(LuaToUe::ReleaseKey(key)).unwrap();
        Response::Result(())
    }

    fn move_mouse(&mut self, x: i32, y: i32) -> Response<()> {
        if self.syscall() { return Response::ExitPlease }
        self.lua_ue_tx.send(LuaToUe::MoveMouse(x, y)).unwrap();
        Response::Result(())
    }

    fn get_delta(&mut self) -> Response<f64> {
        if self.syscall() { return Response::ExitPlease }
        Response::Result(FApp::delta())
    }

    fn set_delta(&mut self, delta: f64) -> Response<()> {
        if self.syscall() { return Response::ExitPlease }
        FApp::set_delta(delta);
        Response::Result(())
    }

    fn get_location(&mut self) -> Response<(f32, f32, f32)> {
        if self.syscall() { return Response::ExitPlease }
        Response::Result(AMyCharacter::location())
    }

    fn set_location(&mut self, x: f32, y: f32, z: f32) -> Response<()> {
        if self.syscall() { return Response::ExitPlease }
        AMyCharacter::set_location(x, y, z);
        Response::Result(())
    }

    fn get_rotation(&mut self) -> Response<(f32, f32, f32)> {
        if self.syscall() { return Response::ExitPlease }
        Response::Result(AController::rotation())
    }

    fn set_rotation(&mut self, pitch: f32, yaw: f32, roll: f32) -> Response<()> {
        if self.syscall() { return Response::ExitPlease }
        AController::set_rotation(pitch, yaw, roll);
        Response::Result(())
    }

    fn get_velocity(&mut self) -> Response<(f32, f32, f32)> {
        if self.syscall() { return Response::ExitPlease }
        Response::Result(AMyCharacter::velocity())
    }

    fn set_velocity(&mut self, x: f32, y: f32, z: f32) -> Response<()> {
        if self.syscall() { return Response::ExitPlease }
        AMyCharacter::set_velocity(x, y, z);
        Response::Result(())
    }

    fn get_acceleration(&mut self) -> Response<(f32, f32, f32)> {
        if self.syscall() { return Response::ExitPlease }
        Response::Result(AMyCharacter::acceleration())
    }

    fn set_acceleration(&mut self, x: f32, y: f32, z: f32) -> Response<()> {
        if self.syscall() { return Response::ExitPlease }
        AMyCharacter::set_acceleration(x, y, z);
        Response::Result(())
    }

    fn wait_for_new_game(&mut self) -> Response<()> {
        loop {
            match self.step() {
                Response::ExitPlease => return Response::ExitPlease,
                Response::Result(Event::Stopped) => continue,
                Response::Result(Event::NewGame) => return Response::Result(()),
            }
        }
    }

    fn print(&mut self, s: String) -> Response<()> {
        if self.syscall() { return Response::ExitPlease }
        self.lua_stream_tx.send(LuaToStream::Print(s)).unwrap();
        Response::Result(())
    }

    fn sleep(&mut self, time: u64) -> Response<()> {
        if self.syscall() { return Response::ExitPlease }
        thread::sleep(Duration::new(time, 0));
        Response::Result(())
    }
}
