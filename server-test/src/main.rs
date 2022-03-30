use std::io::ErrorKind;
use std::thread;
use std::time::Duration;
use websocket::{ClientBuilder, Message, OwnedMessage, WebSocketError};
use protocol::Request;

include!("locations.in");

fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "Test".to_string());
    // let mut client = ClientBuilder::new("ws://localhost:8080/ws").unwrap().connect(None).unwrap();
    // let mut client = ClientBuilder::new("wss://refunct-tas.oberien.de/ws").unwrap().connect(None).unwrap();
    let mut client = ClientBuilder::new("wss://refunct-tas-test.oberien.de/ws").unwrap().connect(None).unwrap();
    let msg = Request::JoinRoom("Test".to_string(), name, 0.0, 0.0, 0.0);
    client.send_message(&Message::text(serde_json::to_string(&msg).unwrap())).unwrap();
    thread::sleep(Duration::new(5, 0));
    // for &(x, y, z) in LOCATIONS.iter() {
    for req in REQUESTS.iter().cloned() {
        thread::sleep(Duration::new(0, (1_000_000_000 / 60) as u32));
        // let msg = Request::MoveSelf(x, y, z);
        let msg = req;
        client.send_message(&Message::text(serde_json::to_string(&msg).unwrap())).unwrap();
        loop {
            client.set_nonblocking(true).unwrap();
            let res = client.recv_message();
            client.set_nonblocking(false).unwrap();
            match res {
                Ok(OwnedMessage::Text(_)) => (),
                Ok(OwnedMessage::Binary(_) | OwnedMessage::Ping(_) | OwnedMessage::Pong(_)) => continue,
                Err(WebSocketError::IoError(io)) if io.kind() == ErrorKind::WouldBlock => break,
                Ok(OwnedMessage::Close(_)) | Err(_) => break,
            }
        }
    }
}
