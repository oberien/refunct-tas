use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::time::Duration;
use crossbeam_channel::{Sender, TryRecvError};
use clipboard::{ClipboardProvider, ClipboardContext};
use rebo::{ExecError, ReboConfig, Stdlib, VmContext, Output, Value, DisplayValue, IncludeDirectoryConfig, Map};
use itertools::Itertools;
use websocket::{ClientBuilder, Message, OwnedMessage, WebSocketError};
use crate::native::{AMyCharacter, AMyHud, FApp, LevelState, UWorld};
use protocol::{Request, Response};
use crate::threads::{ReboToStream, ReboToUe, StreamToRebo, UeToRebo};
use super::STATE;
use serde::{Serialize, Deserialize};

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
        .add_function(step_yield)
        .add_function(load_settings)
        .add_function(store_settings)
        .add_function(list_recordings)
        .add_function(save_recording)
        .add_function(load_recording)
        .add_function(remove_recording)
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
        .add_function(get_player_name)
        .add_function(get_steamid)
        .add_function(get_velocity)
        .add_function(set_velocity)
        .add_function(get_acceleration)
        .add_function(set_acceleration)
        .add_function(get_level_state)
        .add_function(wait_for_new_game)
        .add_function(draw_line)
        .add_function(draw_text)
        .add_function(project)
        .add_function(get_text_size)
        .add_function(spawn_pawn)
        .add_function(destroy_pawn)
        .add_function(move_pawn)
        .add_function(set_pawn_velocity)
        .add_function(pawn_location)
        .add_function(connect_to_server)
        .add_function(disconnect_from_server)
        .add_function(join_multiplayer_room)
        .add_function(move_on_server)
        .add_function(press_platform_on_server)
        .add_function(press_button_on_server)
        .add_function(new_game_pressed)
        .add_function(get_level)
        .add_function(set_level)
        .add_function(set_start_seconds)
        .add_function(set_start_partial)
        .add_function(set_end_seconds)
        .add_function(set_end_partial)
        .add_function(is_windows)
        .add_function(is_linux)
        .add_function(get_clipboard)
        .add_function(set_clipboard)
        .add_external_type(Location)
        .add_external_type(Rotation)
        .add_external_type(Velocity)
        .add_external_type(Acceleration)
        .add_external_type(Vector)
        .add_external_type(TextSize)
        .add_external_type(Line)
        .add_external_type(Color)
        .add_external_type(DrawText)
        .add_external_type(LevelState)
        .add_external_type(Server)
        .add_external_type(Step)
        .add_external_type(Disconnected)
        .add_external_type(RecordFrame)
        .add_external_type(InputEvent)
        .add_required_rebo_function(on_key_down)
        .add_required_rebo_function(on_key_up)
        .add_required_rebo_function(on_mouse_move)
        .add_required_rebo_function(draw_hud)
        .add_required_rebo_function(player_joined_multiplayer_room)
        .add_required_rebo_function(player_left_multiplayer_room)
        .add_required_rebo_function(player_moved)
        .add_required_rebo_function(platform_pressed)
        .add_required_rebo_function(button_pressed)
        .add_required_rebo_function(player_pressed_new_game)
        .add_required_rebo_function(start_new_game_at)
        .add_required_rebo_function(disconnected)
        .add_required_rebo_function(on_level_state_change)
    ;
    if let Some(working_dir) = &STATE.lock().unwrap().as_ref().unwrap().working_dir {
        cfg = cfg.include_directory(IncludeDirectoryConfig::Path(PathBuf::from(working_dir)));
    }
    cfg
}

#[derive(rebo::ExternalType)]
pub enum Step {
    Tick,
    NewGame,
    Yield,
}

#[derive(rebo::ExternalType)]
pub enum Disconnected {
    Closed,
    ManualDisconnect,
    SendFailed,
    ReceiveFailed,
    ConnectionRefused,
    LocalTimeOffsetTooManyTries,
    RoomNameTooLong,
}

