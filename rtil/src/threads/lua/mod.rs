use std::thread;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashSet, HashMap};
use std::net::TcpStream;
use std::io::{Write, Read};
use std::sync::Mutex;

use lua::{Lua, LuaInterface, LuaEvents, Event, IfaceResult, IfaceError, Context, LuaResult};
use failure::Fail;
use protocol::Message;
use crossbeam_channel::{Sender, Receiver, TryRecvError};

use threads::{StreamToLua, LuaToStream, LuaToUe, UeToLua, Config};
use native::{AMyCharacter, FApp, UWorld, AMyHud, LevelState};

mod rebo_init;

lazy_static! {
    static ref STATE: Mutex<Option<State>> = Mutex::new(None);
}

struct State {
    delta: Option<f64>,
    stream_lua_rx: Receiver<StreamToLua>,
    lua_stream_tx: Sender<LuaToStream>,
    lua_ue_tx: Sender<LuaToUe>,
    ue_lua_rx: Receiver<UeToLua>,
    config: Config,
    working_dir: Option<String>,
    pressed_keys: HashSet<i32>,
    tcp_stream: Option<(TcpStream, Receiver<Message>)>,
    pawns: HashMap<u32, AMyCharacter>,
    pawn_id: u32,
}

pub fn run(stream_lua_rx: Receiver<StreamToLua>, lua_stream_tx: Sender<LuaToStream>,
           lua_ue_tx: Sender<LuaToUe>, ue_lua_rx: Receiver<UeToLua>) {
    log!("starting lua thread");
    thread::spawn(move || {
        *STATE.lock().unwrap() = Some(State {
            delta: None,
            stream_lua_rx,
            lua_stream_tx,
            lua_ue_tx,
            ue_lua_rx,
            config: Config::default(),
            working_dir: None,
            pressed_keys: HashSet::new(),
            tcp_stream: None,
            pawns: HashMap::new(),
            pawn_id: 0,
        });

        loop {
            handle_rx();
        }
    });
}

