static mut JOIN_ROOM_LABEL = Text { text: "Join/Create Room" };

fn create_multiplayer_menu() -> Ui {
    Ui::new("Multiplayer:", List::of(
        UiElement::Input(Input {
            label: JOIN_ROOM_LABEL,
            input: "",
            onclick: fn(input: string) {
                if input.len_utf8() == 0 {
                    JOIN_ROOM_LABEL.text = "Join/Create Room (Error: empty room name)";
                    return;
                }
                multiplayer_join_room(input);
                JOIN_ROOM_LABEL.text = "Join/Create Room";
                add_component(MULTIPLAYER_COMPONENT);
                leave_ui();
            },
            onchange: fn(input: string) {},
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Disconnect" },
            onclick: fn(label: Text) {
                remove_component(MULTIPLAYER_COMPONENT);
                leave_ui();
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui(); },
        }),
    ))
}


enum Connection {
    Connected,
    Error(string),
    Disconnected,
}
enum ReadyState {
    NotReady,
    NotReadySomeoneElseReady,
    Ready,
    StartingAt(int),
}
struct MultiplayerState {
    connection: Connection,
    current_room: Option<string>,
    players: Map<int, Player>,
    pawns: List<Pawn>,
    pressed_platforms: Set<int>,
    current_platforms: int,
    pressed_buttons: Set<int>,
    current_buttons: int,
    /// custer-id -> timestamp
    risen_clusters: Map<int, int>,
    ready_state: ReadyState,
}
struct Player {
    id: int,
    name: string,
    col: Color,
    loc: Location,
    rot: Rotation,
}
struct Pawn {
    id: int,
    spawned_at: int,
}
impl Pawn {
    fn spawn(loc: Location) -> Pawn {
        let rot = Rotation { pitch: 0., yaw: 0., roll: 0. };
        let id = Tas::spawn_pawn(loc, rot);
        Tas::set_pawn_velocity(id, Velocity { x: 0., y: 0., z: -1000000. });
        Pawn {
            id: id,
            spawned_at: current_time_millis(),
        }
    }

    fn destroy(self) {
        Tas::destroy_pawn(self.id);
    }
}

static mut MULTIPLAYER_STATE = MultiplayerState {
    connection: Connection::Disconnected,
    current_room: Option::None,
    players: Map::new(),
    pawns: List::new(),
    pressed_platforms: Set::new(),
    current_platforms: 0,
    pressed_buttons: Set::new(),
    current_buttons: 0,
    risen_clusters: Map::new(),
    ready_state: ReadyState::NotReady,
};

