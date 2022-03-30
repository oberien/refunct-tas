enum Connection {
    Connected,
    Error(string),
    Disconnected,
}
struct MultiplayerState {
    connection: Connection,
    current_room: Option<string>,
    players: Map<int, Player>,
    pawns: List<Pawn>,
    colored_platforms: Set<int>,
    current_platforms: int,
    current_buttons: int,
}
struct Player {
    name: string,
    loc: Location,
}
struct Pawn {
    id: int,
    spawned_at: int,
}
impl Pawn {
    fn spawn(loc: Location) -> Pawn {
        let rot = Rotation { pitch: 0., yaw: 0., roll: 0. };
        // UE is a perfect game engine without any flaws or anything like that.
        // Not in the least.
        //
        // That is why button 4 does not activate if you spawn pawns on top of it.
        // They get stuck in the platform above and don't collide with anything.
        // However, if you spawn a pawn somewhere completely different, and move it
        // on top of button 4 literally in the same tick in the same frame as the
        // very next instruction to the game engine, it still gets stuck, but for
        // some reason activates the button.
        let loc2 = Location { x: -500., y: -1125., z: 90. };
        let id = Tas::spawn_pawn(loc2, rot);
        Tas::move_pawn(id, loc);
        Pawn {
            id: id,
            spawned_at: current_time_millis(),
        }
    }
}

static mut MULTIPLAYER_STATE = MultiplayerState {
    connection: Connection::Disconnected,
    current_room: Option::None,
    players: Map::new(),
    pawns: List::new(),
    colored_platforms: Set::new(),
    current_platforms: 0,
    current_buttons: 0,
};

static MULTIPLAYER_COMPONENT = Component {
    tick: update_players,
    draw_hud: fn(text: string) -> string {
        match MULTIPLAYER_STATE.connection {
            Connection::Disconnected => return text,
            Connection::Error(err_msg) => return f"{text}\nMultiplayer error: {err_msg}",
            Connection::Connected => {
                // draw players
                for player_id in MULTIPLAYER_STATE.players.keys() {
                    let player = MULTIPLAYER_STATE.players.get(player_id).unwrap();
                    draw_player(player.name, player.loc);
                }
//                for pawn in MULTIPLAYER_STATE.pawns {
//                    draw_player("pawn", Tas::pawn_location(pawn.id));
//                }

                match MULTIPLAYER_STATE.current_room {
                    Option::None => f"{text}\nMultiplayer connected to server",
                    Option::Some(room) => f"{text}\nMultiplayer Room: {room}",
                }
            }
        }
    },
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {
        if old > new {
            assert(new == 0);
            MULTIPLAYER_STATE.current_platforms = new;
            MULTIPLAYER_STATE.colored_platforms = Set::new();
            return;
        }
        if MULTIPLAYER_STATE.current_platforms >= new {
            return;
        }
        MULTIPLAYER_STATE.current_platforms = new;

        let player = Tas::get_location();
        let mut min_distance = 999999.;
        let mut platform_num = 0;
        let mut i = 0;
        for platform in PLATFORMS {
            if MULTIPLAYER_STATE.colored_platforms.contains(i) {
                i += 1;
                continue;
            }
            let x1 = platform.loc.x - platform.size.x;
            let x2 = platform.loc.x + platform.size.x;
            let dx = float::max(x1 - player.x, 0., player.x - x2);
            let y1 = platform.loc.y - platform.size.y;
            let y2 = platform.loc.y + platform.size.y;
            let dy = float::max(y1 - player.y, 0., player.y - y2);
            let z = platform.loc.z + platform.size.z + 89.15;
            let dz = z - player.z;
            let distance = float::sqrt(dx*dx + dy*dy + dz*dz);
            if distance < min_distance {
                min_distance = distance;
                platform_num = i;
            }
            i += 1;
        }
        Tas::press_platform_on_server(platform_num);
        MULTIPLAYER_STATE.colored_platforms.insert(platform_num);
    },
    on_buttons_change: fn(old: int, new: int) {
        if old > new {
            MULTIPLAYER_STATE.current_buttons = new;
            return;
        }
        if MULTIPLAYER_STATE.current_buttons >= new {
            return;
        }
        MULTIPLAYER_STATE.current_buttons = new;

        let player = Tas::get_location();
        let mut min_distance = 999999.;
        let mut button_num = 0;
        let mut i = 0;
        for button in BUTTONS {
            let dx = button.x - player.x;
            let dy = button.y - player.y;
            let dz = button.z - player.z;
            let distance = float::sqrt(dx*dx + dy*dy + dz*dz);
            if distance < min_distance {
                min_distance = distance;
                button_num = i;
            }
            i += 1;
        }
        Tas::press_button_on_server(button_num);
    },
    on_component_exit: fn() { multiplayer_disconnect(); },
};

