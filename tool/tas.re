static mut TAS_STATE = TasState {
    step_frame_mode: false,
    is_f_repeat: false,
    is_recording: false,
    is_replaying: Replaying::Nothing,
    recording: List::new(),
    events: List::new(),
    replay_index: 0,
    replay_keys_pressed: Set::new(),
    recording_start_timestamp: 0,
    recording_end_timestamp: 0,
};

struct TasState {
    step_frame_mode: bool,
    is_f_repeat: bool,
    is_recording: bool,
    is_replaying: Replaying,
    recording: List<RecordFrame>,
    events: List<InputEvent>,
    replay_index: int,
    replay_keys_pressed: Set<int>,
    recording_start_timestamp: int,
    recording_end_timestamp: int,
}
enum Replaying {
    Nothing,
    Inputs,
    Positions,
    PositionsAndInputs,
}

fn tas_save_recording(name: string) {
    Tas::save_recording(name, TAS_STATE.recording, TAS_STATE.recording_start_timestamp, TAS_STATE.recording_end_timestamp);
}

fn tas_load_recording(name: string) {
    TAS_STATE.recording = Tas::load_recording(name);
}

impl TasState {
    fn stop_replaying(self) {
        TAS_STATE.is_replaying = Replaying::Nothing;
        for code in TAS_STATE.replay_keys_pressed.values() {
            Tas::key_up(code, code, false);
        }
        TAS_STATE.replay_keys_pressed.clear();
    }

    fn replay_current_positions(self) {
        let frame = TAS_STATE.recording.get(TAS_STATE.replay_index).unwrap();
        Tas::set_location(frame.location);
        Tas::set_rotation(frame.rotation);
        Tas::set_velocity(frame.velocity);
        Tas::set_acceleration(frame.acceleration);
    }

    fn replay_current_events(self) {
        let frame = TAS_STATE.recording.get(TAS_STATE.replay_index).unwrap();
        for event in frame.events {
            match event {
                InputEvent::KeyPressed(code) => {
                    TAS_STATE.replay_keys_pressed.insert(code);
                    Tas::key_down(code, code, false);
                },
                InputEvent::KeyReleased(code) => {
                    TAS_STATE.replay_keys_pressed.remove(code);
                    Tas::key_up(code, code, false);
                },
                InputEvent::MouseMoved(x, y) => {
                    Tas::move_mouse(x, y);
                },
            }
        }
    }
}

