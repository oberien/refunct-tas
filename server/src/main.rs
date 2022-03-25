use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use futures::{SinkExt, StreamExt};
use std::sync::{Mutex as StdMutex, RwLock as StdRwLock};
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
    async fn broadcast(&self, sender: PlayerId, message: Response) {
        let players = self.players.read().unwrap();
        for (id, player) in players.iter() {
            if *id == sender {
                continue;
            }
            player.send(message.clone());
        }
    }
}

struct Player {
    id: PlayerId,
    data: StdMutex<PlayerData>,
    sender: Sender<Response>,
}
struct PlayerData {
    name: String,
    x: f32,
    y: f32,
    z: f32,
}
impl Player {
    fn send(&self, message: Response) {
        let _ = self.sender.try_send(message);
    }
}

#[tokio::main]
async fn main() {
    console_subscriber::init();
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
            res += &format!("<li>{}({}): {} {} {}", data.name, player.id.id(), data.x, data.y, data.z);
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

    let mut sender = Some(sender);

    let player_id = PlayerId::next();
    log::info!("Player connected: {:?}", player_id);
    let multiplayer_room: TokioMutex<Option<MultiplayerRoom>> = TokioMutex::new(None);

    let remove_from_current_room = || async {
        if let Some(room) = multiplayer_room.lock().await.take() {
            log::debug!("Removed {player_id:?} from room {:?}", room.name);
            let player = room.players.write().unwrap().remove(&player_id);

            if player.is_some() {
                room.broadcast(player_id, Response::PlayerLeftRoom(player_id)).await;
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
            Request::JoinRoom(room_name, player_name, x, y, z) => {
                log::info!("Player {player_id:?} ({player_name}) joins room {room_name:?}");

                let player = match remove_from_current_room().await {
                    Some(player) => {
                        {
                            let mut data = player.data.lock().unwrap();
                            data.x = x;
                            data.y = y;
                            data.z = z;
                            data.name = player_name.clone();
                        }
                        player
                    },
                    None => Arc::new(Player {
                        id: player_id,
                        data: StdMutex::new(PlayerData { name: player_name.clone(), x, y, z }),
                        sender: sender.take().unwrap()
                    }),
                };
                let room = state.lock().unwrap().multiplayer_rooms.entry(room_name)
                    .or_insert_with_key(|key| MultiplayerRoom { players: Default::default(), name: key.clone() })
                    .clone();

                {
                    let players = room.players.read().unwrap();
                    for (id, other_player) in &*players {
                        let (x, y, z, name) = {
                            let data = other_player.data.lock().unwrap();
                            other_player.send(Response::PlayerJoinedRoom(player_id, player_name.clone(), x, y, z));
                            (data.x, data.y, data.z, data.name.clone())
                        };
                        player.send(Response::PlayerJoinedRoom(*id, name, x, y, z));
                    }
                }

                room.players.write().unwrap().insert(player_id, player);
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
                let player = room.players.read().unwrap().get(&player_id).cloned();
                match player {
                    Some(player) => {
                        let mut data = player.data.lock().unwrap();
                        data.x = x;
                        data.y = y;
                        data.z = z;
                    },
                    None => {
                        log::error!("Player {player_id:?} doesn't exist in room {:?}", room.name);
                        continue
                    }
                };

                // inform other player's about the new location
                room.broadcast(player_id, Response::MoveOther(player_id, x, y, z)).await;
            }
        }
    }
}
