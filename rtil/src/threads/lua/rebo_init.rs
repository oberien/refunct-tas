use std::net::TcpStream;
use std::path::PathBuf;
use std::thread;
use crossbeam_channel::{Sender, TryRecvError};
use clipboard::{ClipboardProvider, ClipboardContext};
use rebo::{ExecError, ReboConfig, Stdlib, VmContext, Output, Value, DisplayValue};
use itertools::Itertools;
use native::{AMyCharacter, AMyHud, FApp, LevelState, UWorld};
use protocol::Message;
use threads::{Config, LuaToStream, LuaToUe, StreamToLua, UeToLua};
use super::STATE;

pub fn create_config(lua_stream_tx: Sender<LuaToStream>) -> ReboConfig {
    let mut cfg = ReboConfig::new()
        .stdlib(Stdlib::all() - Stdlib::PRINT)
        .interrupt_interval(100)
        .interrupt_function(interrupt_function)
        .diagnostic_output(Output::buffered(move |s| {
            log!("{}", s);
            eprintln!("{}", s);
            lua_stream_tx.send(LuaToStream::Print(s)).unwrap()
        }))
        .add_function(print)
        .add_function(step)
        .add_function(press_key)
        .add_function(release_key)
        .add_function(key_down)
        .add_function(key_up)
        .add_function(move_mouse)
        .add_function(get_last_frame_delta)
        .add_function(get_delta)
        .add_function(set_delta)
        .add_function(get_location)
        .add_function(set_location)
        .add_function(get_rotation)
        .add_function(set_rotation)
        .add_function(get_velocity)
        .add_function(set_velocity)
        .add_function(get_acceleration)
        .add_function(set_acceleration)
        .add_function(wait_for_new_game)
        .add_function(draw_line)
        .add_function(draw_text)
        .add_function(project)
        .add_function(spawn_pawn)
        .add_function(destroy_pawn)
        .add_function(move_pawn)
        .add_function(tcp_connect)
        .add_function(tcp_disconnect)
        .add_function(tcp_move)
        .add_function(set_level)
        .add_function(is_windows)
        .add_function(is_linux)
        .add_function(get_clipboard)
        .add_external_type(Key)
        .add_external_type(Location)
        .add_external_type(Rotation)
        .add_external_type(Velocity)
        .add_external_type(Acceleration)
        .add_external_type(Vector)
        .add_external_type(Line)
        .add_external_type(Color)
        .add_external_type(DrawText)
        .add_required_rebo_function(on_key_down)
        .add_required_rebo_function(on_key_up)
        .add_required_rebo_function(draw_hud)
        .add_required_rebo_function(tcp_joined)
        .add_required_rebo_function(tcp_left)
        .add_required_rebo_function(tcp_moved)
        .add_required_rebo_function(on_level_change)
        .add_required_rebo_function(on_reset)
    ;
    if let Some(working_dir) = &STATE.lock().unwrap().as_ref().unwrap().working_dir {
        cfg = cfg.include_directory(PathBuf::from(working_dir));
    }
    cfg
}

pub enum Event {
    Stopped,
    NewGame,
}

/// Check internal state and channels to see if we should stop.
fn interrupt_function(_vm: &mut VmContext) -> Result<(), ExecError> {
    loop {
        let result = STATE.lock().unwrap().as_ref().unwrap().stream_lua_rx.try_recv();
        match result {
            Ok(res) => match res {
                StreamToLua::Config(cfg) => {
                    log!("Set Config while running");
                    STATE.lock().unwrap().as_mut().unwrap().config = cfg;
                }
                StreamToLua::WorkingDir(_) => {
                    log!("Got WorkingDir, but can't set it during execution");
                    panic!()
                }
                StreamToLua::Start(_) => {
                    log!("Got StreamToLua::Start but rebo is already running");
                    panic!()
                }
                StreamToLua::Stop => {
                    log!("Should Exit");
                    return Err(ExecError::Panic);
                }
            }
            Err(TryRecvError::Empty) => return Ok(()),
            Err(e) => {
                log!("Error stream_lua_rx.try_recv: {:?}", e);
                panic!();
            }
        }
    }
}

#[rebo::function(raw("print"))]
fn print(..: _) {
    let joined = args.as_slice().into_iter().map(|arg| DisplayValue(arg)).join(", ");
    log!("{}", joined);
    STATE.lock().unwrap().as_ref().unwrap().lua_stream_tx.send(LuaToStream::Print(joined)).unwrap();
}

