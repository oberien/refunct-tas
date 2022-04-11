static mut TAS_STATE = TasState {
    is_f_pressed: false,
    step_frame_mode: false,
    is_recording: false,
    is_replaying: Replaying::Nothing,
    recording: List::new(),
    key_events: List::new(),
    replay_index: 0,
    replay_keys_pressed: Set::new(),
};

struct TasState {
    is_f_pressed: bool,
    step_frame_mode: bool,
    is_recording: bool,
    is_replaying: Replaying,
    recording: List<RecordFrame>,
    key_events: List<KeyEvent>,
    replay_index: int,
    replay_keys_pressed: Set<int>,
}
struct RecordFrame {
    key_events: List<KeyEvent>,
    location: Location,
    rotation: Rotation,
    velocity: Velocity,
    acceleration: Acceleration,
}
enum KeyEvent {
    KeyPressed(int),
    KeyReleased(int),
}
enum Replaying {
    Nothing,
    Inputs,
    Positions,
}

impl TasState {
    fn stop_replaying(self) {
        TAS_STATE.is_replaying = Replaying::Nothing;
        for code in TAS_STATE.replay_keys_pressed.values() {
            Tas::key_up(code, code, false);
        }
        TAS_STATE.replay_keys_pressed.clear();
    }
}

static TAS_COMPONENT = Component {
    draw_hud: fn(text: string) -> string {
        let text = f"{text}\nTAS: REQUIRES 60 FPS";
        let text = f"{text}\n     t toggle frame-step mode, f advance one frame";
        let text = f"{text}\n     r to record/stop, g to replay inputs, h to replay position";
        let text = f"{text}\n     Step-Frame: {TAS_STATE.step_frame_mode}    Recording: {TAS_STATE.is_recording}    Replay {TAS_STATE.is_replaying}: {TAS_STATE.replay_index}/{TAS_STATE.recording.len()}";
        text
    },
    tick_fn: Tas::yield,
    on_tick: fn() {
        // recording
        if TAS_STATE.is_recording {
            TAS_STATE.recording.push(RecordFrame {
                key_events: TAS_STATE.key_events,
                location: Tas::get_location(),
                rotation: Tas::get_rotation(),
                velocity: Tas::get_velocity(),
                acceleration: Tas::get_acceleration(),
            });
        }
        TAS_STATE.key_events = List::new();

        // replay
        if TAS_STATE.replay_index >= TAS_STATE.recording.len() && TAS_STATE.is_replaying != Replaying::Nothing {
            TAS_STATE.stop_replaying();
        }
        match TAS_STATE.is_replaying {
            Replaying::Nothing => (),
            Replaying::Inputs => {
                let frame = TAS_STATE.recording.get(TAS_STATE.replay_index).unwrap();
                if TAS_STATE.replay_index == 0 {
                    Tas::set_location(frame.location);
                    Tas::set_rotation(frame.rotation);
                    Tas::set_velocity(frame.velocity);
                    Tas::set_acceleration(frame.acceleration);
                }
                for event in frame.key_events {
                    match event {
                        KeyEvent::KeyPressed(code) => {
                            TAS_STATE.replay_keys_pressed.insert(code);
                            Tas::key_down(code, code, false);
                        },
                        KeyEvent::KeyReleased(code) => {
                            TAS_STATE.replay_keys_pressed.remove(code);
                            Tas::key_up(code, code, false);
                        },
                    }
                }
                TAS_STATE.replay_index += 1;
            },
            Replaying::Positions => {
                let frame = TAS_STATE.recording.get(TAS_STATE.replay_index).unwrap();
                Tas::set_location(frame.location);
                Tas::set_rotation(frame.rotation);
                Tas::set_velocity(frame.velocity);
                Tas::set_acceleration(frame.acceleration);
                TAS_STATE.replay_index += 1;
            }
        }
    },
    on_yield: fn() {
        if !TAS_STATE.step_frame_mode || TAS_STATE.is_f_pressed {
            step_frame(Option::Some(1./60.), Tas::step);
        }
    },
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_key_down: fn(key_code: KeyCode, is_repeat: bool) {
        let key = key_code.to_small();
        if key == KEY_T.to_small() {
            TAS_STATE.step_frame_mode = !TAS_STATE.step_frame_mode;
        } else if key == KEY_R.to_small() {
            TAS_STATE.is_recording = !TAS_STATE.is_recording;
            if TAS_STATE.is_recording {
                TAS_STATE.recording = List::new();
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
                TAS_STATE.is_replaying = Replaying::Nothing;
            } else {
                TAS_STATE.is_replaying = Replaying::Positions;
                TAS_STATE.replay_index = 0;
            }
        } else if key == KEY_F.to_small() {
            if is_repeat {
                TAS_STATE.is_f_pressed = true;
            } else {
                step_frame(Option::Some(1./60.), Tas::step);
            }
        } else if !is_repeat {
            TAS_STATE.key_events.push(KeyEvent::KeyPressed(key_code.large_value));
        }
    },
    on_key_up: fn(key_code: KeyCode) {
        let key = key_code.to_small();

        if key == KEY_F.to_small() {
            TAS_STATE.is_f_pressed = false;
        } else if key == KEY_T.to_small() || key == KEY_R.to_small() || key == KEY_G.to_small() || key == KEY_H.to_small() {
            // pass
        } else {
            TAS_STATE.key_events.push(KeyEvent::KeyReleased(key_code.large_value));
        }
    },
    on_component_exit: fn() {},
};
