use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use tokio::sync::Mutex;
use protocol::{PlayerId, Request, Response};

#[derive(Default)]
struct MultiplayerRoom {
    players: HashMap<PlayerId, Player>,
}

struct Player {
    id: PlayerId,
    x: f32,
    y: f32,
    z: f32,
    sender: SplitSink<WebSocket, Message>,
}

struct State {
    multiplayer_rooms: HashMap<String, MultiplayerRoom>,
}

#[tokio::main]
async fn main() {
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
    let state = state.lock().await;
    let mut res = "<html><body>Rooms:<ul>".to_string();
    for (name, room) in &state.multiplayer_rooms {
        res += &format!("<li>{name} ({}):<ul>", room.players.len());
        for player in room.players.values() {
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
    let multiplayer_room = Mutex::new(None);

    let remove_from_current_room = || async {
        if let Some(name) = multiplayer_room.lock().await.take() {
            match state.lock().await.multiplayer_rooms.get_mut(&name) {
                Some(room) => {
                    log::debug!("Removed {player_id:?} from room {name:?}");
                    let player = room.players.remove(&player_id);

                    if player.is_some() {
                        for (_, player) in &mut room.players {
                            let _ = player.sender.send(Message::Text(serde_json::to_string(&Response::PlayerLeftRoom(player_id)).unwrap())).await;
                        }
                    }

                    player
                },
                None => {
                    log::error!("Tried to remove player {player_id:?} from nonexistent room {name:?}");
                    None
                },
            }
        } else {
            None
        }
    };
    let disconnect = || async {
        remove_from_current_room().await;
        log::info!("Player Disconnected: {player_id:?}");
    };

    while let Some(msg) = receiver.next().await {
        if let Ok(Message::Binary(_)) = &msg {
        }
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
            Request::JoinRoom(name, x, y, z) => {
                let mut sender = match remove_from_current_room().await {
                    Some(player) => player.sender,
                    None => sender.take().unwrap(),
                };

                log::info!("Player {player_id:?} joins room {name:?}");
                *multiplayer_room.lock().await = Some(name.clone());
                let mut state = state.lock().await;
                let room = state.multiplayer_rooms.entry(name).or_default();

                for (id, player) in &mut room.players {
                    let _ = player.sender.send(Message::Text(serde_json::to_string(&Response::PlayerJoinedRoom(player_id, x, y, z)).unwrap())).await;
                    let _ = sender.send(Message::Text(serde_json::to_string(&Response::PlayerJoinedRoom(*id, player.x, player.y, player.z)).unwrap())).await;
                }

                room.players.insert(player_id, Player { id: player_id, x, y, z, sender, });
            }
            Request::MoveSelf(x, y, z) => {
                let room = multiplayer_room.lock().await;
                let room_name = match room.as_ref() {
                    Some(name) => name,
                    None => {
                        log::warn!("Player {player_id:?} tried to move without being in a room to {x} {y} {z}");
                        continue
                    }
                };
                let mut state = state.lock().await;
                let room = match state.multiplayer_rooms.get_mut(room_name) {
                    Some(room) => room,
                    None => {
                        log::error!("tried to get nonexistent room {room_name:?} by player {player_id:?}");
                        continue
                    }
                };
                let player = match room.players.get_mut(&player_id) {
                    Some(player) => player,
                    None => {
                        log::error!("Player {player_id:?} doesn't exist in room {room_name:?}");
                        continue
                    }
                };

                player.x = x;
                player.y = y;
                player.z = z;

                for (id, player) in &mut room.players {
                    if *id == player_id {
                        continue;
                    }
                    let _ = player.sender.send(Message::Text(serde_json::to_string(&Response::MoveOther(player_id, x, y, z)).unwrap())).await;
                }
            }
        }
    }
}
