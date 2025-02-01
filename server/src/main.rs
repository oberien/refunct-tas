use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use futures::{SinkExt, StreamExt};
use std::sync::{Mutex as StdMutex, RwLock as StdRwLock};
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex as TokioMutex;
use tokio::sync::mpsc::{self, Sender};
use protocol::{PlayerId, Request, Response};

struct State {
    multiplayer_rooms: HashMap<String, MultiplayerRoom>,
}

#[derive(Clone, Default)]
struct MultiplayerRoom {
    players: Arc<StdRwLock<HashMap<PlayerId, Arc<Player>>>>,
    name: String,
}

impl MultiplayerRoom {
    async fn broadcast(&self, sender: Option<PlayerId>, message: Response) {
        let players = self.players.read().unwrap();
        for (id, player) in players.iter() {
            if Some(*id) == sender {
                continue;
            }
            player.send(message.clone());
        }
    }
    /// check if all players pressed "New Game"
    async fn check_new_game(&self) {
        let players: Vec<_> = self.players.read().unwrap().values().cloned().collect();
        if players.iter().all(|p| *p.is_waiting_for_new_game.lock().unwrap()) {
            for player in players.iter() {
                *player.is_waiting_for_new_game.lock().unwrap() = false;
            }
            let time = SystemTime::now();
            let when_to_start = time + Duration::from_millis(5000);
            let timestamp = when_to_start.duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
            self.broadcast(None, Response::StartNewGameAt(timestamp)).await;
        }
    }
}

struct Player {
    id: PlayerId,
    data: StdMutex<PlayerData>,
    sender: Sender<Response>,
    is_waiting_for_new_game: StdMutex<bool>,
}
struct PlayerData {
    name: String,
    red: f32,
    green: f32,
    blue: f32,
    x: f32,
    y: f32,
    z: f32,
    pitch: f32,
    yaw: f32,
    roll: f32,
}
impl Player {
    fn send(&self, message: Response) {
        let _ = self.sender.try_send(message);
    }
}

#[tokio::main]
async fn main() {
    // console_subscriber::init();
    env_logger::init();

    let state = Arc::new(StdMutex::new(State {
        multiplayer_rooms: HashMap::new(),
    }));

    let app = Router::new()
        .route("/", get({
            let state = Arc::clone(&state);
            move || hello_world(state)
        })).route("/ws", get({
            let state = Arc::clone(&state);
            move |ws| handle_socket_upgrade(ws, state)
        }))
    ;

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world(state: Arc<StdMutex<State>>) -> Html<String> {
    let mut res = "<html><body>Rooms:<ul>".to_string();
    let rooms: Vec<_> = state.lock().unwrap().multiplayer_rooms.values().cloned().collect();
    for room in rooms {
        let players = room.players.read().unwrap();
        res += &format!("<li>{} ({}):<ul>", room.name, players.len());
        for player in players.values() {
            let data = player.data.lock().unwrap();
            let is_waiting_for_new_game = *player.is_waiting_for_new_game.lock().unwrap();
            res += &format!("<li>{}({}): is_waiting_for_new_game: {}, location: x={} y={} z={}", data.name, player.id.id(), is_waiting_for_new_game, data.x, data.y, data.z);
        }
        res += "</ul></li>";
    }
    res += "</ul></body></html>";
    Html(res)
}

async fn handle_socket_upgrade(ws: WebSocketUpgrade, state: Arc<StdMutex<State>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async { handle_socket(socket, state).await })
}