fn draw_player(name: string, loc: Location) {
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

    fn draw_player_line(start: Vector, end: Vector) {
        if start.z > 0. && end.z > 0. {
            Tas::draw_line(Line {
                startx: start.x,
                starty: start.y,
                endx: end.x,
                endy: end.y,
                color: Color { red: 0., green: 0., blue: 0., alpha: 0. },
                thickness: 3.,
            });
        }
    }

    draw_player_line(a, b);
    draw_player_line(b, c);
    draw_player_line(c, d);
    draw_player_line(d, a);

    draw_player_line(e, f);
    draw_player_line(f, g);
    draw_player_line(g, h);
    draw_player_line(h, e);
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
    MULTIPLAYER_STATE.colored_platforms = Set::new();
    MULTIPLAYER_STATE.current_buttons = level_state.buttons;
    Tas::connect_to_server(Server::Testing);
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
        Tas::destroy_pawn(pawn.id);
    }
    MULTIPLAYER_STATE.players = Map::new();
}
fn multiplayer_join_room(room: string) {
    multiplayer_disconnect();
    multiplayer_connect();
    let loc = Tas::get_location();
    Tas::join_multiplayer_room(room, SETTINGS.multiplayer_name, loc);
    MULTIPLAYER_STATE.current_room = Option::Some(room);
}

fn update_players() {
    static mut LAST_MILLIS = current_time_millis();
    let current_millis = current_time_millis();

    // only update ~30 times per second (capped at FPS as we are in draw_hud)
    if MULTIPLAYER_STATE.connection == Connection::Connected && current_millis - LAST_MILLIS > 33 {
        // update server location
        let loc = Tas::get_location();
        Tas::move_on_server(loc);
        LAST_MILLIS += 33;
    }

    let mut i = 0;
    let pawns = MULTIPLAYER_STATE.pawns;
    while i < pawns.len() {
        let pawn = pawns.get(i).unwrap();
        if pawn.spawned_at + 10000 < current_millis {
            Tas::destroy_pawn(pawn.id);
            pawns.swap_remove(i);
            continue;
        }
        i += 1;
    }
}

fn player_joined_multiplayer_room(id: int, name: string, loc: Location) {
    print(f"player {id} joined at x={loc.x}, y={loc.y}, z={loc.z}");
    MULTIPLAYER_STATE.players.insert(id, Player {
        name: name,
        loc: loc,
    });
}
fn player_left_multiplayer_room(id: int) {
    print(f"player {id} left");
    MULTIPLAYER_STATE.players.remove(id).unwrap();
}
fn player_moved(id: int, loc: Location) {
    let mut player = MULTIPLAYER_STATE.players.get(id).unwrap();
    player.loc = loc;
}
fn platform_pressed(id: int) {
    let platform = match PLATFORMS.get(id) {
        Option::Some(platform) => platform,
        Option::None => {
            print("Server sent invalid platform number {id}");
            return
        },
    };

    let loc = platform_pawn_spawn_location(platform);
    MULTIPLAYER_STATE.pawns.push(Pawn::spawn(loc));
    if !MULTIPLAYER_STATE.colored_platforms.contains(id) {
        MULTIPLAYER_STATE.current_platforms += 1;
    }
    MULTIPLAYER_STATE.colored_platforms.insert(id);
}
fn button_pressed(id: int) {
    let loc = match BUTTONS.get(id) {
        Option::Some(loc) => loc,
        Option::None => {
            print("Server sent invalid button number {id}");
            return
        },
    };
    MULTIPLAYER_STATE.pawns.push(Pawn::spawn(loc));
    MULTIPLAYER_STATE.current_buttons += 1;
}
fn disconnected(reason: Disconnected) {
    match reason {
        Disconnected::Closed => MULTIPLAYER_STATE.connection = Connection::Error("Connection Closed"),
        Disconnected::ManualDisconnect => (),
        Disconnected::SendFailed => MULTIPLAYER_STATE.connection = Connection::Error("Send Failed"),
        Disconnected::ConnectionRefused => MULTIPLAYER_STATE.connection = Connection::Error("Connection Refused"),
        Disconnected::ReceiveFailed => MULTIPLAYER_STATE.connection = Connection::Error("Receive Failed"),
    }
}