static mut TAS_COMPONENT = Component {
    id: TAS_COMPONENT_ID,
    conflicts_with: List::of(MULTIPLAYER_COMPONENT_ID, TAS_COMPONENT_ID),
    draw_hud_text: fn(text: string) -> string {
        let text = f"{text}\nTAS: REQUIRES 60 FPS";
        let text = f"{text}\n     <t> toggle frame-step mode, <f> advance one frame";
        let text = f"{text}\n     <r> to record/stop, <g> to replay inputs, <h> to replay position, <j> to replay positions + inputs";
        let mut text = f"{text}\n     Step-Frame: {TAS_STATE.step_frame_mode}    Recording: {TAS_STATE.is_recording}    Replay {TAS_STATE.is_replaying}: {TAS_STATE.replay_index}/{TAS_STATE.recording.len()}";

        if TAS_STATE.is_replaying == Replaying::Inputs || TAS_STATE.is_replaying == Replaying::PositionsAndInputs {
            text = f"{text}\n\n";
            for key in TAS_STATE.replay_keys_pressed.values() {
                let key_string = if KEY_A.to_small() <= key && key <= KEY_Z.to_small() {
                    string::from_char(key)
                } else if key == KEY_LEFT_SHIFT.to_small() {
                    "SHIFT"
                } else if key == KEY_ESCAPE.to_small() {
                    "ESC"
                } else if key == KEY_SPACE.to_small() {
                    "SPACE"
                } else {
                    "?"
                };
                text = f"{text} {key_string}";
            }
            text = f"{text}\n";
        }

        text
    },
    draw_hud_always: fn() {},
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::Some(1./60.),
    on_tick: fn() {
        if TAS_STATE.step_frame_mode && !TAS_STATE.is_f_repeat {
            TAS_COMPONENT.tick_mode = TickMode::Yield;
        }

        // recording
        if TAS_STATE.is_recording {
            TAS_STATE.recording.push(RecordFrame {
                delta: Tas::get_last_frame_delta(),
                events: TAS_STATE.events,
                location: Tas::get_location(),
                rotation: Tas::get_rotation(),
                velocity: Tas::get_velocity(),
                acceleration: Tas::get_acceleration(),
            });
        }
        TAS_STATE.events = List::new();

        // replay
        if TAS_STATE.replay_index >= TAS_STATE.recording.len() && TAS_STATE.is_replaying != Replaying::Nothing {
            TAS_STATE.stop_replaying();
        }
        match TAS_STATE.is_replaying {
            Replaying::Nothing => (),
            Replaying::Inputs => {
                let frame = TAS_STATE.recording.get(TAS_STATE.replay_index).unwrap();
                if TAS_STATE.replay_index == 0 {
                    TAS_STATE.replay_current_positions();
                }
                TAS_STATE.replay_current_events();
                TAS_STATE.replay_index += 1;
            },
            Replaying::Positions => {
                let frame = TAS_STATE.recording.get(TAS_STATE.replay_index).unwrap();
                TAS_STATE.replay_current_positions();
                TAS_STATE.replay_index += 1;
            },
            Replaying::PositionsAndInputs => {
                let frame = TAS_STATE.recording.get(TAS_STATE.replay_index).unwrap();
                TAS_STATE.replay_current_positions();
                TAS_STATE.replay_current_events();
                TAS_STATE.replay_index += 1;
            }
        }
    },
    on_yield: fn() {},
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_cubes_change: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_element_pressed: fn(index: ElementIndex) {},
    on_element_released: fn(index: ElementIndex) {},
    on_key_down: fn(key_code: KeyCode, is_repeat: bool) {
        let key = key_code.to_small();
        if key == KEY_T.to_small() {
            TAS_STATE.step_frame_mode = !TAS_STATE.step_frame_mode;
            if TAS_STATE.step_frame_mode {
                TAS_COMPONENT.tick_mode = TickMode::Yield;
            } else {
                TAS_COMPONENT.tick_mode = TickMode::DontCare;
            }
        } else if key == KEY_R.to_small() {
            TAS_STATE.is_recording = !TAS_STATE.is_recording;
            if TAS_STATE.is_recording {
                TAS_STATE.recording = List::new();
                TAS_STATE.recording_start_timestamp = current_time_millis();
            } else {
                TAS_STATE.recording_end_timestamp = current_time_millis();
            }
        } else if key == KEY_G.to_small() {
            if TAS_STATE.is_replaying == Replaying::Inputs {
                TAS_STATE.stop_replaying();
            } else {
                TAS_STATE.is_replaying = Replaying::Inputs;
                TAS_STATE.replay_index = 0;
            }
        } else if key == KEY_H.to_small() {
            if TAS_STATE.is_replaying == Replaying::Positions {
                TAS_STATE.stop_replaying();
            } else {
                TAS_STATE.is_replaying = Replaying::Positions;
                TAS_STATE.replay_index = 0;
            }
        } else if key == KEY_J.to_small() {
            if TAS_STATE.is_replaying == Replaying::PositionsAndInputs {
                TAS_STATE.stop_replaying();
            } else {
                TAS_STATE.is_replaying = Replaying::PositionsAndInputs;
                TAS_STATE.replay_index = 0;
            }
        } else if key == KEY_F.to_small() {
            TAS_STATE.is_f_repeat = is_repeat;
            TAS_COMPONENT.tick_mode = TickMode::DontCare;
        } else if !is_repeat {
            TAS_STATE.events.push(InputEvent::KeyPressed(key_code.large_value));
        }
    },
    on_key_down_always: fn(key: KeyCode, is_repeat: bool) {},
    on_key_up: fn(key_code: KeyCode) {
        let key = key_code.to_small();

        if key == KEY_F.to_small() {
            TAS_STATE.is_f_repeat = false;
            if TAS_STATE.step_frame_mode {
                TAS_COMPONENT.tick_mode = TickMode::Yield;
            }
        } else if key == KEY_T.to_small() || key == KEY_R.to_small() || key == KEY_G.to_small() || key == KEY_H.to_small() || key == KEY_J.to_small() {
            // pass
        } else {
            TAS_STATE.events.push(InputEvent::KeyReleased(key_code.large_value));
        }
    },
    on_key_up_always: fn(key: KeyCode) {},
    on_mouse_move: fn(x: int, y: int) {
        TAS_STATE.events.push(InputEvent::MouseMoved(x, y));
    },
    on_component_enter: fn() {},
    on_component_exit: fn() {
        TAS_STATE.step_frame_mode = false;
        TAS_COMPONENT.tick_mode = TickMode::DontCare;
    },
    on_resolution_change: fn() {},
    on_menu_open: fn() {},
};