#[rebo::function(raw("Tas::step"))]
fn step() {
    step_internal(vm)?;
}
fn step_internal(vm: &mut VmContext) -> Result<Event, ExecError> {
    // get level state before and after we advance the UE frame to see changes created by Refunct itself
    let level_state = LevelState::get();

    if let Some(delta) = STATE.lock().unwrap().as_ref().unwrap().delta {
        FApp::set_delta(delta);
    }

    STATE.lock().unwrap().as_ref().unwrap().lua_ue_tx.send(LuaToUe::AdvanceFrame).unwrap();
    loop {
        let res = STATE.lock().unwrap().as_ref().unwrap().ue_lua_rx.recv().unwrap();
        match res {
            e @ UeToLua::Tick | e @ UeToLua::NewGame => {
                // call level-state event functions
                let new_level_state = LevelState::get();
                // level changed
                if level_state.level != new_level_state.level
                    // new game but no level change will be triggered because we hit new game
                    // when level was still 0
                    || e == UeToLua::NewGame && level_state.level == 0
                {
                    on_level_change(vm, new_level_state.level)?;
                }
                if level_state.resets != new_level_state.resets {
                    on_reset(vm, new_level_state.resets)?;
                }
                return Ok(match e {
                    UeToLua::Tick => Event::Stopped,
                    UeToLua::NewGame => Event::NewGame,
                    _ => unreachable!()
                });
            },
            UeToLua::KeyDown(key, char, repeat) => on_key_down(vm, key, char, repeat)?,
            UeToLua::KeyUp(key, char, repeat) => on_key_up(vm, key, char, repeat)?,
            UeToLua::DrawHud => draw_hud(vm)?,
            UeToLua::AMyCharacterSpawned(_) => unreachable!(),
        }
        loop {
            if STATE.lock().unwrap().as_ref().unwrap().tcp_stream.is_none() {
                break;
            }
            let res = STATE.lock().unwrap().as_ref().unwrap().tcp_stream.as_ref().unwrap().1.try_recv();
            match res {
                Ok(Message::PlayerJoinedRoom(id, x, y, z)) => tcp_joined(vm, id, x, y, z)?,
                Ok(Message::PlayerLeftRoom(id)) => tcp_left(vm, id)?,
                Ok(Message::MoveOther(id, x, y, z)) => tcp_moved(vm, id, x, y, z)?,
                Ok(msg @ Message::JoinRoom(..))
                | Ok(msg @ Message::MoveSelf(..)) => {
                    log!("got unexpected message from server, ignoring: {:?}", msg);
                }
                Err(TryRecvError::Disconnected) => drop(STATE.lock().unwrap().as_mut().unwrap().tcp_stream.take()),
                Err(TryRecvError::Empty) => break,
            }
        }
        // We aren't actually advancing a frame, but just returning from the
        // key event or drawhud interceptor.
        STATE.lock().unwrap().as_ref().unwrap().lua_ue_tx.send(LuaToUe::AdvanceFrame).unwrap();
    }
}

#[rebo::required_rebo_functions]
extern "rebo" {
    fn on_key_down(key_code: i32, character_code: u32, is_repeat: bool);
    fn on_key_up(key_code: i32, character_code: u32, is_repeat: bool);
    fn draw_hud();
    fn tcp_joined(id: u32, x: f32, y: f32, z: f32);
    fn tcp_left(id: u32);
    fn tcp_moved(id: u32, x: f32, y: f32, z: f32);
    fn on_level_change(level: i32);
    fn on_reset(reset: i32);
}

#[derive(Debug, Clone, Copy, rebo::ExternalType)]
enum Key {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
    Crouch,
    Menu,
}
impl Key {
    fn to_key(self, config: &Config) -> i32 {
        match self {
            Key::Forward => config.forward,
            Key::Backward => config.backward,
            Key::Left => config.left,
            Key::Right => config.right,
            Key::Jump => config.jump,
            Key::Crouch => config.crouch,
            Key::Menu => config.menu,
        }
    }
}

