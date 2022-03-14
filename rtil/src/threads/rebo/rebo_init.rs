use std::io::ErrorKind;
use std::path::PathBuf;
use crossbeam_channel::{Sender, TryRecvError};
use clipboard::{ClipboardProvider, ClipboardContext};
use rebo::{ExecError, ReboConfig, Stdlib, VmContext, Output, Value, DisplayValue, IncludeDirectoryConfig};
use itertools::Itertools;
use websocket::{ClientBuilder, Message, OwnedMessage, WebSocketError};
use crate::native::{AMyCharacter, AMyHud, FApp, LevelState, UWorld};
use protocol::{Request, Response};
use crate::threads::{Config, ReboToStream, ReboToUe, StreamToRebo, UeToRebo};
use super::STATE;

pub fn create_config(rebo_stream_tx: Sender<ReboToStream>) -> ReboConfig {
    let mut cfg = ReboConfig::new()
        .stdlib(Stdlib::all() - Stdlib::PRINT)
        .interrupt_interval(100)
        .interrupt_function(interrupt_function)
        .diagnostic_output(Output::buffered(move |s| {
            log!("{}", s);
            eprintln!("{}", s);
            rebo_stream_tx.send(ReboToStream::Print(s)).unwrap()
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
        .add_function(get_level_state)
        .add_function(wait_for_new_game)
        .add_function(draw_line)
        .add_function(draw_text)
        .add_function(project)
        .add_function(spawn_pawn)
        .add_function(destroy_pawn)
        .add_function(move_pawn)
        .add_function(connect_to_server)
        .add_function(disconnect_from_server)
        .add_function(join_multiplayer_room)
        .add_function(move_on_server)
        .add_function(set_level)
        .add_function(is_windows)
        .add_function(is_linux)
        .add_function(get_clipboard)
        .add_function(set_clipboard)
        .add_external_type(Key)
        .add_external_type(Location)
        .add_external_type(Rotation)
        .add_external_type(Velocity)
        .add_external_type(Acceleration)
        .add_external_type(Vector)
        .add_external_type(Line)
        .add_external_type(Color)
        .add_external_type(DrawText)
        .add_external_type(LevelState)
        .add_required_rebo_function(on_key_down)
        .add_required_rebo_function(on_key_up)
        .add_required_rebo_function(draw_hud)
        .add_required_rebo_function(player_joined_multiplayer_room)
        .add_required_rebo_function(player_left_multiplayer_room)
        .add_required_rebo_function(player_moved)
        .add_required_rebo_function(on_level_state_change)
    ;
    if let Some(working_dir) = &STATE.lock().unwrap().as_ref().unwrap().working_dir {
        cfg = cfg.include_directory(IncludeDirectoryConfig::Path(PathBuf::from(working_dir)));
    }
    cfg
}

pub enum Event {
    Stopped,
    NewGame,
}

/// Check internal state and channels to see if we should stop.
fn interrupt_function<'a, 'i>(_vm: &mut VmContext<'a, '_, '_, 'i>) -> Result<(), ExecError<'a, 'i>> {
    loop {
        let result = STATE.lock().unwrap().as_ref().unwrap().stream_rebo_rx.try_recv();
        match result {
            Ok(res) => match res {
                StreamToRebo::Config(cfg) => {
                    log!("Set Config while running");
                    STATE.lock().unwrap().as_mut().unwrap().config = cfg;
                }
                StreamToRebo::WorkingDir(_) => {
                    log!("Got WorkingDir, but can't set it during execution");
                    panic!()
                }
                StreamToRebo::Start(_) => {
                    log!("Got StreamToRebo::Start but rebo is already running");
                    panic!()
                }
                StreamToRebo::Stop => {
                    log!("Should Exit");
                    return Err(ExecError::Panic);
                }
            }
            Err(TryRecvError::Empty) => return Ok(()),
            Err(e) => {
                log!("Error stream_rebo_rx.try_recv: {:?}", e);
                panic!();
            }
        }
    }
}

#[rebo::function(raw("print"))]
fn print(..: _) {
    let joined = args.as_slice().iter().map(DisplayValue).join(", ");
    log!("{}", joined);
    STATE.lock().unwrap().as_ref().unwrap().rebo_stream_tx.send(ReboToStream::Print(joined)).unwrap();
}

#[rebo::function(raw("Tas::step"))]
fn step() {
    step_internal(vm)?;
}
fn step_internal<'a, 'i>(vm: &mut VmContext<'a, '_, '_, 'i>) -> Result<Event, ExecError<'a, 'i>> {
    // get level state before and after we advance the UE frame to see changes created by Refunct itself
    let old_level_state = LevelState::get();

    if let Some(delta) = STATE.lock().unwrap().as_ref().unwrap().delta {
        FApp::set_delta(delta);
    }

    STATE.lock().unwrap().as_ref().unwrap().rebo_ue_tx.send(ReboToUe::AdvanceFrame).unwrap();
    loop {
        let res = STATE.lock().unwrap().as_ref().unwrap().ue_rebo_rx.recv().unwrap();
        match res {
            e @ UeToRebo::Tick | e @ UeToRebo::NewGame => {
                // call level-state event function
                let new_level_state = LevelState::get();
                if old_level_state != new_level_state {
                    on_level_state_change(vm, old_level_state, new_level_state)?;
                }
                return Ok(match e {
                    UeToRebo::Tick => Event::Stopped,
                    UeToRebo::NewGame => Event::NewGame,
                    _ => unreachable!()
                });
            },
            UeToRebo::KeyDown(key, char, repeat) => on_key_down(vm, key, char, repeat)?,
            UeToRebo::KeyUp(key, char, repeat) => on_key_up(vm, key, char, repeat)?,
            UeToRebo::DrawHud => draw_hud(vm)?,
            UeToRebo::AMyCharacterSpawned(_) => unreachable!(),
        }
        loop {
            // let mut state = STATE.lock().unwrap();
            // if state.as_ref().unwrap().websocket.is_none() {
            //     break;
            // }
            if STATE.lock().unwrap().as_ref().unwrap().websocket.is_none() {
                break;
            }
            // let websocket = state.as_mut().unwrap().websocket.as_mut().unwrap();
            // websocket.set_nonblocking(true).unwrap();
            // let res = websocket.recv_message();
            // websocket.set_nonblocking(false).unwrap();
            STATE.lock().unwrap().as_mut().unwrap().websocket.as_mut().unwrap().set_nonblocking(true).unwrap();
            let res = STATE.lock().unwrap().as_mut().unwrap().websocket.as_mut().unwrap().recv_message();
            STATE.lock().unwrap().as_mut().unwrap().websocket.as_mut().unwrap().set_nonblocking(false).unwrap();
            let response = match res {
                Ok(OwnedMessage::Text(text)) => serde_json::from_str(&text).unwrap(),
                Ok(OwnedMessage::Binary(_) | OwnedMessage::Ping(_) | OwnedMessage::Pong(_)) => continue,
                Err(WebSocketError::IoError(io)) if io.kind() == ErrorKind::WouldBlock => break,
                Ok(OwnedMessage::Close(_)) | Err(_) => {
                    // drop(state.as_mut().unwrap().websocket.take());
                    drop(STATE.lock().unwrap().as_mut().unwrap().websocket.take());
                    break
                },
            };
            // drop(state);
            match response {
                Response::PlayerJoinedRoom(id, x, y, z) => player_joined_multiplayer_room(vm, id.id(), Location { x, y, z})?,
                Response::PlayerLeftRoom(id) => player_left_multiplayer_room(vm, id.id())?,
                Response::MoveOther(id, x, y, z) => player_moved(vm, id.id(), Location { x, y, z })?,
            }
        }
        // We aren't actually advancing a frame, but just returning from the
        // key event or drawhud interceptor.
        STATE.lock().unwrap().as_ref().unwrap().rebo_ue_tx.send(ReboToUe::AdvanceFrame).unwrap();
    }
}

#[rebo::required_rebo_functions]
extern "rebo" {
    fn on_key_down(key_code: i32, character_code: u32, is_repeat: bool);
    fn on_key_up(key_code: i32, character_code: u32, is_repeat: bool);
    fn draw_hud();
    fn player_joined_multiplayer_room(id: u32, loc: Location);
    fn player_left_multiplayer_room(id: u32);
    fn player_moved(id: u32, loc: Location);
    fn on_level_state_change(old: LevelState, new: LevelState);
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
    state.rebo_ue_tx.send(ReboToUe::PressKey(key, key as u32, false)).unwrap();
}
#[rebo::function("Tas::release_key")]
fn release_key(key: Key) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    let key = key.to_key(&state.config);
    state.pressed_keys.remove(&key);
    state.rebo_ue_tx.send(ReboToUe::ReleaseKey(key, key as u32, false)).unwrap();
}
#[rebo::function("Tas::key_down")]
fn key_down(key_code: i32, character_code: u32, is_repeat: bool) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    state.pressed_keys.insert(key_code);
    state.rebo_ue_tx.send(ReboToUe::PressKey(key_code, character_code, is_repeat)).unwrap();
}
#[rebo::function("Tas::key_up")]
fn key_up(key_code: i32, character_code: u32, is_repeat: bool) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    state.pressed_keys.remove(&key_code);
    state.rebo_ue_tx.send(ReboToUe::ReleaseKey(key_code, character_code, is_repeat)).unwrap();
}
#[rebo::function("Tas::move_mouse")]
fn move_mouse(x: i32, y: i32) {
    STATE.lock().unwrap().as_ref().unwrap().rebo_ue_tx.send(ReboToUe::MoveMouse(x, y)).unwrap();
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
#[rebo::function("Tas::get_level_state")]
fn get_level_state() -> LevelState {
    LevelState::get()
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
    STATE.lock().unwrap().as_ref().unwrap().rebo_ue_tx.send(ReboToUe::DrawLine(line.startx, line.starty, line.endx, line.endy, (line.color.red, line.color.green, line.color.blue, line.color.alpha), line.thickness)).unwrap();
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
    STATE.lock().unwrap().as_ref().unwrap().rebo_ue_tx.send(ReboToUe::DrawText(text.text, (text.color.red, text.color.green, text.color.blue, text.color.alpha), text.x, text.y, text.scale, text.scale_position)).unwrap();
}
#[derive(Debug, Clone, Copy, rebo::ExternalType)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}
#[rebo::function("Tas::project")]
fn project(vec: Vector) -> Vector {
    let (x, y, z) = AMyHud::project(vec.x, vec.y, vec.z);
    Vector { x, y, z }
}
#[rebo::function("Tas::spawn_pawn")]
fn spawn_pawn() -> u32 {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    state.rebo_ue_tx.send(ReboToUe::SpawnAMyCharacter).unwrap();
    let spawned = state.ue_rebo_rx.recv().unwrap();
    let my_character = match spawned {
        UeToRebo::AMyCharacterSpawned(c) => c,
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
#[rebo::function("Tas::connect_to_server")]
fn connect_to_server() {
    let client = ClientBuilder::new(&format!("ws://localhost:8080/ws")).unwrap().connect(None);
    // let client = ClientBuilder::new(&format!("wss://refunct-tas.oberien.de/ws")).unwrap().connect(None).unwrap();
    let client = match client {
        Ok(client) => client,
        Err(e) => {
            log!("couldn't connect to server: {e:?}");
            return Ok(Value::Unit)
        }
    };
    log!("connected to server");
    STATE.lock().unwrap().as_mut().unwrap().websocket = Some(client);
}
#[rebo::function("Tas::disconnect_from_server")]
fn disconnect_from_server() {
    STATE.lock().unwrap().as_mut().unwrap().websocket.take();
}
#[rebo::function("Tas::join_multiplayer_room")]
fn join_multiplayer_room(room: String, loc: Location) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    if state.websocket.is_none() {
        log!("called join room without active websocket session");
        // TODO: error propagation?
        return Ok(Value::Unit);
    }
    let msg = Request::JoinRoom(room, loc.x, loc.y, loc.z);
    let msg = serde_json::to_string(&msg).unwrap();
    let msg = Message::text(msg);
    if let Err(e) = state.websocket.as_mut().unwrap().send_message(&msg) {
        log!("error sending join room request: {:?}", e);
        state.websocket.take();
    }
}
#[rebo::function("Tas::move_on_server")]
fn move_on_server(loc: Location) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    if state.websocket.is_none() {
        log!("called move without active websocket session");
        // TODO: error propagation?
        return Ok(Value::Unit);
    }
    let msg = Request::MoveSelf(loc.x, loc.y, loc.z);
    let msg = serde_json::to_string(&msg).unwrap();
    let msg = Message::text(msg);
    if let Err(e) = state.websocket.as_mut().unwrap().send_message(&msg) {
        log!("error sending move request: {:?}", e);
        state.websocket.take();
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
#[rebo::function("Tas::set_clipboard")]
fn set_clipboard(content: String) {
    let _ = (|| {
        let mut ctx: ClipboardContext = match ClipboardProvider::new() {
            Ok(ctx) => ctx,
            Err(_) => return,
        };
        let _ = ctx.set_contents(content);
    })();
}