async fn handle_socket(socket: WebSocket, state: Arc<StdMutex<State>>) {
    let (mut wstx, mut wsrx) = socket.split();

    // spawn writing task
    let (sender, mut receiver) = mpsc::channel(1000);
    tokio::spawn(async move {
        loop {
            match receiver.recv().await {
                Some(msg) => {
                    let _ = wstx.send(Message::Text(serde_json::to_string(&msg).unwrap())).await;
                }
                None => break,
            }
        }
    });

    let local_sender = sender.clone();
    let mut sender = Some(sender);

    let player_id = PlayerId::next();
    log::info!("Player connected: {:?}", player_id);
    let multiplayer_room: TokioMutex<Option<MultiplayerRoom>> = TokioMutex::new(None);

    let remove_from_current_room = || async {
        if let Some(room) = multiplayer_room.lock().await.take() {
            log::debug!("Removed {player_id:?} from room {:?}", room.name);
            let player = room.players.write().unwrap().remove(&player_id);

            if player.is_some() {
                room.broadcast(Some(player_id), Response::PlayerLeftRoom(player_id)).await;
            }
            room.check_new_game().await;

            player
        } else {
            None
        }
    };
    let disconnect = || async {
        remove_from_current_room().await;
        log::info!("Player Disconnected: {player_id:?}");
    };

    while let Some(msg) = wsrx.next().await {
        let msg = match msg {
            Ok(Message::Close(_)) | Err(_) => {
                disconnect().await;
                return
            },
            Ok(Message::Binary(_)) => {
                log::warn!("Got binary from player {player_id:?}, disconnecting...");
                disconnect().await;
                return
            }
            Ok(Message::Ping(_)) | Ok(Message::Pong(_)) => continue,
            Ok(Message::Text(text)) => text,
        };

        let request = match serde_json::from_str(&msg) {
            Ok(request) => request,
            Err(e) => {
                log::warn!("Got invalid JSON from player {player_id:?} ({e:?}), disconnecting...");
                disconnect().await;
                return
            }
        };

        match request {
            Request::GetServerTime => {
                let _ = local_sender.send(Response::ServerTime(SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64)).await;
            },
            Request::JoinRoom(room_name, player_name, red, green, blue, x, y, z, pitch, yaw, roll) => {
                if room_name.len() > 128 {
                    log::warn!("Player {player_id:?} ({player_name}) tried to join room {room_name:?}, but room name is greater than 128 chars.");
                    let _ = local_sender.send(Response::RoomNameTooLong).await;
                    continue
                }
                log::info!("Player {player_id:?} ({player_name}) joins room {room_name:?}");

                let player = match remove_from_current_room().await {
                    Some(player) => {
                        {
                            let mut data = player.data.lock().unwrap();
                            data.red = red;
                            data.green = green;
                            data.blue = blue;
                            data.x = x;
                            data.y = y;
                            data.z = z;
                            data.pitch = pitch;
                            data.yaw = yaw;
                            data.roll = roll;
                            data.name = player_name.clone();
                        }
                        player
                    },
                    None => Arc::new(Player {
                        id: player_id,
                        is_waiting_for_new_game: StdMutex::new(false),
                        data: StdMutex::new(PlayerData { name: player_name.clone(), red, green, blue, x, y, z, pitch, yaw, roll }),
                        sender: sender.take().unwrap()
                    }),
                };
                let room = state.lock().unwrap().multiplayer_rooms.entry(room_name)
                    .or_insert_with_key(|key| MultiplayerRoom { players: Default::default(), name: key.clone() })
                    .clone();

                {
                    let players = room.players.read().unwrap();
                    for (id, other_player) in &*players {
                        let (red, green, blue, x, y, z, pitch, yaw, roll, is_waiting_for_new_game, name) = {
                            let data = other_player.data.lock().unwrap();
                            other_player.send(Response::PlayerJoinedRoom(player_id, player_name.clone(), red, green, blue, x, y, z, pitch, yaw, roll));
                            (data.red, data.green, data.blue, data.x, data.y, data.z, data.pitch, data.yaw, data.roll, *other_player.is_waiting_for_new_game.lock().unwrap(), data.name.clone())
                        };
                        player.send(Response::PlayerJoinedRoom(*id, name, red, green, blue, x, y, z, pitch, yaw, roll));
                        if is_waiting_for_new_game {
                            player.send(Response::NewGamePressed(*id));
                        }
                    }
                }

                room.players.write().unwrap().insert(player_id, player);
                *multiplayer_room.lock().await = Some(room);
            }
            Request::MoveSelf(x, y, z, pitch, yaw, roll) => {
                let lock = multiplayer_room.lock().await;
                let room = match lock.as_ref() {
                    Some(name) => name,
                    None => {
                        log::warn!("Player {player_id:?} tried to move without being in a room to {x} {y} {z}");
                        continue
                    }
                };
                // update player's location
                let player = room.players.read().unwrap().get(&player_id).cloned();
                match player {
                    Some(player) => {
                        let mut data = player.data.lock().unwrap();
                        data.x = x;
                        data.y = y;
                        data.z = z;
                        data.pitch = pitch;
                        data.yaw = yaw;
                        data.roll = roll;
                    },
                    None => {
                        log::error!("Player {player_id:?} tried to update its location without being in room {:?}", room.name);
                        continue
                    }
                };

                room.broadcast(Some(player_id), Response::MoveOther(player_id, x, y, z, pitch, yaw, roll)).await;
            }
            Request::PressPlatform(id) => {
                let lock = multiplayer_room.lock().await;
                let room = match lock.as_ref() {
                    Some(name) => name,
                    None => {
                        log::warn!("Player {player_id:?} tried to press platform {id} without being in a room");
                        continue
                    }
                };
                room.broadcast(Some(player_id), Response::PressPlatform(id)).await;
            }
            Request::PressButton(id) => {
                let lock = multiplayer_room.lock().await;
                let room = match lock.as_ref() {
                    Some(name) => name,
                    None => {
                        log::warn!("Player {player_id:?} tried to press button {id} without being in a room");
                        continue
                    }
                };
                room.broadcast(Some(player_id), Response::PressButton(id)).await;
            }
            Request::NewGamePressed => {
                log::info!("Player {player_id:?} pressed New Game");
                let lock = multiplayer_room.lock().await;
                let room = match lock.as_ref() {
                    Some(name) => name,
                    None => {
                        log::warn!("Player {player_id:?} tried to press new game while not in a room");
                        continue
                    }
                };
                let player = room.players.read().unwrap().get(&player_id).cloned();
                match player {
                    Some(player) => {
                        *player.is_waiting_for_new_game.lock().unwrap() = true;
                    },
                    None => {
                        log::error!("Player {player_id:?} pressed new game but isn't in room {:?}", room.name);
                        continue
                    }
                };

                room.broadcast(Some(player_id), Response::NewGamePressed(player_id)).await;
                room.check_new_game().await;
            }
        }
    }
}