static mut MULTIPLAYER_COMPONENT = Component {
    id: MULTIPLAYER_COMPONENT_ID,
    conflicts_with: List::of(MULTIPLAYER_COMPONENT_ID, NEW_GAME_100_PERCENT_COMPONENT_ID, NEW_GAME_ALL_BUTTONS_COMPONENT_ID, NEW_GAME_NGG_COMPONENT_ID, PRACTICE_COMPONENT_ID, RANDOMIZER_COMPONENT_ID, TAS_COMPONENT_ID, WINDSCREEN_WIPERS_COMPONENT_ID),
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: update_players,
    on_yield: fn() {},
    draw_hud_text: fn(text: string) -> string {
        match MULTIPLAYER_STATE.connection {
            Connection::Disconnected => return text,
            Connection::Error(err_msg) => return f"{text}\nMultiplayer error: {err_msg}",
            Connection::Connected => {
                let room = match MULTIPLAYER_STATE.current_room {
                    Option::None => f"{text}\nMultiplayer connected to server",
                    Option::Some(room) => f"{text}\nMultiplayer Room: {room}",
                };
                let message = match MULTIPLAYER_STATE.ready_state {
                    ReadyState::NotReady => "You are not yet ready. Press <R> to ready up.",
                    ReadyState::NotReadySomeoneElseReady => "Other players are waiting for you to ready up. Press <R> to ready up.",
                    ReadyState::Ready => "You are ready. Please wait for others to ready up.",
                    ReadyState::StartingAt(_ts) => "Starting New Game...",
                };
                f"{room}\n\n{message}\n"
            }
        }
    },
    draw_hud_always: fn() {
        match MULTIPLAYER_STATE.connection {
            Connection::Disconnected => (),
            Connection::Error(err_msg) => (),
            Connection::Connected => {
                for player_id in MULTIPLAYER_STATE.players.keys() {
                    let player = MULTIPLAYER_STATE.players.get(player_id).unwrap();
                    draw_player(player.name, player.loc, player.col);
                    minimap_draw_player(player.loc, player.rot, player.col);
                }
//                for pawn in MULTIPLAYER_STATE.pawns {
//                    draw_player("pawn", Tas::pawn_location(pawn.id));
//                }
            }
        }
        match MULTIPLAYER_STATE.ready_state {
            ReadyState::StartingAt(ts) => {
                let time = current_time_millis();
                if time >= ts {
                    print(f"starting synchronized new game at {time} (expected {ts})");
                    Tas::restart_game();
                    MULTIPLAYER_STATE.ready_state = ReadyState::NotReady;
                }
                let viewport = Tas::get_viewport_size();
                let new_time = f"{(ts - time) / 1000:1}.{(ts - time) % 1000:03}";
                let msg = "Starting new game in...";
                let text_size = Tas::get_text_size(new_time, 1.);
                Tas::draw_text(DrawText {
                    text: new_time,
                    color: Color { red: 0., green: 1., blue: 1., alpha: 1. },
                    x: (viewport.width.to_float() / 2.) - (text_size.width / 2.),
                    y: ((viewport.height.to_float() / 2.) - (text_size.height / 2.) + 50.),
                    scale: 1.,
                    scale_position: false,
                });
                let text_size = Tas::get_text_size(msg, 1.);
                Tas::draw_text(DrawText {
                    text: msg,
                    color: Color { red: 0., green: 1., blue: 1., alpha: 1. },
                    x: viewport.width.to_float() / 2. - (text_size.width / 2.),
                    y: (viewport.height.to_float() / 2. - (text_size.height / 2.) - 50.),
                    scale: 1.,
                    scale_position: false,
                });
            },
            _ => return,
        }
    },
    on_new_game: fn() {
        MULTIPLAYER_STATE.current_platforms = 0;
        MULTIPLAYER_STATE.current_buttons = 0;
        MULTIPLAYER_STATE.pressed_platforms = Set::new();
        MULTIPLAYER_STATE.pressed_buttons = Set::new();
        MULTIPLAYER_STATE.risen_clusters = Map::new();
        MULTIPLAYER_STATE.risen_clusters.insert(0, 0);
        for pawn in MULTIPLAYER_STATE.pawns {
            pawn.destroy();
        }
        MULTIPLAYER_STATE.pawns = List::new();
    },
    on_level_change: fn(old: int, new: int) {
        if old > new {
            assert(new == 0);
            return;
        }
        MULTIPLAYER_STATE.risen_clusters.insert(new, current_time_millis());
    },
    on_buttons_change: fn(old: int, new: int) {},
    on_cubes_change: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_element_pressed: fn(index: ElementIndex) {},
    on_element_released: fn(index: ElementIndex) {},
//    on_platforms_change: fn(old: int, new: int) {
//        if old > new {
//            assert(new == 0);
//            return;
//        }
//        if MULTIPLAYER_STATE.current_platforms >= new {
//            return;
//        }
//        MULTIPLAYER_STATE.current_platforms = new;
//
//        let player = Tas::get_location();
//        let mut min_distance = 999999.;
//        let mut platform_num = 0;
//        let mut i = -1;
//        for platform in PLATFORMS {
//            i += 1;
//            if MULTIPLAYER_STATE.pressed_platforms.contains(i) {
//                continue;
//            }
//            let cluster_depth = match cluster_depth(platform.cluster) {
//                Option::Some(depth) => depth,
//                Option::None => continue,
//            };
//
//            let x1 = platform.loc.x - platform.size.x;
//            let x2 = platform.loc.x + platform.size.x;
//            let dx = float::max(x1 - player.x, 0., player.x - x2);
//            let y1 = platform.loc.y - platform.size.y;
//            let y2 = platform.loc.y + platform.size.y;
//            let dy = float::max(y1 - player.y, 0., player.y - y2);
//            let z = platform.loc.z + platform.size.z + 89.15 - cluster_depth;
//            // ignore platforms below the player to prevent invalid platforms during step-ups
//            if z + 5. < player.z {
//                // list of platforms to ignore as they can be triggered from higher up
//                // but don't have any platform you can step-up to from them
//                let excempted_platforms = List::of(
//                    // button platforms
//                    // if you jump on a button, your z is higher than the platform's z but you still activate the platform
//                    0, 9, 15, 26, 32, 38, 43, 44, 57, 63,
//                    68, 70, 84, 85, 90, 96, 98, 101, 105, 109,
//                    108, 123, 130, 141, 150, 153, 155, 168, 183, 178,
//                    180, 193, 211, 201, 217, 243, 247,
//                    // spring platforms: similar
//                    159, 160, 170, 189,
//                );
//                if !excempted_platforms.contains(i) {
//                    continue;
//                }
//            }
//            let dz = z - player.z;
//            let distance = float::sqrt(dx*dx + dy*dy + dz*dz);
//            if distance < min_distance {
//                min_distance = distance;
//                platform_num = i;
//            }
//        }
//        Tas::press_platform_on_server(platform_num);
//        MULTIPLAYER_STATE.pressed_platforms.insert(platform_num);
//    },
//    on_buttons_change: fn(old: int, new: int) {
//        if old > new {
//            assert(new == 0);
//            return;
//        }
//        if MULTIPLAYER_STATE.current_buttons >= new {
//            return;
//        }
//        MULTIPLAYER_STATE.current_buttons = new;
//
//        let player = Tas::get_location();
//        let mut min_distance = 999999.;
//        let mut button_num = 0;
//        let mut i = -1;
//        for button in BUTTONS {
//            i += 1;
//            if MULTIPLAYER_STATE.pressed_buttons.contains(i) {
//                continue;
//            }
//            let cluster_depth = match cluster_depth(button.cluster) {
//                Option::Some(depth) => depth,
//                Option::None => continue,
//            };
//
//            let dx = button.loc.x - player.x;
//            let dy = button.loc.y - player.y;
//            let dz = button.loc.z - player.z - cluster_depth;
//            let distance = float::sqrt(dx*dx + dy*dy + dz*dz);
//            if distance < min_distance {
//                min_distance = distance;
//                button_num = i;
//            }
//        }
//        Tas::press_button_on_server(button_num);
//        MULTIPLAYER_STATE.pressed_buttons.insert(button_num);
//    },
    on_key_down: fn(key: KeyCode, is_repeat: bool) {
        let new_key = key.to_small();
        if new_key == KEY_R.to_small() {
            MULTIPLAYER_STATE.ready_state = ReadyState::Ready;
            Tas::new_game_pressed();
        }
    },
    on_key_down_always: fn(key: KeyCode, is_repeat: bool) {},
    on_key_up: fn(key: KeyCode) {},
    on_key_up_always: fn(key: KeyCode) {},
    on_mouse_move: fn(x: int, y: int) {},
    on_component_enter: fn() {},
    on_component_exit: fn() { multiplayer_disconnect(); },
    on_resolution_change: fn() {},
    on_menu_open: fn() {},
};

