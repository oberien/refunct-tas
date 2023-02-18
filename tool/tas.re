static mut TAS_STATE = TasState {
    is_f_pressed: false,
    step_frame_mode: false,
    is_recording: false,
    is_replaying: Replaying::Nothing,
    recording: List::new(),
    events: List::new(),
    replay_index: 0,
    replay_keys_pressed: Set::new(),
    start_time: 0.,
    end_time: 0.,
};

struct TasState {
    is_f_pressed: bool,
    step_frame_mode: bool,
    is_recording: bool,
    is_replaying: Replaying,
    recording: List<RecordFrame>,
    events: List<InputEvent>,
    replay_index: int,
    replay_keys_pressed: Set<int>,
    start_time: float,
    end_time: float,

}
enum Replaying {
    Nothing,
    Inputs,
    Positions,
    PositionsAndInputs,
}

fn tas_save_recording(name: string) {
    Tas::save_recording(name, TAS_STATE.recording);
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

static TAS_COMPONENT = Component {
    id: TAS_COMPONENT_ID,
    conflicts_with: List::of(MULTIPLAYER_COMPONENT_ID, TAS_COMPONENT_ID),
    draw_hud: fn(text: string) -> string {
        let text = f"{text}\nTAS: REQUIRES 60 FPS";
        let text = f"{text}\n     t toggle frame-step mode, f advance one frame";
        let text = f"{text}\n     r to record/stop, g to replay inputs, h to replay position, j to replay positions + inputs";
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
    tick_mode: TickMode::Yield,
    requested_delta_time: Option::Some(1./60.),
    on_tick: fn() {
        let mut foo = Tas::get_accurate_real_time();
        let mut baz = Tas::get_start_seconds();
        let mut bar = (baz.to_float() + Tas::get_start_partial_seconds());
        let mut rar = Tas::get_end_seconds();
        let mut xyz = (rar.to_float() + Tas::get_end_partial_seconds());
        print(f"on_tick            (GART): {foo}");
        print(f"on_tick            (GSS ): {bar}");
        print(f"on_tick            (GES ): {xyz}");
        
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
    on_yield: fn() {
        if !TAS_STATE.step_frame_mode || TAS_STATE.is_f_pressed {
            step_frame(TickMode::DontCare);
        }
    },
    on_new_game: fn() {
        let mut foo = Tas::get_accurate_real_time();
        let mut baz = Tas::get_start_seconds();
        let mut bar = (baz.to_float() + Tas::get_start_partial_seconds());
        print(f"on_new_game         (GART): {foo}");
        print(f"on_new_game         (GSS ): {bar}");
    },
    on_level_change: fn(old: int, new: int) {
        if new == 31 {
            let mut foo = Tas::get_accurate_real_time();
            let mut baz = Tas::get_start_seconds();
            let mut bar = (baz.to_float() + Tas::get_start_partial_seconds());
            let mut rar = Tas::get_end_seconds();
            let mut xyz = (rar.to_float() + Tas::get_end_partial_seconds());
            let mut ree = xyz - bar;
            let mut raa = Tas::get_accurate_real_time() - bar;
            print(f"on_level_change     (GART): {foo}");
            print(f"on_level_change     (GSS ): {bar}");
            print(f"on_level_change     (GES ): {xyz}");
            print(f"end time            (time): {ree}");
            print(f"end time 2          (time): {raa}");
        } else {
            let mut foo = Tas::get_accurate_real_time();
            let mut baz = Tas::get_start_seconds();
            let mut bar = (baz.to_float() + Tas::get_start_partial_seconds());
            let mut rar = Tas::get_end_seconds();
            let mut xyz = (rar.to_float() + Tas::get_end_partial_seconds());
            print(f"on_level_change     (GART): {foo}");
            print(f"on_level_change     (GSS ): {bar}");
            print(f"on_level_change     (GES ): {xyz}");
        }
    },
    on_reset: fn(old: int, new: int) {
        let mut foo = Tas::get_accurate_real_time();
        let mut baz = Tas::get_start_seconds();
        let mut bar = (baz.to_float() + Tas::get_start_partial_seconds());
        print(f"on_reset            (GART): {foo}");
        print(f"on_reset            (GSS ): {bar}");
    },
    on_platforms_change: fn(old: int, new: int) {
        let mut foo = Tas::get_accurate_real_time();
        let mut baz = Tas::get_start_seconds();
        let mut bar = (baz.to_float() + Tas::get_start_partial_seconds());
        let mut rar = Tas::get_end_seconds();
        let mut xyz = (rar.to_float() + Tas::get_end_partial_seconds());
        print(f"on_buttons_change   (GART): {foo}");
        print(f"on_buttons_change   (GSS ): {bar}");
        print(f"on_buttons_change   (GES ): {xyz}");
    },
    on_buttons_change: fn(old: int, new: int) {
        let mut foo = Tas::get_accurate_real_time();
        let mut baz = Tas::get_start_seconds();
        let mut bar = (baz.to_float() + Tas::get_start_partial_seconds());
        let mut rar = Tas::get_end_seconds();
        let mut xyz = (rar.to_float() + Tas::get_end_partial_seconds());
        print(f"on_buttons_change   (GART): {foo}");
        print(f"on_buttons_change   (GSS ): {bar}");
        print(f"on_buttons_change   (GES ): {xyz}");
    },
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
            if is_repeat {
                TAS_STATE.is_f_pressed = true;
            } else {
                step_frame(TickMode::DontCare);
            }
        } else if !is_repeat {
            TAS_STATE.events.push(InputEvent::KeyPressed(key_code.large_value));
        }
    },
    on_key_up: fn(key_code: KeyCode) {
        let key = key_code.to_small();

        if key == KEY_F.to_small() {
            TAS_STATE.is_f_pressed = false;
        } else if key == KEY_T.to_small() || key == KEY_R.to_small() || key == KEY_G.to_small() || key == KEY_H.to_small() || key == KEY_J.to_small() {
            // pass
        } else {
            TAS_STATE.events.push(InputEvent::KeyReleased(key_code.large_value));
        }
    },
    on_mouse_move: fn(x: int, y: int) {
        TAS_STATE.events.push(InputEvent::MouseMoved(x, y));
    },
    on_component_exit: fn() {},
};