#[rebo::function("Tas::press_key")]
fn press_key(key: Key) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    let key = key.to_key(&state.config);
    state.pressed_keys.insert(key);
    state.lua_ue_tx.send(LuaToUe::PressKey(key, key as u32, false)).unwrap();
}
#[rebo::function("Tas::release_key")]
fn release_key(key: Key) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    let key = key.to_key(&state.config);
    state.pressed_keys.remove(&key);
    state.lua_ue_tx.send(LuaToUe::ReleaseKey(key, key as u32, false)).unwrap();
}
#[rebo::function("Tas::key_down")]
fn key_down(key_code: i32, character_code: u32, is_repeat: bool) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    state.pressed_keys.insert(key_code);
    state.lua_ue_tx.send(LuaToUe::PressKey(key_code, character_code, is_repeat)).unwrap();
}
#[rebo::function("Tas::key_up")]
fn key_up(key_code: i32, character_code: u32, is_repeat: bool) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    state.pressed_keys.remove(&key_code);
    state.lua_ue_tx.send(LuaToUe::ReleaseKey(key_code, character_code, is_repeat)).unwrap();
}
#[rebo::function("Tas::move_mouse")]
fn move_mouse(x: i32, y: i32) {
    STATE.lock().unwrap().as_ref().unwrap().lua_ue_tx.send(LuaToUe::MoveMouse(x, y)).unwrap();
}
#[rebo::function("Tas::get_last_frame_delta")]
fn get_last_frame_delta() -> f64 {
    FApp::delta()
}
#[rebo::function("Tas::get_delta")]
fn get_delta() -> Option<f64> {
    STATE.lock().unwrap().as_mut().unwrap().delta
}
#[rebo::function("Tas::set_delta")]
fn set_delta(delta: Option<f64>) {
    STATE.lock().unwrap().as_mut().unwrap().delta = delta;
}
#[derive(Debug, Clone, Copy, rebo::ExternalType)]
struct Location {
    x: f32,
    y: f32,
    z: f32,
}
#[rebo::function("Tas::get_location")]
fn get_location() -> Location {
    let (x, y, z) = AMyCharacter::get_player().location();
    Location { x, y, z }
}
#[rebo::function("Tas::set_location")]
fn set_location(loc: Location) {
    AMyCharacter::get_player().set_location(loc.x, loc.y, loc.z);
}
#[derive(Debug, Clone, Copy, rebo::ExternalType)]
struct Rotation {
    pitch: f32,
    yaw: f32,
    roll: f32,
}
#[rebo::function("Tas::get_rotation")]
fn get_rotation() -> Rotation {
    let (pitch, yaw, roll) = AMyCharacter::get_player().rotation();
    Rotation { pitch, yaw, roll }
}
#[rebo::function("Tas::set_rotation")]
fn set_rotation(rot: Rotation) {
    AMyCharacter::get_player().set_rotation(rot.pitch, rot.yaw, rot.roll);
}
#[derive(Debug, Clone, Copy, rebo::ExternalType)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}
#[rebo::function("Tas::get_velocity")]
fn get_velocity() -> Velocity {
    let (x, y, z) = AMyCharacter::get_player().velocity();
    Velocity { x, y, z }
}
#[rebo::function("Tas::set_velocity")]
fn set_velocity(vel: Velocity) {
    AMyCharacter::get_player().set_velocity(vel.x, vel.y, vel.z);
}
#[derive(Debug, Clone, Copy, rebo::ExternalType)]
struct Acceleration {
    x: f32,
    y: f32,
    z: f32,
}
#[rebo::function("Tas::get_acceleration")]
fn get_acceleration() -> Acceleration {
    let (x, y, z) = AMyCharacter::get_player().acceleration();
    Acceleration { x, y, z }
}
#[rebo::function("Tas::set_acceleration")]
fn set_acceleration(acc: Acceleration) {
    AMyCharacter::get_player().set_acceleration(acc.x, acc.y, acc.z);
}
#[rebo::function(raw("Tas::wait_for_new_game"))]
fn wait_for_new_game() {
    loop {
        match step_internal(vm)? {
            Event::Stopped => continue,
            Event::NewGame => break,
        }
    }
}
#[derive(Debug, Clone, Copy, rebo::ExternalType)]
struct Line {
    startx: f32,
    starty: f32,
    endx: f32,
    endy: f32,
    color: Color,
    thickness: f32,
}
#[derive(Debug, Clone, Copy, rebo::ExternalType)]
struct Color {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}
#[rebo::function("Tas::draw_line")]
fn draw_line(line: Line) {
    STATE.lock().unwrap().as_ref().unwrap().lua_ue_tx.send(LuaToUe::DrawLine(line.startx, line.starty, line.endx, line.endy, (line.color.red, line.color.green, line.color.blue, line.color.alpha), line.thickness)).unwrap();
}
#[derive(Debug, Clone, rebo::ExternalType)]
struct DrawText {
    text: String,
    color: Color,
    x: f32,
    y: f32,
    scale: f32,
    scale_position: bool,
}
#[rebo::function("Tas::draw_text")]
fn draw_text(text: DrawText) {
    STATE.lock().unwrap().as_ref().unwrap().lua_ue_tx.send(LuaToUe::DrawText(text.text, (text.color.red, text.color.green, text.color.blue, text.color.alpha), text.x, text.y, text.scale, text.scale_position)).unwrap();
}
#[derive(Debug, Clone, Copy, rebo::ExternalType)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}
#[rebo::function("Tas::project")]
fn project(loc: Vector) -> Vector {
    let (x, y, z) = AMyHud::project(loc.x, loc.y, loc.z);
    Vector { x, y, z }
}
#[rebo::function("Tas::spawn_pawn")]
fn spawn_pawn() -> u32 {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    state.lua_ue_tx.send(LuaToUe::SpawnAMyCharacter).unwrap();
    let spawned = state.ue_lua_rx.recv().unwrap();
    let my_character = match spawned {
        UeToLua::AMyCharacterSpawned(c) => c,
        _ => unreachable!(),
    };
    let id = state.pawn_id;
    state.pawn_id += 1;
    state.pawns.insert(id, my_character);
    id
}
#[rebo::function("Tas::destroy_pawn")]
fn destroy_pawn(pawn_id: u32) {
    let my_character = STATE.lock().unwrap().as_mut().unwrap().pawns.remove(&pawn_id).expect("pawn_id not valid anymore");
    UWorld::destroy_amycharaccter(my_character);
}
#[rebo::function("Tas::move_pawn")]
fn move_pawn(pawn_id: u32, loc: Location) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    let my_character = state.pawns.get_mut(&pawn_id).expect("pawn_id not valid");
    my_character.set_location(loc.x, loc.y, loc.z);
}
#[rebo::function("Tas::tcp_connect")]
fn tcp_connect(server_port: String) {
    let stream = TcpStream::connect(server_port).expect("Could not connect to server");
    let mut read = stream.try_clone().unwrap();
    let (msg_tx, msg_rx) = crossbeam_channel::unbounded();
    thread::spawn(move || {
        loop {
            let msg = Message::deserialize(&mut read).unwrap();
            msg_tx.send(msg).unwrap();
        }
    });
    STATE.lock().unwrap().as_mut().unwrap().tcp_stream = Some((stream, msg_rx));
}
#[rebo::function("Tas::tcp_disconnect")]
fn tcp_disconnect() {
    STATE.lock().unwrap().as_mut().unwrap().tcp_stream.take();
}
#[rebo::function("Tas::tcp_join_room")]
fn tcp_join_room(room: String, loc: Location) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    if state.tcp_stream.is_none() {
        log!("called join room without active tcp session");
        // TODO: error propagation?
        return Ok(Value::Unit);
    }
    let msg = Message::JoinRoom(room, loc.x, loc.y, loc.z);
    if let Err(e) = msg.serialize(&mut state.tcp_stream.as_mut().unwrap().0) {
        log!("error sending join room request: {:?}", e);
        state.tcp_stream.take();
    }
}
#[rebo::function("Tas::tcp_move")]
fn tcp_move(loc: Location) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    if state.tcp_stream.is_none() {
        log!("called move without active tcp session");
        // TODO: error propagation?
        return Ok(Value::Unit);
    }
    let msg = Message::MoveSelf(loc.x, loc.y, loc.z);
    if let Err(e) = msg.serialize(&mut state.tcp_stream.as_mut().unwrap().0) {
        log!("error sending join room request: {:?}", e);
        state.tcp_stream.take();
    }
}
#[rebo::function("Tas::set_level")]
fn set_level(level: i32) {
    LevelState::set_level(level);
}
#[rebo::function("Tas::is_windows")]
fn is_windows() -> bool {
    cfg!(windows)
}
#[rebo::function("Tas::is_linux")]
fn is_linux() -> bool {
    !cfg!(windows)
}
#[rebo::function("Tas::get_clipboard")]
fn get_clipboard() -> String {
    (|| {
        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
        ctx.get_contents()
    })().unwrap_or_default()
}
