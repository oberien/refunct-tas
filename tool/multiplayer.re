enum Connection {
    Connected,
    Error(string),
    Disconnected,
}
struct MultiplayerState {
    connection: Connection,
    current_room: Option<string>,
    players: Map<int, Player>,
}
struct Player {
    name: string,
    alive: bool,
    loc: Location,
    pawns: List<Pawn>,
}
struct Pawn {
    id: int,
    spawned_at_millis: int,
    at_00: bool,
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
            spawned_at_millis: current_time_millis(),
            at_00: false,
        }
    }
}

static mut MULTIPLAYER_STATE = MultiplayerState {
    connection: Connection::Disconnected,
    current_room: Option::None,
    players: Map::new(),
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
//                    for pawn in player.pawns {
//                        draw_player(player.name, Tas::pawn_location(pawn.id));
//                    }
                }

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
    on_platforms_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
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
        for pawn in player.pawns {
            let loc = Location { x: 0., y: 0., z: -1000. };
            Tas::move_pawn(pawn.id, loc);
            Tas::destroy_pawn(pawn.id);
        }
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

    // only update ~30 times per second (capped at FPS as we are in draw_hud)
    let current_millis = current_time_millis();
    if current_millis - LAST_MILLIS > 33 {
        // update server location
        let loc = Tas::get_location();
        Tas::move_on_server(loc);
        LAST_MILLIS += 33;
    }

    for player_id in MULTIPLAYER_STATE.players.keys() {
        let player = MULTIPLAYER_STATE.players.get(player_id).unwrap();

        let mut i = 0;
        while i < player.pawns.len() {
            // keep last pawn if player is alive
            if i == player.pawns.len() - 1 && player.alive {
                break;
            }

            let mut pawn = player.pawns.get(i).unwrap();
            if pawn.spawned_at_millis + 1000 < current_millis {
                Tas::destroy_pawn(pawn.id);
                player.pawns.swap_remove(i);
                continue;
            } else if !pawn.at_00 && pawn.spawned_at_millis + 125 < current_millis {
                pawn.at_00 = true;
                let x = Rng::gen_int_range(-5000, 5000);
                let y = Rng::gen_int_range(-5000, 5000);
                let loc = Location { x: x.to_float(), y: y.to_float(), z: -2000. };
                Tas::move_pawn(pawn.id, loc);
            }

            i += 1;
        }
        if player.pawns.len() == 0 {
            assert(!player.alive);
            MULTIPLAYER_STATE.players.remove(player_id).unwrap();
        }
    }
}

fn player_joined_multiplayer_room(id: int, name: string, loc: Location) {
    print(f"player {id} joined at x={loc.x}, y={loc.y}, z={loc.z}");
    MULTIPLAYER_STATE.players.insert(id, Player {
        name: name,
        alive: true,
        loc: loc,
        pawns: List::of(Pawn::spawn(loc)),
    });
}
fn player_left_multiplayer_room(id: int) {
    print(f"player {id} left");
    let mut player = MULTIPLAYER_STATE.players.get(id).unwrap();
    player.alive = false;
}
fn player_moved(id: int, loc: Location) {
    let mut player = MULTIPLAYER_STATE.players.get(id).unwrap();
    player.loc = loc;
//    player.pawns.push(Pawn::spawn(loc));
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