fn cluster_depth(cluster: int) -> Option<float> {
    let cluster_rise_start = match MULTIPLAYER_STATE.risen_clusters.get(cluster) {
        Option::Some(ts) => ts,
        Option::None => return Option::None,
    };
    let cluster_rise_time = current_time_millis() - cluster_rise_start;
    let cluster_rise_time = cluster_rise_time.to_float() / 1000.;
    let cluster_depth = float::max(CLUSTER_DEPTHS.get(cluster).unwrap() - 700. * cluster_rise_time, 0.);
    Option::Some(cluster_depth)
}

fn draw_player(name: string, loc: Location, col: Color) {
    let x = loc.x;
    let y = loc.y;
    let z = loc.z - 100.;

    let a = Tas::project(Vector { x: x-50., y: y, z: z });
    let b = Tas::project(Vector { x: x+50., y: y, z: z });
    let c = Tas::project(Vector { x: x-50., y: y, z: z+200. });
    let d = Tas::project(Vector { x: x+50., y: y, z: z+200. });

    let e = Tas::project(Vector { x: x, y: y-50., z: z });
    let f = Tas::project(Vector { x: x, y: y+50., z: z });
    let g = Tas::project(Vector { x: x, y: y-50., z: z+200. });
    let h = Tas::project(Vector { x: x, y: y+50., z: z+200. });

    let top_middle = Tas::project(Vector { x: loc.x, y: loc.y, z: loc.z+100. });

    fn draw_player_line(start: Vector, end: Vector, color: Color) {
        if start.z > 0. && end.z > 0. {
            Tas::draw_line(Line {
                startx: start.x,
                starty: start.y,
                endx: end.x,
                endy: end.y,
                color: color,
                thickness: 3.,
            });
        }
    }

    draw_player_line(a, b, col);
    draw_player_line(b, c, col);
    draw_player_line(c, d, col);
    draw_player_line(d, a, col);

    draw_player_line(e, f, col);
    draw_player_line(f, g, col);
    draw_player_line(g, h, col);
    draw_player_line(h, e, col);
    if top_middle.z > 0. {
        let size = Tas::get_text_size(name, SETTINGS.ui_scale);
        Tas::draw_text(DrawText {
            text: name,
            color: COLOR_BLACK,
            x: top_middle.x - size.width / 2.,
            y: top_middle.y - size.height,
            scale: SETTINGS.ui_scale,
            scale_position: false,
        });
    }
}

