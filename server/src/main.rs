use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use tokio::sync::{Mutex, RwLock};
use protocol::{PlayerId, Request, Response};

struct State {
    multiplayer_rooms: HashMap<String, MultiplayerRoom>,
}

#[derive(Clone, Default)]
struct MultiplayerRoom {
    players: Arc<RwLock<HashMap<PlayerId, Arc<Mutex<Player>>>>>,
    name: String,
}

impl MultiplayerRoom {
    async fn broadcast(&self, sender: PlayerId, message: &Response) {
        let players: Vec<_> = self.players.read().await.iter().map(|(k, v)| (*k, Arc::clone(v))).collect();
        for (id, player) in players {
            if id == sender {
                continue;
            }
            player.lock().await.send(message).await;
        }
    }
}

struct Player {
    id: PlayerId,
    name: String,
    x: f32,
    y: f32,
    z: f32,
    sender: SplitSink<WebSocket, Message>,
}
impl Player {
    async fn send(&mut self, message: &Response) {
        let _ = self.sender.send(Message::Text(serde_json::to_string(&message).unwrap())).await;
    }
}

#[tokio::main]
async fn main() {
    console_subscriber::init();
    env_logger::init();

    let state = Arc::new(Mutex::new(State {
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

async fn hello_world(state: Arc<Mutex<State>>) -> Html<String> {
    let mut res = "<html><body>Rooms:<ul>".to_string();
    let rooms: Vec<_> = state.lock().await.multiplayer_rooms.values().cloned().collect();
    for room in rooms {
        let players: Vec<_> = room.players.read().await.values().cloned().collect();
        res += &format!("<li>{} ({}):<ul>", room.name, players.len());
        for player in players {
            let player = player.lock().await;
            res += &format!("<li>{}: {} {} {}", player.id.id(), player.x, player.y, player.z);
        }
        res += "</ul></li>";
    }
    res += "</ul></body></html>";
    Html(res)
}

async fn handle_socket_upgrade(ws: WebSocketUpgrade, state: Arc<Mutex<State>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async { handle_socket(socket, state).await })
}

async fn handle_socket(socket: WebSocket, state: Arc<Mutex<State>>) {
    let (sender, mut receiver) = socket.split();
    let mut sender = Some(sender);

    let player_id = PlayerId::next();
    log::info!("Player connected: {:?}", player_id);
    let multiplayer_room: Mutex<Option<MultiplayerRoom>> = Mutex::new(None);

    let remove_from_current_room = || async {
        if let Some(room) = multiplayer_room.lock().await.take() {
            log::debug!("Removed {player_id:?} from room {:?}", room.name);
            let player = room.players.write().await.remove(&player_id);

            if player.is_some() {
                room.broadcast(player_id, &Response::PlayerLeftRoom(player_id)).await;
            }

            player
        } else {
            None
        }
    };
    let disconnect = || async {
        remove_from_current_room().await;
        log::info!("Player Disconnected: {player_id:?}");
    };

    while let Some(msg) = receiver.next().await {
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
            Request::JoinRoom(room_name, player_name, x, y, z) => {
                log::info!("Player {player_id:?} ({player_name}) joins room {room_name:?}");

                let player = match remove_from_current_room().await {
                    Some(player) => {
                        {
                            let mut player = player.lock().await;
                            player.x = x;
                            player.y = y;
                            player.z = z;
                            player.name = player_name.clone();
                        }
                        player
                    },
                    None => Arc::new(Mutex::new(Player { id: player_id, name: player_name.clone(), x, y, z, sender: sender.take().unwrap() })),
                };
                let room = state.lock().await.multiplayer_rooms.entry(room_name)
                    .or_insert_with_key(|key| MultiplayerRoom { players: Default::default(), name: key.clone() })
                    .clone();

                let players: Vec<_> = room.players.read().await.iter().map(|(k, v)| (*k, Arc::clone(v))).collect();
                for (id, other_player) in players {
                    let (x, y, z, name) = {
                        let mut other_player = other_player.lock().await;
                        other_player.send(&Response::PlayerJoinedRoom(player_id, player_name.clone(), x, y, z)).await;
                        (other_player.x, other_player.y, other_player.z, other_player.name.clone())
                    };
                    player.lock().await.send(&Response::PlayerJoinedRoom(id, name, x, y, z)).await;
                }

                room.players.write().await.insert(player_id, player);
                *multiplayer_room.lock().await = Some(room);
            }
            Request::MoveSelf(x, y, z) => {
                let lock = multiplayer_room.lock().await;
                let room = match lock.as_ref() {
                    Some(name) => name,
                    None => {
                        log::warn!("Player {player_id:?} tried to move without being in a room to {x} {y} {z}");
                        continue
                    }
                };
                // update player's location
                match room.players.write().await.get(&player_id).cloned() {
                    Some(player) => {
                        let mut player = player.lock().await;
                        player.x = x;
                        player.y = y;
                        player.z = z;
                    },
                    None => {
                        log::error!("Player {player_id:?} doesn't exist in room {:?}", room.name);
                        continue
                    }
                };

                // inform other player's about the new location
                room.broadcast(player_id, &Response::MoveOther(player_id, x, y, z)).await;
            }
        }
    }
}