/// Check internal state and channels to see if we should stop.
fn interrupt_function<'a, 'i>(_vm: &mut VmContext<'a, '_, '_, 'i>) -> Result<(), ExecError<'a, 'i>> {
    loop {
        let result = STATE.lock().unwrap().as_ref().unwrap().stream_rebo_rx.try_recv();
        match result {
            Ok(res) => match res {
                StreamToRebo::WorkingDir(_) => {
                    log!("Got WorkingDir, but can't set it during execution");
                    panic!()
                }
                StreamToRebo::Start(_, _) => {
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
fn step() -> Step {
    step_internal(vm, StepKind::Step)?
}
#[rebo::function(raw("Tas::yield"))]
fn step_yield() -> Step {
    step_internal(vm, StepKind::Yield)?
}
enum StepKind {
    Step,
    Yield,
}
fn step_internal<'a, 'i>(vm: &mut VmContext<'a, '_, '_, 'i>, step_kind: StepKind) -> Result<Step, ExecError<'a, 'i>> {
    // get level state before and after we advance the UE frame to see changes created by Refunct itself
    let old_level_state = LevelState::get();

    if let Some(delta) = STATE.lock().unwrap().as_ref().unwrap().delta {
        FApp::set_delta(delta);
    }

    match step_kind {
        StepKind::Step => STATE.lock().unwrap().as_ref().unwrap().rebo_ue_tx.send(ReboToUe::AdvanceFrame).unwrap(),
        StepKind::Yield => STATE.lock().unwrap().as_ref().unwrap().rebo_ue_tx.send(ReboToUe::PumpMessages).unwrap(),
    }
    loop {
        let mut to_be_returned = None;
        // check UE-thread
        let res = match step_kind {
            StepKind::Step => STATE.lock().unwrap().as_ref().unwrap().ue_rebo_rx.recv().unwrap(),
            StepKind::Yield => STATE.lock().unwrap().as_ref().unwrap().ue_rebo_rx.recv().unwrap(),
        };
        match res {
            UeToRebo::Tick => to_be_returned = Some(Step::Tick),
            UeToRebo::NewGame => to_be_returned = Some(Step::NewGame),
            UeToRebo::PumpedMessages => to_be_returned = Some(Step::Yield),
            UeToRebo::KeyDown(key, char, repeat) => on_key_down(vm, key, char, repeat)?,
            UeToRebo::KeyUp(key, char, repeat) => on_key_up(vm, key, char, repeat)?,
            UeToRebo::MouseMove(x, y) => on_mouse_move(vm, x, y)?,
            UeToRebo::DrawHud => draw_hud(vm)?,
            UeToRebo::AMyCharacterSpawned(_) => unreachable!(),
        }

        // check websocket
        loop {
            let response = match receive_from_server(vm, true) {
                Ok(response) => response,
                Err(ReceiveError::ExecError(err)) => return Err(err),
                Err(ReceiveError::Error) => break,
            };
            match response {
                Response::ServerTime(_) => unreachable!("got Response::ServerTime in step-function"),
                Response::PlayerJoinedRoom(id, name, x, y, z) => player_joined_multiplayer_room(vm, id.id(), name, Location { x, y, z})?,
                Response::PlayerLeftRoom(id) => player_left_multiplayer_room(vm, id.id())?,
                Response::MoveOther(id, x, y, z) => player_moved(vm, id.id(), Location { x, y, z })?,
                Response::PressPlatform(id) => platform_pressed(vm, id)?,
                Response::PressButton(id) => button_pressed(vm, id)?,
                Response::NewGamePressed(id) => player_pressed_new_game(vm, id.id())?,
                Response::StartNewGameAt(timestamp) => {
                    let local_time_offset = STATE.lock().unwrap().as_ref().unwrap().local_time_offset as i64;
                    start_new_game_at(vm, (timestamp as i64 + local_time_offset) as u64)?
                },
                Response::RoomNameTooLong => {
                    disconnected(vm, Disconnected::RoomNameTooLong)?;
                }
            }
        }

        match to_be_returned {
            Some(ret) => {
                // call level-state event function
                let new_level_state = LevelState::get();
                if old_level_state != new_level_state {
                    on_level_state_change(vm, old_level_state.clone(), new_level_state)?;
                }
                return Ok(ret)
            },
            None => (),
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
    fn on_mouse_move(x: i32, y: i32);
    fn draw_hud();
    fn player_joined_multiplayer_room(id: u32, name: String, loc: Location);
    fn player_left_multiplayer_room(id: u32);
    fn player_moved(id: u32, loc: Location);
    fn platform_pressed(id: u8);
    fn button_pressed(id: u8);
    fn player_pressed_new_game(id: u32);
    fn start_new_game_at(timestamp: u64);
    fn disconnected(reason: Disconnected);
    fn on_level_state_change(old: LevelState, new: LevelState);
}

fn config_path() -> PathBuf {
    let cfg_dir = dirs::config_dir().unwrap()
        .join("refunct-tas");
    if !cfg_dir.is_dir() {
        std::fs::create_dir(&cfg_dir).unwrap();
    }
    cfg_dir
}
fn data_path() -> PathBuf {
    let cfg_dir = dirs::data_dir().unwrap()
        .join("refunct-tas");
    if !cfg_dir.is_dir() {
        std::fs::create_dir(&cfg_dir).unwrap();
    }
    cfg_dir
}

#[rebo::function("Tas::load_settings")]
fn load_settings() -> Option<Map<String, String>> {
    let path = config_path().join("settings.json");
    let file = File::open(path).ok()?;
    let map: HashMap<String, String> = serde_json::from_reader(file).unwrap();
    Some(Map::new(map))
}
#[rebo::function("Tas::store_settings")]
fn store_settings(settings: Map<String, String>) {
    let path = config_path().join("settings.json");
    let mut file = File::create(path).unwrap();
    let map = settings.clone_btreemap();
    serde_json::to_writer_pretty(&mut file, &map).unwrap();
    writeln!(file).unwrap();
}

#[derive(rebo::ExternalType, Serialize, Deserialize)]
struct RecordFrame {
    delta: f64,
    events: Vec<InputEvent>,
    location: Location,
    rotation: Rotation,
    velocity: Velocity,
    acceleration: Acceleration,
}
#[derive(rebo::ExternalType, Serialize, Deserialize)]
enum InputEvent {
    KeyPressed(i32),
    KeyReleased(i32),
    MouseMoved(i32, i32),
}
fn recording_path() -> PathBuf {
    let appdata_path = data_path();
    let recording_path = appdata_path.join("recordings/");
    if !recording_path.is_dir() {
        std::fs::create_dir(&recording_path).unwrap();
    }
    recording_path
}
#[rebo::function("Tas::list_recordings")]
fn list_recordings() -> Vec<String> {
    let path = recording_path();
    std::fs::read_dir(path).unwrap().flatten()
        .map(|entry| {
            assert!(entry.file_type().unwrap().is_file());
            entry.file_name().into_string().unwrap()
        }).collect()
}
#[rebo::function("Tas::save_recording")]
fn save_recording(filename: String, recording: Vec<RecordFrame>) {
    let filename = sanitize_filename::sanitize(filename);
    let path = recording_path().join(filename);
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, &recording).unwrap();
}
#[rebo::function("Tas::load_recording")]
fn load_recording(filename: String) -> Vec<RecordFrame> {
    let filename = sanitize_filename::sanitize(filename);
    let path = recording_path().join(filename);
    let content = std::fs::read_to_string(path).unwrap();
    let res = serde_json::from_str(&content).unwrap();
    res
}
#[rebo::function("Tas::remove_recording")]
fn remove_recording(filename: String) -> bool {
    let filename = sanitize_filename::sanitize(filename);
    let path = recording_path().join(filename);
    std::fs::remove_file(path).is_ok()
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
    let delta = STATE.lock().unwrap().as_mut().unwrap().delta;
    match delta {
        Some(delta) => delta,
        None => FApp::delta(),
    }
}
#[rebo::function("Tas::get_delta")]
fn get_delta() -> Option<f64> {
    STATE.lock().unwrap().as_mut().unwrap().delta
}
#[rebo::function("Tas::set_delta")]
fn set_delta(delta: Option<f64>) {
    STATE.lock().unwrap().as_mut().unwrap().delta = delta;
}
#[derive(Debug, Clone, Copy, rebo::ExternalType, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, rebo::ExternalType, Serialize, Deserialize)]
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
#[rebo::function("Tas::get_player_name")]
fn get_player_name() -> String {
    AMyCharacter::get_player().get_player_name()
}
#[rebo::function("Tas::get_steamid")]
fn get_steamid() -> u64 {
    AMyCharacter::get_player().get_steamid()
}
#[derive(Debug, Clone, Copy, rebo::ExternalType, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, rebo::ExternalType, Serialize, Deserialize)]
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
        match step_internal(vm, StepKind::Step)? {
            Step::Tick => continue,
            Step::NewGame => break,
            Step::Yield => unreachable!("step_internal(StepKind::Step) returned Yield"),
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
#[derive(Debug, Clone, Copy, rebo::ExternalType)]
struct TextSize {
    width: f32,
    height: f32,
}
#[rebo::function("Tas::get_text_size")]
fn get_text_size(text: String, scale: f32) -> TextSize {
    let (width, height) = AMyHud::get_text_size(text, scale);
    TextSize { width, height }
}
#[rebo::function("Tas::spawn_pawn")]
fn spawn_pawn(loc: Location, rot: Rotation) -> u32 {
    STATE.lock().unwrap().as_mut().unwrap().rebo_ue_tx.send(ReboToUe::SpawnAMyCharacter(loc.x, loc.y, loc.z, rot.pitch, rot.yaw, rot.roll)).unwrap();
    let spawned = STATE.lock().unwrap().as_mut().unwrap().ue_rebo_rx.recv().unwrap();
    let my_character = match spawned {
        UeToRebo::AMyCharacterSpawned(c) => c,
        _ => unreachable!(),
    };
    let id = STATE.lock().unwrap().as_mut().unwrap().pawn_id;
    STATE.lock().unwrap().as_mut().unwrap().pawn_id += 1;
    STATE.lock().unwrap().as_mut().unwrap().pawns.insert(id, my_character);
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
#[rebo::function("Tas::set_pawn_velocity")]
fn set_pawn_velocity(pawn_id: u32, vel: Velocity) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    let my_character = state.pawns.get_mut(&pawn_id).expect("pawn_id not valid");
    my_character.set_velocity(vel.x, vel.y, vel.z);
}
#[rebo::function("Tas::pawn_location")]
fn pawn_location(pawn_id: u32) -> Location {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    let my_character = state.pawns.get_mut(&pawn_id).expect("pawn_id not valid");
    let (x, y, z) = my_character.location();
    Location { x, y, z }
}
#[derive(rebo::ExternalType)]
enum Server {
    Localhost,
    Remote,
    Testing,
}

#[rebo::function(raw("Tas::connect_to_server"))]
fn connect_to_server(server: Server) {
    let address = match server {
        Server::Localhost => "ws://localhost:8080/ws",
        Server::Remote => "wss://refunct-tas.oberien.de/ws",
        Server::Testing => "wss://refunct-tas-test.oberien.de/ws",
    };
    let client = ClientBuilder::new(address).unwrap().connect(None);
    let client = match client {
        Ok(client) => client,
        Err(e) => {
            log!("couldn't connect to server: {e:?}");
            disconnected(vm, Disconnected::ConnectionRefused)?;
            return Ok(Value::Unit)
        }
    };
    log!("connected to server, figuring out time delta");
    STATE.lock().unwrap().as_mut().unwrap().websocket = Some(client);

    // time delta calculation
    let mut deltas: Vec<i32> = vec![0];
    let delta = loop {
        deltas.sort();
        let median = deltas[deltas.len() / 2];
        let matches = deltas.iter().copied().filter(|&m| (m - median).abs() < 100).count();
        if deltas.len() > 5 && matches as f64 / deltas.len() as f64 > 0.8 {
            break median;
        }
        if deltas.len() > 20 {
            log!("connection too unstable to get local time offset: {deltas:?}");
            disconnected(vm, Disconnected::LocalTimeOffsetTooManyTries)?;
            return Ok(Value::Unit);
        }

        let before = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        send_to_server(vm, "timesync", Request::GetServerTime)?;
        let remote_time = match receive_from_server(vm, false) {
            Err(ReceiveError::Error) => return Ok(Value::Unit),
            Err(ReceiveError::ExecError(err)) => return Err(err),
            Ok(Response::ServerTime(time)) => time,
            Ok(response) => unreachable!("got non-ServerTime during deltatime calculation: {response:?}"),
        };
        let after = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let local_time = ((before + after) / 2) as u64;
        let delta = local_time as i64 - remote_time as i64;
        deltas.push(delta as i32);
        std::thread::sleep(Duration::from_millis(500));
    };
    let msg = format!("local time offset between us and server: {delta} ms");
    log!("{}", msg);
    STATE.lock().unwrap().as_ref().unwrap().rebo_stream_tx.send(ReboToStream::Print(msg)).unwrap();

    STATE.lock().unwrap().as_mut().unwrap().local_time_offset = delta;
}
#[rebo::function(raw("Tas::disconnect_from_server"))]
fn disconnect_from_server() {
    STATE.lock().unwrap().as_mut().unwrap().websocket.take();
    disconnected(vm, Disconnected::ManualDisconnect)?;
}
fn send_to_server<'a, 'i>(vm: &mut VmContext<'a, '_, '_, 'i>, desc: &str, request: Request) -> Result<(), ExecError<'a, 'i>> {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    if state.websocket.is_none() {
        log!("called {desc} without active websocket session");
        // TODO: error propagation?
        return Ok(());
    }
    let msg = serde_json::to_string(&request).unwrap();
    let msg = Message::text(msg);
    if let Err(e) = state.websocket.as_mut().unwrap().send_message(&msg) {
        log!("error sending {desc} request: {e:?}");
        state.websocket.take();
        disconnected(vm, Disconnected::SendFailed)?;
    }
    Ok(())
}
enum ReceiveError<'a, 'i> {
    ExecError(ExecError<'a, 'i>),
    Error,
}
impl<'a, 'i> From<ExecError<'a, 'i>> for ReceiveError<'a, 'i> {
    fn from(e: ExecError<'a, 'i>) -> Self {
        ReceiveError::ExecError(e)
    }
}
fn receive_from_server<'a, 'i>(vm: &mut VmContext<'a, '_, '_, 'i>, nonblocking: bool) -> Result<Response, ReceiveError<'a, 'i>> {
    if STATE.lock().unwrap().as_ref().unwrap().websocket.is_none() {
        return Err(ReceiveError::Error);
    }
    loop {
        if nonblocking {
            STATE.lock().unwrap().as_mut().unwrap().websocket.as_mut().unwrap().set_nonblocking(true).unwrap();
        }
        let res = STATE.lock().unwrap().as_mut().unwrap().websocket.as_mut().unwrap().recv_message();
        if nonblocking {
            STATE.lock().unwrap().as_mut().unwrap().websocket.as_mut().unwrap().set_nonblocking(false).unwrap();
        }
        return match res {
            Ok(OwnedMessage::Text(text)) => Ok(serde_json::from_str(&text).unwrap()),
            Ok(OwnedMessage::Binary(_) | OwnedMessage::Ping(_) | OwnedMessage::Pong(_)) => continue,
            Err(WebSocketError::IoError(io)) if nonblocking && io.kind() == ErrorKind::WouldBlock => Err(ReceiveError::Error),
            Ok(OwnedMessage::Close(_)) => {
                drop(STATE.lock().unwrap().as_mut().unwrap().websocket.take());
                disconnected(vm, Disconnected::Closed)?;
                Err(ReceiveError::Error)
            },
            Err(_) => {
                drop(STATE.lock().unwrap().as_mut().unwrap().websocket.take());
                disconnected(vm, Disconnected::ReceiveFailed)?;
                Err(ReceiveError::Error)
            }
        };
    }
}
#[rebo::function(raw("Tas::join_multiplayer_room"))]
fn join_multiplayer_room(room: String, name: String, loc: Location) {
    send_to_server(vm, "join room", Request::JoinRoom(room, name, loc.x, loc.y, loc.z))?;
}
#[rebo::function(raw("Tas::move_on_server"))]
fn move_on_server(loc: Location) {
    send_to_server(vm, "move", Request::MoveSelf(loc.x, loc.y, loc.z))?;
}
#[rebo::function(raw("Tas::press_platform_on_server"))]
fn press_platform_on_server(platform_id: u8) {
    send_to_server(vm, "press platform", Request::PressPlatform(platform_id))?;
}
#[rebo::function(raw("Tas::press_button_on_server"))]
fn press_button_on_server(button_id: u8) {
    send_to_server(vm, "press button", Request::PressButton(button_id))?;
}
#[rebo::function(raw("Tas::new_game_pressed"))]
fn new_game_pressed() {
    send_to_server(vm, "new game pressed", Request::NewGamePressed)?;
}
#[rebo::function("Tas::get_level")]
fn get_level() -> i32 {
    LevelState::get_level()
}
#[rebo::function("Tas::set_level")]
fn set_level(level: i32) {
    LevelState::set_level(level);
}
#[rebo::function("Tas::set_start_seconds")]
fn set_start_seconds(startsecs: f32) {
    LevelState::set_start_secs(startsecs);
}
#[rebo::function("Tas::set_start_partial")]
fn set_start_partial(sp: f32) {
    LevelState::set_start_partial(sp);
}
#[rebo::function("Tas::set_end_seconds")]
fn set_end_seconds(es: f32) {
    LevelState::set_end_seconds(es);
}
#[rebo::function("Tas::set_end_partial")]
fn set_end_partial(ep: f32) {
    LevelState::set_end_partial(ep);
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
