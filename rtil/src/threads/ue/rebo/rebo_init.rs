use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::ops::Deref;
use std::path::PathBuf;
use std::time::Duration;
use crossbeam_channel::{Sender, TryRecvError};
use clipboard::{ClipboardProvider, ClipboardContext};
use image::Rgba;
use rebo::{ExecError, ReboConfig, Stdlib, VmContext, Output, Value, DisplayValue, IncludeDirectoryConfig, Map};
use itertools::Itertools;
use once_cell::sync::Lazy;
use websocket::{ClientBuilder, Message, OwnedMessage, WebSocketError};
use crate::native::{AMyCharacter, AMyHud, FApp, LevelState, ObjectWrapper, UWorld, UGameplayStatics, UTexture2D, EBlendMode, LEVELS, ActorWrapper, LevelWrapper, KismetSystemLibrary, FSlateApplication, unhook_fslateapplication_onkeydown, hook_fslateapplication_onkeydown, unhook_fslateapplication_onkeyup, hook_fslateapplication_onkeyup, unhook_fslateapplication_onrawmousemove, hook_fslateapplication_onrawmousemove, UMyGameInstance, ue::FVector, character::USceneComponent, UeScope, try_find_element_index, UObject, Level, ObjectIndex, UeObjectWrapperType, AActor};
use protocol::{Request, Response};
use crate::threads::{ReboToStream, StreamToRebo};
use super::STATE;
use serde::{Serialize, Deserialize};
use crate::threads::ue::{Suspend, UeEvent, rebo::YIELDER};
use crate::native::{ElementIndex, ElementType, ue::FRotator, UEngine, TimeOfDay};
use opener;

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
        .add_function(new_version_string)
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
        .add_function(get_movement_mode)
        .add_function(set_movement_mode)
        .add_function(get_max_fly_speed)
        .add_function(set_max_fly_speed)
        .add_function(get_level_state)
        .add_function(restart_game)
        .add_function(wait_for_new_game)
        .add_function(draw_line)
        .add_function(draw_text)
        .add_function(draw_minimap)
        .add_function(set_minimap_alpha)
        .add_function(draw_player_minimap)
        .add_function(player_minimap_size)
        .add_function(minimap_size)
        .add_function(project)
        .add_function(get_viewport_size)
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
        .add_function(trigger_element)
        .add_function(set_start_seconds)
        .add_function(set_start_partial_seconds)
        .add_function(set_end_seconds)
        .add_function(set_end_partial_seconds)
        .add_function(get_accurate_real_time)
        .add_function(is_windows)
        .add_function(is_linux)
        .add_function(get_clipboard)
        .add_function(set_clipboard)
        .add_function(show_hud)
        .add_function(set_all_cluster_speeds)
        .add_function(list_maps)
        .add_function(load_map)
        .add_function(save_map)
        .add_function(remove_map)
        .add_function(current_map)
        .add_function(original_map)
        .add_function(apply_map)
        .add_function(apply_map_cluster_speeds)
        .add_function(get_looked_at_element_index)
        .add_function(get_element_bounds)
        .add_function(enable_collision)
        .add_function(disable_collision)
        .add_function(exit_water)
        .add_function(open_maps_folder)
        .add_function(set_lighting_casts_shadows)
        .add_function(set_sky_light_enabled)
        .add_function(set_time_dilation)
        .add_function(set_gravity)
        .add_function(get_time_of_day)
        .add_function(set_time_of_day)
        .add_function(set_sky_time_speed)
        .add_function(set_sky_light_brightness)
        .add_function(set_sky_light_intensity)
        .add_function(set_stars_brightness)
        .add_function(set_fog_enabled)
        .add_function(set_sun_redness)
        .add_function(set_cloud_redness)
        .add_function(set_reflection_render_scale)
        .add_function(set_cloud_speed)
        .add_function(set_outro_time_dilation)
        .add_function(set_outro_dilated_duration)
        .add_function(set_kill_z)
        .add_function(set_gamma)
        .add_function(set_screen_percentage)
        .add_function(set_reticle_width)
        .add_function(set_reticle_height)
        .add_function(set_reticle_scale)
        .add_external_type(Location)
        .add_external_type(Rotation)
        .add_external_type(Velocity)
        .add_external_type(Acceleration)
        .add_external_type(Vector)
        .add_external_type(TextSize)
        .add_external_type(Line)
        .add_external_type(Color)
        .add_external_type(DrawText)
        .add_external_type(Size)
        .add_external_type(LevelState)
        .add_external_type(Server)
        .add_external_type(Step)
        .add_external_type(Disconnected)
        .add_external_type(RecordFrame)
        .add_external_type(InputEvent)
        .add_external_type(RefunctMap)
        .add_external_type(Cluster)
        .add_external_type(Element)
        .add_external_type(ElementType)
        .add_external_type(ElementIndex)
        .add_external_type(Bounds)
        .add_external_type(TimeOfDay)
        .add_required_rebo_function(element_pressed)
        .add_required_rebo_function(element_released)
        .add_required_rebo_function(on_key_down)
        .add_required_rebo_function(on_key_up)
        .add_required_rebo_function(on_mouse_move)
        .add_required_rebo_function(draw_hud)
        .add_required_rebo_function(player_joined_multiplayer_room)
        .add_required_rebo_function(player_left_multiplayer_room)
        .add_required_rebo_function(player_moved)
        .add_required_rebo_function(press_platform)
        .add_required_rebo_function(press_button)
        .add_required_rebo_function(player_pressed_new_game)
        .add_required_rebo_function(start_new_game_at)
        .add_required_rebo_function(disconnected)
        .add_required_rebo_function(on_level_state_change)
        .add_required_rebo_function(on_resolution_change)
        .add_required_rebo_function(on_menu_open)
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

