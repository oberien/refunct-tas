struct MultiplayerState {
    connected: bool,
    current_room: Option<string>,
    players: Map<int, Player>,
}
struct Player {
    pawn_id: int,
    loc: Location,
}

static mut MULTIPLAYER_STATE = MultiplayerState {
    connected: false,
    current_room: Option::None,
    players: Map::new(),
};

static MULTIPLAYER_COMPONENT = Component {
    draw_hud: fn(text: string) -> string {
        if !MULTIPLAYER_STATE.connected {
            return text;
        }

        // update server location
        let loc = Tas::get_location();
        Tas::move_on_server(loc);

        // draw pawns
        for player in MULTIPLAYER_STATE.players.values() {
            let loc = player.loc;
            draw_player(Location { x: loc.x, y: loc.y, z: loc.z });
        }

        match MULTIPLAYER_STATE.current_room {
            Option::None => f"{text}\nMultiplayer connected to server",
            Option::Some(room) => f"{text}\nMultiplayer Room: {room}",
        }
    },
    on_new_game: fn() {},
    on_level_change: fn(level: int) {},
    on_reset: fn(reset: int) {},
};

fn draw_player(mut loc: Location) {
    // print(f"draw {loc:?}");
    loc.z -= 100.;
    let a = Tas::project(Vector { x: loc.x-50., y: loc.y, z: loc.z});
    let b = Tas::project(Vector { x: loc.x+50., y: loc.y, z: loc.z});
    let c = Tas::project(Vector { x: loc.x-50., y: loc.y, z: loc.z+200.});
    let d = Tas::project(Vector { x: loc.x+50., y: loc.y, z: loc.z+200.});

    let e = Tas::project(Vector { x: loc.x, y: loc.y-50., z: loc.z});
    let f = Tas::project(Vector { x: loc.x, y: loc.y+50., z: loc.z});
    let g = Tas::project(Vector { x: loc.x, y: loc.y-50., z: loc.z+200.});
    let h = Tas::project(Vector { x: loc.x, y: loc.y+50., z: loc.z+200.});

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
}

fn multiplayer_connect() {
    if MULTIPLAYER_STATE.connected {
        return;
    }
    Tas::connect_to_server();
    MULTIPLAYER_STATE.connected = true;
}
fn multiplayer_disconnect() {
    Tas::disconnect_from_server();
    MULTIPLAYER_STATE.connected = false;
    MULTIPLAYER_STATE.current_room = Option::None;
    MULTIPLAYER_STATE.players = Map::new();
}
fn multiplayer_join_room(room: string) {
    if !MULTIPLAYER_STATE.connected {
        return;
    }
    let loc = Tas::get_location();
    Tas::join_multiplayer_room(room, loc);
    MULTIPLAYER_STATE.current_room = Option::Some(room);
}

fn player_joined_multiplayer_room(id: int, loc: Location) {
    print(f"player {id} joined at x={loc.x}, y={loc.y}, z={loc.z}");
    let pawn_id = Tas::spawn_pawn();
    Tas::move_pawn(pawn_id, loc);
    MULTIPLAYER_STATE.players.insert(id, Player {
        pawn_id: pawn_id,
        loc: loc,
    });
}
fn player_left_multiplayer_room(id: int) {
    print(f"player {id} left");
    let player = MULTIPLAYER_STATE.players.remove(id).unwrap();
    Tas::destroy_pawn(player.pawn_id);
}
fn player_moved(id: int, loc: Location) {
    let mut player = MULTIPLAYER_STATE.players.get(id).unwrap();
    Tas::move_pawn(player.pawn_id, loc);
    // print(f"moved {loc:?}");
    // player.loc = loc;
    player.loc.x = loc.x;
    player.loc.y = loc.y;
    player.loc.z = loc.z;
    // print(f"bar {loc:?}");
    // print(f"foo {player.loc:?}");
}