fn multiplayer_connect() {
    if MULTIPLAYER_STATE.connection == Connection::Connected {
        multiplayer_disconnect();
    }
    MULTIPLAYER_STATE.connection = Connection::Connected;
    let level_state = Tas::get_level_state();
    MULTIPLAYER_STATE.current_platforms = level_state.platforms;
    MULTIPLAYER_STATE.pressed_platforms = Set::new();
    MULTIPLAYER_STATE.pressed_buttons = Set::new();
    MULTIPLAYER_STATE.current_buttons = level_state.buttons;
    MULTIPLAYER_STATE.risen_clusters = Map::new();
    let mut i = 0;
    while i <= level_state.level {
        MULTIPLAYER_STATE.risen_clusters.insert(i, 0);
        i += 1;
    }
    Tas::connect_to_server(Server::Remote);
}
fn multiplayer_disconnect() {
    if MULTIPLAYER_STATE.connection != Connection::Connected {
        return;
    }
    Tas::disconnect_from_server();
    MULTIPLAYER_STATE.connection = Connection::Disconnected;
    MULTIPLAYER_STATE.current_room = Option::None;
    for player_id in MULTIPLAYER_STATE.players.keys() {
        let player = MULTIPLAYER_STATE.players.remove(player_id).unwrap();
    }
    for pawn in MULTIPLAYER_STATE.pawns {
        pawn.destroy();
    }
    MULTIPLAYER_STATE.pawns.clear();
    MULTIPLAYER_STATE.players = Map::new();
}
fn multiplayer_join_room(room: string) {
    multiplayer_disconnect();
    multiplayer_connect();
    let loc = Tas::get_location();
    let rot = Tas::get_rotation();
    let col = Color {
        red: SETTINGS.player_color_red,
        green: SETTINGS.player_color_green,
        blue: SETTINGS.player_color_blue,
        alpha: 1.,
    };
    Tas::join_multiplayer_room(room, Tas::get_player_name(), col, loc, rot);
    MULTIPLAYER_STATE.current_room = Option::Some(room);
}