#[rebo::function(raw("Tas::new_version_string"))]
fn new_version_string() -> Option<String> {
    STATE.lock().unwrap().as_ref().unwrap().new_version_string.clone()
}

#[rebo::function(raw("print"))]
fn print(..: _) {
    let joined = args.as_slice().iter().map(DisplayValue).join(", ");
    log!("{}", joined);
    STATE.lock().unwrap().as_ref().unwrap().rebo_stream_tx.send(ReboToStream::Print(joined)).unwrap();
}

#[rebo::function(raw("Tas::step"))]
fn step() -> Step {
    step_internal(vm, Suspend::Return)?
}
#[rebo::function(raw("Tas::yield"))]
fn step_yield() -> Step {
    step_internal(vm, Suspend::Yield)?
}
fn step_internal<'a, 'i>(vm: &mut VmContext<'a, '_, '_, 'i>, suspend: Suspend) -> Result<Step, ExecError<'a, 'i>> {
    // get level state before and after we advance the UE frame to see changes created by Refunct itself
    let old_level_state = LevelState::get();

    if let Some(delta) = STATE.lock().unwrap().as_ref().unwrap().delta {
        FApp::set_delta(delta);
    }

    loop {
        let mut to_be_returned = None;
        // yield back to UE stack, but not necessarily the UE loop
        let evt = YIELDER.with(|yielder| unsafe { (*yielder.get()).suspend(suspend) });
        match evt {
            UeEvent::Tick => to_be_returned = Some(Step::Tick),
            UeEvent::ElementPressed(index) => element_pressed(vm, index)?,
            UeEvent::ElementReleased(index) => element_released(vm, index)?,
            UeEvent::NothingHappened => to_be_returned = Some(Step::Yield),
            UeEvent::NewGame => to_be_returned = Some(Step::NewGame),
            UeEvent::KeyDown(key, char, repeat) => on_key_down(vm, key, char, repeat)?,
            UeEvent::KeyUp(key, char, repeat) => on_key_up(vm, key, char, repeat)?,
            UeEvent::MouseMove(x, y) => on_mouse_move(vm, x, y)?,
            UeEvent::DrawHud => draw_hud(vm)?,
            UeEvent::ApplyResolutionSettings => on_resolution_change(vm)?,
            UeEvent::AddToScreen => on_menu_open(vm)?,
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
                Response::PlayerJoinedRoom(id, name, red, green, blue, x, y, z, pitch, yaw, roll) => player_joined_multiplayer_room(vm, id.id(), name, Color { red, green, blue, alpha: 1. }, Location { x, y, z}, Rotation { pitch, yaw, roll })?,
                Response::PlayerLeftRoom(id) => player_left_multiplayer_room(vm, id.id())?,
                Response::MoveOther(id, x, y, z, pitch, yaw, roll) => player_moved(vm, id.id(), Location { x, y, z }, Rotation { pitch, yaw, roll })?,
                Response::PressPlatform(id) => press_platform(vm, id)?,
                Response::PressButton(id) => press_button(vm, id)?,
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
    }
}

#[rebo::required_rebo_functions]
extern "rebo" {
    fn element_pressed(index: ElementIndex);
    fn element_released(index: ElementIndex);
    fn on_key_down(key_code: i32, character_code: u32, is_repeat: bool);
    fn on_key_up(key_code: i32, character_code: u32, is_repeat: bool);
    fn on_mouse_move(x: i32, y: i32);
    fn draw_hud();
    fn player_joined_multiplayer_room(id: u32, name: String, col: Color, loc: Location, rot: Rotation);
    fn player_left_multiplayer_room(id: u32);
    fn player_moved(id: u32, loc: Location, rot: Rotation);
    fn press_platform(id: u8);
    fn press_button(id: u8);
    fn player_pressed_new_game(id: u32);
    fn start_new_game_at(timestamp: u64);
    fn disconnected(reason: Disconnected);
    fn on_level_state_change(old: LevelState, new: LevelState);
    fn on_resolution_change();
    fn on_menu_open();
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
    // we don't want to trigger our keyevent handler for emulated presses
    unhook_fslateapplication_onkeydown();
    FSlateApplication::press_key(key_code, character_code, is_repeat);
    hook_fslateapplication_onkeydown();
}
#[rebo::function("Tas::key_up")]
fn key_up(key_code: i32, character_code: u32, is_repeat: bool) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();
    state.pressed_keys.remove(&key_code);
    // we don't want to trigger our keyevent handler for emulated presses
    unhook_fslateapplication_onkeyup();
    FSlateApplication::release_key(key_code, character_code, is_repeat);
    hook_fslateapplication_onkeyup();
}
#[rebo::function("Tas::move_mouse")]
fn move_mouse(x: i32, y: i32) {
    // we don't want to trigger our mouseevent handler for emulated mouse movements
    unhook_fslateapplication_onrawmousemove();
    FSlateApplication::move_mouse(x, y);
    hook_fslateapplication_onrawmousemove();
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
#[rebo::function("Tas::get_movement_mode")]
fn get_movement_mode() -> u8 {
    AMyCharacter::get_player().movement_mode()
}
#[rebo::function("Tas::set_movement_mode")]
fn set_movement_mode(mode: u8) {
    AMyCharacter::get_player().set_movement_mode(mode);
}
#[rebo::function("Tas::get_max_fly_speed")]
fn get_max_fly_speed() -> f32 {
    AMyCharacter::get_player().max_fly_speed()
}
#[rebo::function("Tas::set_max_fly_speed")]
fn set_max_fly_speed(speed: f32) {
    AMyCharacter::get_player().set_max_fly_speed(speed);
}
#[rebo::function("Tas::get_level_state")]
fn get_level_state() -> LevelState {
    LevelState::get()
}
#[rebo::function("Tas::restart_game")]
fn restart_game() {
    UMyGameInstance::restart_game();
}
#[rebo::function(raw("Tas::wait_for_new_game"))]
fn wait_for_new_game() {
    loop {
        match step_internal(vm, Suspend::Return)? {
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
    let color = (line.color.red, line.color.green, line.color.blue, line.color.alpha);
    AMyHud::draw_line(line.startx, line.starty, line.endx, line.endy, color, line.thickness);
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
    let color = (text.color.red, text.color.green, text.color.blue, text.color.alpha);
    AMyHud::draw_text(text.text, color, text.x, text.y, text.scale, text.scale_position);
}
#[rebo::function("Tas::draw_minimap")]
fn draw_minimap(x: f32, y: f32, scale: f32, scale_position: bool) {
    AMyHud::draw_texture_simple(STATE.lock().unwrap().as_ref().unwrap().minimap_texture.as_ref().unwrap(), x, y, scale, scale_position);
}
#[rebo::function("Tas::set_minimap_alpha")]
fn set_minimap_alpha(alpha: f32) {
    let mut lock = STATE.lock().unwrap();
    let state = lock.as_mut().unwrap();
    let mut image = state.minimap_image.clone();
    for pixel in image.pixels_mut() {
        pixel.0[3] = (255.0 * alpha).round() as u8;
    }
    state.minimap_texture.as_mut().unwrap().set_image(&image);
}
#[derive(Debug, Clone, Copy, rebo::ExternalType)]
struct Size {
    width: i32,
    height: i32,
}
#[rebo::function("Tas::minimap_size")]
fn minimap_size() -> Size {
    let lock = STATE.lock().unwrap();
    let minimap = lock.as_ref().unwrap().minimap_texture.as_ref().unwrap();
    Size {
        width: minimap.width(),
        height: minimap.height(),
    }
}
#[rebo::function("Tas::draw_player_minimap")]
fn draw_player_minimap(x: f32, y: f32, width: f32, height: f32, rotation_degrees: f32, color: Color) {
    let mut lock = STATE.lock().unwrap();
    let state = lock.as_mut().unwrap();
    let color = Rgba([
        (255.0 * color.red).round() as u8,
        (255.0 * color.green).round() as u8,
        (255.0 * color.blue).round() as u8,
        (255.0 * color.alpha).round() as u8,
    ]);
    let texture = &mut state.player_minimap_textures.entry(color)
        .or_insert_with(|| {
            let mut image = state.player_minimap_image.clone();
            for pixel in image.pixels_mut() {
                if *pixel == Rgba([0, 0, 0, 255]) {
                    *pixel = color;
                } else if pixel.0[3] != 0 {
                    pixel.0[3] = color.0[3];
                }
            }
            UTexture2D::create(&image)
        });
    AMyHud::draw_texture(
        texture, x, y, width, height, 0., 0., 1., 1.,
        (1., 1., 1.), EBlendMode::Translucent, 1., false,
        rotation_degrees, 0.5, 0.5
    );
}
#[rebo::function("Tas::player_minimap_size")]
fn player_minimap_size() -> Size {
    let lock = STATE.lock().unwrap();
    let image = &lock.as_ref().unwrap().player_minimap_image;
    Size {
        width: image.width().try_into().unwrap(),
        height: image.height().try_into().unwrap(),
    }
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
#[rebo::function("Tas::get_viewport_size")]
fn get_viewport_size() -> Size {
    let (width, height) = AMyCharacter::get_player().get_viewport_size();
    Size { width, height }
}
#[rebo::function("Tas::spawn_pawn")]
fn spawn_pawn(loc: Location, rot: Rotation) -> u32 {
    let my_character = UWorld::spawn_amycharacter(loc.x, loc.y, loc.z, rot.pitch, rot.yaw, rot.roll);
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
fn join_multiplayer_room(room: String, name: String, col: Color, loc: Location, rot: Rotation) {
    send_to_server(vm, "join room", Request::JoinRoom(room, name, col.red, col.green, col.blue, loc.x, loc.y, loc.z, rot.pitch, rot.yaw, rot.roll))?;
}
#[rebo::function(raw("Tas::move_on_server"))]
fn move_on_server(loc: Location, rot: Rotation) {
    send_to_server(vm, "move", Request::MoveSelf(loc.x, loc.y, loc.z, rot.pitch, rot.yaw, rot.roll))?;
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
#[rebo::function("Tas::trigger_element")]
fn trigger_element(index: ElementIndex) {
    fn add_remove_based_character(actor: &ActorWrapper<'_>) {
        crate::native::unhook_aliftbase_addbasedcharacter();
        crate::native::unhook_aliftbase_removebasedcharacter();
        let add_based_character = actor.class().find_function("AddBasedCharacter").unwrap();
        let remove_based_character = actor.class().find_function("RemoveBasedCharacter").unwrap();
        let args = add_based_character.create_argument_struct();
        let character = unsafe { ObjectWrapper::new(AMyCharacter::get_player().as_ptr() as *mut UObject) };
        args.get_field("BasedCharacter").set_object(&character);
        unsafe { add_based_character.call(actor.as_ptr(), &args); }
        unsafe { remove_based_character.call(actor.as_ptr(), &args); }
        crate::native::hook_aliftbase_removebasedcharacter();
        crate::native::hook_aliftbase_addbasedcharacter();
    }
    UeScope::with(|scope| {
        let levels = LEVELS.lock().unwrap();
        match index.element_type {
            ElementType::Platform => add_remove_based_character(&*scope.get(levels[index.cluster_index].platforms[index.element_index])),
            ElementType::Cube => (),
            ElementType::Button => add_remove_based_character(&*scope.get(levels[index.cluster_index].buttons[index.element_index])),
            ElementType::Lift => add_remove_based_character(&*scope.get(levels[index.cluster_index].lifts[index.element_index])),
            ElementType::Pipe => (),
            ElementType::Springpad => (),
        }
    })
}
#[rebo::function("Tas::set_start_seconds")]
fn set_start_seconds(start_seconds: i32) {
    LevelState::set_start_seconds(start_seconds);
}
#[rebo::function("Tas::set_start_partial_seconds")]
fn set_start_partial_seconds(start_partial_seconds: f32) {
    LevelState::set_start_partial_seconds(start_partial_seconds);
}
#[rebo::function("Tas::set_end_seconds")]
fn set_end_seconds(end_seconds: i32) {
    LevelState::set_end_seconds(end_seconds);
}
#[rebo::function("Tas::set_end_partial_seconds")]
fn set_end_partial_seconds(end_partial_seconds: f32) {
    LevelState::set_end_partial_seconds(end_partial_seconds);
}
#[rebo::function("Tas::get_accurate_real_time")]
fn get_accurate_real_time() -> f64 {
    return UGameplayStatics::get_accurate_real_time();
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
#[rebo::function("Tas::show_hud")]
fn show_hud() {
    AMyHud::show_hud();
}

#[rebo::function("Tas::set_all_cluster_speeds")]
fn set_all_cluster_speeds(speed: f32) {
    // initialize before we change anything
    let _ = &*ORIGINAL_MAP;
    UeScope::with(|scope| {
        for level in &*LEVELS.lock().unwrap() {
            scope.get(level.level).set_speed(speed);
        }
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RefunctMapV0 {
    clusters: Vec<ClusterV0>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClusterV0 {
    platforms: Vec<ElementV0>,
    cubes: Vec<ElementV0>,
    buttons: Vec<ElementV0>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ElementV0 {
    x: f32,
    y: f32,
    z: f32,
    pitch: f32,
    yaw: f32,
    roll: f32,
    xscale: f32,
    yscale: f32,
    zscale: f32,
}

// Changes since the last released map version:
// - Cluster 25 pipe -> Cluster 24 pipe
// - Cluster 25 springpad -> Cluster 24 springpad
// - Cluster 24 springpad -> Cluster 25 springpad
// - Cluster 26 springpad -> Cluster 25 springpad

fn migrate_v0_to_v1(map: RefunctMapV0) -> RefunctMap {
    fn migrate_element(orig: &RefunctMap, e: ElementV0, index: ElementIndex) -> Element {
        let orig = get_indexed_element(orig, index);
        Element {
            x: e.x,
            y: e.y,
            z: e.z,
            pitch: e.pitch,
            yaw: e.yaw,
            roll: e.roll,
            sizex: e.xscale * orig.sizex,
            sizey: e.yscale * orig.sizey,
            sizez: e.zscale * orig.sizez,
        }
    }
    let orig = &*ORIGINAL_MAP;
    RefunctMap {
        version: 1,
        clusters: map.clusters.into_iter().enumerate().map(|(cluster_index, cluster)| Cluster {
            z: orig.clusters[cluster_index].z,
            rise_speed: orig.clusters[cluster_index].rise_speed,
            platforms: cluster.platforms.into_iter().enumerate().map(|(element_index, e)| migrate_element(&orig, e, ElementIndex { cluster_index, element_type: ElementType::Platform, element_index })).collect(),
            cubes: cluster.cubes.into_iter().enumerate().map(| (element_index, e)| migrate_element(&orig, e, ElementIndex { cluster_index, element_type: ElementType::Cube, element_index })).collect(),
            buttons: cluster.buttons.into_iter().enumerate().map(| (element_index, e)| migrate_element(&orig, e, ElementIndex { cluster_index, element_type: ElementType::Button, element_index })).collect(),
            lifts: orig.clusters[cluster_index].lifts.clone(),
            pipes: orig.clusters[cluster_index].pipes.clone(),
            springpads: orig.clusters[cluster_index].springpads.clone(),
        }).collect(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, rebo::ExternalType)]
pub struct RefunctMap {
    version: u32,
    clusters: Vec<Cluster>,
}
#[derive(Debug, Clone, Serialize, Deserialize, rebo::ExternalType)]
struct Cluster {
    z: f32,
    rise_speed: f32,
    platforms: Vec<Element>,
    cubes: Vec<Element>,
    buttons: Vec<Element>,
    lifts: Vec<Element>,
    pipes: Vec<Element>,
    springpads: Vec<Element>,
}
#[derive(Debug, Clone, Serialize, Deserialize, rebo::ExternalType)]
struct Element {
    x: f32,
    y: f32,
    z: f32,
    pitch: f32,
    yaw: f32,
    roll: f32,
    sizex: f32,
    sizey: f32,
    sizez: f32,
}

fn map_path() -> PathBuf {
    let appdata_path = data_path();
    let map_path = appdata_path.join("maps/");
    if !map_path.is_dir() {
        std::fs::create_dir(&map_path).unwrap();
    }
    map_path
}
#[rebo::function("Tas::list_maps")]
fn list_maps() -> Vec<String> {
    let path = map_path();
    std::fs::read_dir(path).unwrap().flatten()
        .map(|entry| {
            assert!(entry.file_type().unwrap().is_file());
            entry.file_name().into_string().unwrap()
        }).collect()
}
#[rebo::function("Tas::load_map")]
fn load_map(filename: String) -> RefunctMap {
    #[derive(Deserialize)]
    struct Version {
        #[serde(default)]
        version: u32,
    }
    let filename = sanitize_filename::sanitize(filename);
    let path = map_path().join(filename);
    let content = std::fs::read_to_string(path).unwrap();
    let version: Version = serde_json::from_str(&content).unwrap();
    let map = match version.version {
        0 => {
            let map = serde_json::from_str(&content).unwrap();
            migrate_v0_to_v1(map)
        }
        1 => serde_json::from_str(&content).unwrap(),
        version => panic!("the map lives in the future (unknown map version {version})"),
    };
    map
}
#[rebo::function("Tas::save_map")]
fn save_map(filename: String, map: RefunctMap) {
    let filename = sanitize_filename::sanitize(filename);
    let path = map_path().join(filename);
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, &map).unwrap();
}
#[rebo::function("Tas::remove_map")]
fn remove_map(filename: String) -> bool {
    let filename = sanitize_filename::sanitize(filename);
    let path = map_path().join(filename);
    std::fs::remove_file(path).is_ok()
}

fn get_current_map(original: bool) -> RefunctMap {
    UeScope::with(|scope| {
        fn actor_list_to_element_list<'a, T, F>(scope: &'a UeScope, level: &LevelWrapper<'a>, list: &[ObjectIndex<T>], element_type: ElementType, get_orig_size: F) -> Vec<Element>
        where
            T: UeObjectWrapperType,
            T::UeObjectWrapper<'a>: Deref<Target = ActorWrapper<'a>>,
            F: Fn(&ActorWrapper<'a>, ElementIndex) -> (f32, f32, f32),
        {
            list.iter().enumerate().map(|(element_index, actor)| {
                let actor = scope.get(actor);
                let actor = actor.deref();
                let index = ElementIndex { cluster_index: level.level_index(), element_type, element_index };
                let (sizex, sizey, sizez) = get_orig_size(actor, index);
                let (_, _, lz) = level.relative_location();
                let (ax, ay, az) = actor.absolute_location();
                let (pitch, yaw, roll) = actor.relative_rotation();
                let (xscale, yscale, zscale) = actor.relative_scale();
                Element { x: ax, y: ay, z: az - lz, pitch, yaw, roll, sizex: sizex / xscale, sizey: sizey / yscale, sizez: sizez / zscale }
            }).collect()
        }
        let get_orig_size: Box<for<'a> fn(&'a ActorWrapper, _) -> _> = if original {
            Box::new(|actor: &ActorWrapper, _index: ElementIndex| {
                let (_, _, _, hx, hy, hz) = actor.get_actor_bounds();
                // ugly hack for rotated platforms to switch sizex and sizey
                // if they are rotated by 90° or -90°
                let (pitch, yaw, roll) = actor.relative_rotation();
                let (hx, hy) = if (89. < yaw && yaw <= 91.) || (-91. < yaw && yaw < -89.) {
                    (hy, hx)
                } else {
                    (hx, hy)
                };
                // it's never rotated exactly 90° / 180° / -180° / -90° but slightly off
                // like -89.99963 or -179.99976 or 179.99976 or even 0.0033555343
                // we correct that by flooring in those cases
                let (hx, hy) = if pitch.fract() != 0. || yaw.fract() != 0. || roll.fract() != 0. {
                    ((hx * 4.).floor() / 4., (hy * 4.).floor() / 4.)
                } else {
                    (hx, hy)
                };
                (hx*2., hy*2., hz*2.)
            })
        } else {
            Box::new(move |_actor: &ActorWrapper, index: ElementIndex| {
                let e = get_indexed_element(&*ORIGINAL_MAP, index);
                (e.sizex, e.sizey, e.sizez)
            })
        };
        let levels = LEVELS.lock().unwrap();
        let clusters: Vec<Cluster> = levels.iter()
            .map(|level| {
                let level_wrapper = scope.get(level.level);
                Cluster {
                    z: level_wrapper.source_location().2,
                    rise_speed: level_wrapper.speed(),
                    platforms: actor_list_to_element_list(scope, &level_wrapper, &level.platforms, ElementType::Platform, &get_orig_size),
                    cubes: actor_list_to_element_list(scope, &level_wrapper, &level.cubes, ElementType::Cube, &get_orig_size),
                    buttons: actor_list_to_element_list(scope, &level_wrapper, &level.buttons, ElementType::Button, &get_orig_size),
                    lifts: actor_list_to_element_list(scope, &level_wrapper, &level.lifts, ElementType::Lift, &get_orig_size),
                    pipes: actor_list_to_element_list(scope, &level_wrapper, &level.pipes, ElementType::Pipe, &get_orig_size),
                    springpads: actor_list_to_element_list(scope, &level_wrapper, &level.springpads, ElementType::Springpad, &get_orig_size),
                }
            }).collect();
        RefunctMap { version: 1, clusters }
    })
}

#[rebo::function("Tas::current_map")]
fn current_map() -> RefunctMap {
    // initialize original map
    let _ = &*ORIGINAL_MAP;
    get_current_map(false)
}
pub static ORIGINAL_MAP: Lazy<RefunctMap> = Lazy::new(|| get_current_map(true));
#[rebo::function("Tas::original_map")]
fn original_map() -> RefunctMap {
    ORIGINAL_MAP.clone()
}
pub fn apply_map_internal(map: &RefunctMap) {
    // initialize before we change anything
    let _ = &*ORIGINAL_MAP;

    fn set_element(scope: &UeScope, levels: &[Level], target_map: &RefunctMap, index: ElementIndex) {
        let level = scope.get(levels[index.cluster_index].level);
        let actor = get_indexed_actor(scope, levels, index);
        let target = get_indexed_element(target_map, index);
        let orig = get_indexed_element(&*ORIGINAL_MAP, index);
        let (_, _, rz) = level.relative_location();
        let (rpitch, ryaw, rroll) = level.relative_rotation();
        let target_location = FVector { x: target.x, y: target.y, z: target.z + rz };
        let target_rotation = FRotator { pitch: target.pitch + rpitch, yaw: target.yaw + ryaw, roll: target.roll + rroll };
        let target_scale = FVector { x: target.sizex / orig.sizex, y: target.sizey / orig.sizey, z: target.sizez / orig.sizez };
        USceneComponent::set_world_location_and_rotation(target_location, target_rotation, &actor);
        USceneComponent::set_world_scale(target_scale, &actor);
    }

    UeScope::with(|scope| {
        let levels = LEVELS.lock().unwrap();
        assert_eq!(map.clusters.len(), levels.len());
        for (cluster_index, (level, cluster)) in levels.iter().zip(&map.clusters).enumerate() {
            let level_wrapper = scope.get(level.level);
            let (rx, ry, _) = level_wrapper.source_location();
            level_wrapper.set_source_location(rx, ry, cluster.z);
            level_wrapper.set_speed(cluster.rise_speed);
            level.platforms.iter().zip(&cluster.platforms).enumerate().map(|i| (i.0, ElementType::Platform))
                .chain(level.cubes.iter().zip(&cluster.cubes).enumerate().map(|i| (i.0, ElementType::Cube)))
                .chain(level.buttons.iter().zip(&cluster.buttons).enumerate().map(|i| (i.0, ElementType::Button)))
                .chain(level.lifts.iter().zip(&cluster.lifts).enumerate().map(|i| (i.0, ElementType::Lift)))
                .chain(level.pipes.iter().zip(&cluster.pipes).enumerate().map(|i| (i.0, ElementType::Pipe)))
                .chain(level.springpads.iter().zip(&cluster.springpads).enumerate().map(|i| (i.0, ElementType::Springpad)))
                .for_each(|(element_index, element_type)| {
                    let index = ElementIndex { cluster_index, element_type, element_index };
                    set_element(scope, &levels, &map, index);
                });
        }
    })
}
#[rebo::function("Tas::apply_map")]
fn apply_map(map: RefunctMap) {
    apply_map_internal(&map)
}
#[rebo::function("Tas::apply_map_cluster_speeds")]
fn apply_map_cluster_speeds(map: RefunctMap) {
    UeScope::with(|scope| {
        let levels = LEVELS.lock().unwrap();
        assert_eq!(map.clusters.len(), levels.len());
        for (_cluster_index, (level, cluster)) in levels.iter().zip(&map.clusters).enumerate() {
            let level_wrapper = scope.get(level.level);
            level_wrapper.set_speed(cluster.rise_speed);
        }
    })
}
#[rebo::function("Tas::get_looked_at_element_index")]
fn get_looked_at_element_index() -> Option<ElementIndex> {
    let intersected = KismetSystemLibrary::line_trace_single(AMyCharacter::get_player());
    try_find_element_index(intersected as *mut UObject)
}

fn get_indexed_element(map: &RefunctMap, index: ElementIndex) -> Element {
    let level = &map.clusters[index.cluster_index];
    match index.element_type {
        ElementType::Platform => level.platforms[index.element_index].clone(),
        ElementType::Cube => level.cubes[index.element_index].clone(),
        ElementType::Button => level.buttons[index.element_index].clone(),
        ElementType::Lift => level.lifts[index.element_index].clone(),
        ElementType::Pipe => level.pipes[index.element_index].clone(),
        ElementType::Springpad => level.springpads[index.element_index].clone(),
    }
}
fn get_indexed_actor<'a>(scope: &'a UeScope, levels: &[Level], index: ElementIndex) -> ActorWrapper<'a> {
    let level = &levels[index.cluster_index];
    match index.element_type {
        ElementType::Platform => (*scope.get(level.platforms[index.element_index])).clone(),
        ElementType::Cube => (*scope.get(level.cubes[index.element_index])).clone(),
        ElementType::Button => (*scope.get(level.buttons[index.element_index])).clone(),
        ElementType::Lift => (*scope.get(level.lifts[index.element_index])).clone(),
        ElementType::Pipe => (*scope.get(level.pipes[index.element_index])).clone(),
        ElementType::Springpad => (*scope.get(level.springpads[index.element_index])).clone(),
    }
}

#[derive(Debug, Clone, rebo::ExternalType)]
struct Bounds {
    originx: f32,
    originy: f32,
    originz: f32,
    extentx: f32,
    extenty: f32,
    extentz: f32,
}

#[rebo::function("Tas::get_element_bounds")]
fn get_element_bounds(index: ElementIndex) -> Bounds {
    UeScope::with(|scope| {
        let levels = LEVELS.lock().unwrap();
        let actor = get_indexed_actor(scope, &levels, index);
        let (originx, originy, originz, extentx, extenty, extentz) = actor.get_actor_bounds();
        Bounds { originx, originy, originz, extentx, extenty, extentz }
    })
}

#[rebo::function("Tas::enable_collision")]
fn enable_collision() {
    AActor::set_actor_enable_collision(AMyCharacter::get_player().as_ptr() as *const AActor, true);
}

#[rebo::function("Tas::disable_collision")]
fn disable_collision() {
    AActor::set_actor_enable_collision(AMyCharacter::get_player().as_ptr() as *const AActor, false);
}

#[rebo::function("Tas::exit_water")]
fn exit_water() {
    AMyCharacter::exit_water();
}
#[rebo::function("Tas::open_maps_folder")]
fn open_maps_folder() {
    if let Err(err) = opener::open(&map_path()) {
        log!("Error opening maps folder in file manager: {}", err);
    }
}
#[rebo::function("Tas::set_lighting_casts_shadows")]
fn set_lighting_casts_shadows(value: bool) {
    UWorld::set_lighting_casts_shadows(value);
}
#[rebo::function("Tas::set_sky_light_enabled")]
fn set_sky_light_enabled(enabled: bool) {
    UWorld::set_sky_light_enabled(enabled);
}
#[rebo::function("Tas::set_time_dilation")]
fn set_time_dilation(dilation: f32) {
    UWorld::set_time_dilation(dilation);
}
#[rebo::function("Tas::set_gravity")]
fn set_gravity(gravity: f32) {
    UWorld::set_gravity(gravity);
}
#[rebo::function("Tas::get_time_of_day")]
fn get_time_of_day() -> f32 {
    UWorld::get_time_of_day()
}
#[rebo::function("Tas::set_time_of_day")]
fn set_time_of_day(time: f32) {
    UWorld::set_time_of_day(time);
}
#[rebo::function("Tas::set_sky_time_speed")]
fn set_sky_time_speed(speed: f32) {
    UWorld::set_sky_time_speed(speed);
}
#[rebo::function("Tas::set_sky_light_brightness")]
fn set_sky_light_brightness(brightness: f32) {
    UWorld::set_sky_light_brightness(brightness);
}
#[rebo::function("Tas::set_sky_light_intensity")]
fn set_sky_light_intensity(intensity: f32) {
    UWorld::set_sky_light_intensity(intensity);
}
#[rebo::function("Tas::set_stars_brightness")]
fn set_stars_brightness(time_of_day: TimeOfDay, brightness: f32) {
    UWorld::set_stars_brightness(time_of_day, brightness);
}
#[rebo::function("Tas::set_fog_enabled")]
fn set_fog_enabled(enabled: bool) {
    UWorld::set_fog_enabled(enabled);
}
#[rebo::function("Tas::set_sun_redness")]
fn set_sun_redness(redness: f32) {
    UWorld::set_sun_redness(redness);
}
#[rebo::function("Tas::set_cloud_redness")]
fn set_cloud_redness(red: f32) {
    UWorld::set_cloud_redness(red);
}
#[rebo::function("Tas::set_reflection_render_scale")]
fn set_reflection_render_scale(render_scale: i32) {
    UWorld::set_reflection_render_scale(render_scale);
}
#[rebo::function("Tas::set_cloud_speed")]
fn set_cloud_speed(speed: f32) {
    UWorld::set_cloud_speed(speed);
}
#[rebo::function("Tas::set_outro_time_dilation")]
fn set_outro_time_dilation(dilation: f32) {
    UWorld::set_outro_time_dilation(dilation);
}
#[rebo::function("Tas::set_outro_dilated_duration")]
fn set_outro_dilated_duration(duration: f32) {
    UWorld::set_outro_dilated_duration(duration);
}
#[rebo::function("Tas::set_kill_z")]
fn set_kill_z(kill_z: f32) {
    UWorld::set_kill_z(kill_z);
}
#[rebo::function("Tas::set_gamma")]
fn set_gamma(value: f32) {
    UEngine::set_gamma(value);
}
#[rebo::function("Tas::set_screen_percentage")]
fn set_screen_percentage(percentage: f32) {
    UWorld::set_screen_percentage(percentage);
}
#[rebo::function("Tas::set_reticle_width")]
fn set_reticle_width(width: f32) {
    STATE.lock().unwrap().as_mut().unwrap().reticle_w = width;
}
#[rebo::function("Tas::set_reticle_height")]
fn set_reticle_height(height: f32) {
    STATE.lock().unwrap().as_mut().unwrap().reticle_h = height;
}
#[rebo::function("Tas::set_reticle_scale")]
fn set_reticle_scale(scale: f32) {
    STATE.lock().unwrap().as_mut().unwrap().reticle_scale = scale;
}