fn handle_rx() {
    let res = STATE.lock().unwrap().as_ref().unwrap().stream_lua_rx.recv().unwrap();
    match res {
        StreamToLua::Stop => {},
        StreamToLua::Config(config) => {
            log!("Set config before running");
            STATE.lock().unwrap().as_mut().unwrap().config = config;
        },
        StreamToLua::WorkingDir(dir) => {
            log!("Set working dir");
            STATE.lock().unwrap().as_mut().unwrap().working_dir = Some(dir);
        }
        StreamToLua::Start(s) => {
            log!("Starting rebo...");
            log!("Cleaning ue_lua_rx...");
            let mut count = 0;
            while let Ok(_) = STATE.lock().unwrap().as_ref().unwrap().ue_lua_rx.try_recv() {
                count += 1;
            }
            log!("Removed {} messages", count);

            STATE.lock().unwrap().as_ref().unwrap().lua_ue_tx.send(LuaToUe::Stop).unwrap();
            let lua_stream_tx = STATE.lock().unwrap().as_ref().unwrap().lua_stream_tx.clone();
            let config = rebo_init::create_config(lua_stream_tx);
            log!("Executing rebo code.");
            rebo::run_with_config("file.re".to_string(), s, config);
            log!("Rebo execution done. Starting cleanup...");

            // reset STATE
            let mut state = STATE.lock().unwrap();
            let state = state.as_mut().unwrap();
            state.delta = None;
            state.tcp_stream.take();
            state.pawns.clear();
            state.pawn_id = 0;
            for key in state.pressed_keys.drain() {
                state.lua_ue_tx.send(LuaToUe::ReleaseKey(key, key as u32, false)).unwrap();
            }

            state.lua_ue_tx.send(LuaToUe::Resume).unwrap();
            state.lua_stream_tx.send(LuaToStream::MiDone).unwrap();
            log!("Cleanup finished.");
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
    working_dir: RefCell<Option<String>>,
    tcp_stream: RefCell<Option<(TcpStream, Receiver<Message>)>>,
    pawns: RefCell<HashMap<u32, AMyCharacter>>,
    pawn_id: RefCell<u32>,
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
                    self.tcp_stream.borrow_mut().take();
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
    fn step(&self, lua: Context<'_>) -> LuaResult<Event> {
        unimplemented!()
    }

    fn press_key(&self, key: String) -> IfaceResult<()> {
        self.syscall()?;
        let key = self.to_key(&key);
        self.pressed_keys.borrow_mut().insert(key);
        self.lua_ue_tx.send(LuaToUe::PressKey(key, key as u32, false)).unwrap();
        Ok(())
    }

    fn release_key(&self, key: String) -> IfaceResult<()> {
        self.syscall()?;
        let key = self.to_key(&key);
        self.pressed_keys.borrow_mut().remove(&key);
        self.lua_ue_tx.send(LuaToUe::ReleaseKey(key, key as u32, false)).unwrap();
        Ok(())
    }

    fn key_down(&self, key_code: i32, character_code: u32, is_repeat: bool) -> Result<(), IfaceError> {
        self.pressed_keys.borrow_mut().insert(key_code);
        self.lua_ue_tx.send(LuaToUe::PressKey(key_code, character_code, is_repeat)).unwrap();
        Ok(())
    }

    fn key_up(&self, key_code: i32, character_code: u32, is_repeat: bool) -> Result<(), IfaceError> {
        self.pressed_keys.borrow_mut().remove(&key_code);
        self.lua_ue_tx.send(LuaToUe::ReleaseKey(key_code, character_code, is_repeat)).unwrap();
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
        Ok(AMyCharacter::get_player().location())
    }

    fn set_location(&self, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        self.syscall()?;
        AMyCharacter::get_player().set_location(x, y, z);
        Ok(())
    }

    fn get_rotation(&self) -> IfaceResult<(f32, f32, f32)> {
        self.syscall()?;
        Ok(AMyCharacter::get_player().rotation())
    }

    fn set_rotation(&self, pitch: f32, yaw: f32, roll: f32) -> IfaceResult<()> {
        self.syscall()?;
        AMyCharacter::get_player().set_rotation(pitch, yaw, roll);
        Ok(())
    }

    fn get_velocity(&self) -> IfaceResult<(f32, f32, f32)> {
        self.syscall()?;
        Ok(AMyCharacter::get_player().velocity())
    }

    fn set_velocity(&self, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        self.syscall()?;
        AMyCharacter::get_player().set_velocity(x, y, z);
        Ok(())
    }

    fn get_acceleration(&self) -> IfaceResult<(f32, f32, f32)> {
        self.syscall()?;
        Ok(AMyCharacter::get_player().acceleration())
    }

    fn set_acceleration(&self, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        self.syscall()?;
        AMyCharacter::get_player().set_acceleration(x, y, z);
        Ok(())
    }

    fn wait_for_new_game(&self, lua: Context<'_>) -> LuaResult<()> {
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

    fn project(&self, x: f32, y: f32, z: f32) -> IfaceResult<(f32, f32, f32)> {
        self.syscall()?;
        Ok(AMyHud::project(x, y, z))
    }

    fn print(&self, s: String) -> IfaceResult<()> {
        self.syscall()?;
        self.lua_stream_tx.send(LuaToStream::Print(s)).unwrap();
        Ok(())
    }

    fn working_dir(&self) -> IfaceResult<String> {
        Ok(self.working_dir.borrow().clone().unwrap())
    }

    fn spawn_pawn(&self) -> IfaceResult<u32> {
        self.syscall()?;
        self.lua_ue_tx.send(LuaToUe::SpawnAMyCharacter).unwrap();
        let spawned = self.ue_lua_rx.recv().unwrap();
        let my_character = match spawned {
            UeToLua::AMyCharacterSpawned(c) => c,
            _ => unreachable!(),
        };
        let id = *self.pawn_id.borrow();
        *self.pawn_id.borrow_mut() += 1;
        self.pawns.borrow_mut().insert(id, my_character);
        Ok(id)
    }

    fn destroy_pawn(&self, pawn_id: u32) -> IfaceResult<()> {
        let my_character = self.pawns.borrow_mut().remove(&pawn_id).expect("pawn_id not valid anymore");
        UWorld::destroy_amycharaccter(my_character);
        Ok(())
    }

    fn move_pawn(&self, pawn_id: u32, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        let mut borrow = self.pawns.borrow_mut();
        let my_character = borrow.get_mut(&pawn_id).expect("pawn_id not valid");
        my_character.set_location(x, y, z);
        Ok(())
    }

    fn tcp_connect(&self, server_port: String) -> IfaceResult<()> {
        self.syscall()?;
        let stream = TcpStream::connect(server_port)
            .expect("Could not connect to server");
        let mut read = stream.try_clone().unwrap();
        let (msg_tx, msg_rx) = crossbeam_channel::unbounded();
        thread::spawn(move || {
            loop {
                let msg = Message::deserialize(&mut read).unwrap();
                msg_tx.send(msg).unwrap();
            }
        });
        *self.tcp_stream.borrow_mut() = Some((stream, msg_rx));
        Ok(())
    }

    fn tcp_disconnect(&self) -> IfaceResult<()> {
        self.syscall()?;
        self.tcp_stream.borrow_mut().take();
        Ok(())
    }

    fn tcp_join_room(&self, room: String, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        self.syscall()?;
        if self.tcp_stream.borrow().is_none() {
            log!("called join room without active tcp session");
            // TODO: error propagation?
            return Ok(());
        }
        let msg = Message::JoinRoom(room, x, y, z);
        if let Err(e) = msg.serialize(&mut self.tcp_stream.borrow_mut().as_mut().unwrap().0) {
            log!("error sending join room request: {:?}", e);
            self.tcp_stream.borrow_mut().take();
        }
        Ok(())
    }

    fn tcp_move(&self, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        self.syscall()?;
        if self.tcp_stream.borrow().is_none() {
            log!("called move without active tcp session");
            // TODO: error propagation?
            return Ok(());
        }
        let msg = Message::MoveSelf(x, y, z);
        if let Err(e) = msg.serialize(&mut self.tcp_stream.borrow_mut().as_mut().unwrap().0) {
            log!("error sending join room request: {:?}", e);
            self.tcp_stream.borrow_mut().take();
        }
        Ok(())
    }

    fn set_level(&self, level: i32) -> IfaceResult<()> {
        LevelState::set_level(level);
        Ok(())
    }
}