fn update_players() {
    static mut LAST_MILLIS = current_time_millis();
    let current_millis = current_time_millis();

    // only update ~30 times per second (capped at FPS as we are in draw_hud)
    if MULTIPLAYER_STATE.connection == Connection::Connected && current_millis - LAST_MILLIS > 33 {
        // update server location
        let loc = Tas::get_location();
        let rot = Tas::get_rotation();
        Tas::move_on_server(loc, rot);
        LAST_MILLIS += 33;
    }

    let mut i = 0;
    let pawns = MULTIPLAYER_STATE.pawns;
    while i < pawns.len() {
        let pawn = pawns.get(i).unwrap();
        if pawn.spawned_at + 10000 < current_millis {
            pawn.destroy();
            pawns.swap_remove(i);
            continue;
        }
        i += 1;
    }
}

fn player_joined_multiplayer_room(id: int, name: string, col: Color, loc: Location, rot: Rotation) {
    print(f"player {id} joined at x={loc.x}, y={loc.y}, z={loc.z}");
    MULTIPLAYER_STATE.players.insert(id, Player {
        id: id,
        name: name,
        col: col,
        loc: loc,
        rot: rot,
    });
}
fn player_left_multiplayer_room(id: int) {
    print(f"player {id} left");
    MULTIPLAYER_STATE.players.remove(id).unwrap();
}
fn player_moved(id: int, loc: Location, rot: Rotation) {
    let mut player = MULTIPLAYER_STATE.players.get(id).unwrap();
    player.loc = loc;
    player.rot = rot;
}
fn press_platform(id: int) {
    let platform = match PLATFORMS.get(id) {
        Option::Some(platform) => platform,
        Option::None => {
            print("Server sent invalid platform number {id}");
            return
        },
    };

    let loc = platform_pawn_spawn_location(platform);
    MULTIPLAYER_STATE.pawns.push(Pawn::spawn(loc));
    if !MULTIPLAYER_STATE.pressed_platforms.contains(id) {
        MULTIPLAYER_STATE.current_platforms += 1;
    }
    MULTIPLAYER_STATE.pressed_platforms.insert(id);
}
fn press_button(id: int) {
    let button = match BUTTONS.get(id) {
        Option::Some(button) => button,
        Option::None => {
            print("Server sent invalid button number {id}");
            return
        },
    };
    MULTIPLAYER_STATE.pawns.push(Pawn::spawn(button.loc));
    if !MULTIPLAYER_STATE.pressed_buttons.contains(id) {
        MULTIPLAYER_STATE.current_buttons += 1;
    }
    MULTIPLAYER_STATE.pressed_buttons.insert(id);
}
fn player_pressed_new_game(id: int) {
    print(f"player {id} pressed New Game");
    if MULTIPLAYER_STATE.connection != Connection::Connected {
        return;
    }
    if MULTIPLAYER_STATE.ready_state == ReadyState::NotReady {
        MULTIPLAYER_STATE.ready_state = ReadyState::NotReadySomeoneElseReady;
    }
}
fn start_new_game_at(timestamp: int) {
    print(f"start synchronized new game at {timestamp} (current local timestamp: {current_time_millis()})");
    if MULTIPLAYER_STATE.connection != Connection::Connected {
        return;
    }
    MULTIPLAYER_STATE.ready_state = ReadyState::StartingAt(timestamp);
}
fn disconnected(reason: Disconnected) {
    MULTIPLAYER_STATE.ready_state = ReadyState::NotReady;
    MULTIPLAYER_COMPONENT.tick_mode = TickMode::DontCare;
    match reason {
        Disconnected::Closed => MULTIPLAYER_STATE.connection = Connection::Error("Connection Closed"),
        Disconnected::ManualDisconnect => return,
        Disconnected::SendFailed => MULTIPLAYER_STATE.connection = Connection::Error("Send Failed"),
        Disconnected::ConnectionRefused => MULTIPLAYER_STATE.connection = Connection::Error("Connection Refused"),
        Disconnected::ReceiveFailed => MULTIPLAYER_STATE.connection = Connection::Error("Receive Failed"),
        Disconnected::LocalTimeOffsetTooManyTries => MULTIPLAYER_STATE.connection = Connection::Error("Connection too unstable; couldn't get local time offset"),
        Disconnected::RoomNameTooLong => MULTIPLAYER_STATE.connection = Connection::Error("Room name too long"),
    }
    multiplayer_disconnect();
}
