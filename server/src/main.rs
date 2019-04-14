use std::collections::{HashMap, VecDeque};
use std::cell::RefCell;
use std::rc::Rc;
use std::io::{Cursor, Error, ErrorKind};
use std::mem;

use futures::unsync::{mpsc, mpsc::UnboundedSender, mpsc::UnboundedReceiver};
use tokio::prelude::{Future, Stream, future, future::Loop, AsyncWrite};
use tokio::runtime::current_thread;
use tokio::net::TcpListener;
use tokio::io::{self, AsyncRead};
use protocol::{Message, PlayerId};

struct Room {
    members: Vec<(PlayerId, f32, f32, f32, UnboundedSender<Message>)>,
}

impl Room {
    fn new() -> Self {
        Room {
            members: Vec::new(),
        }
    }

    fn join(&mut self, x: f32, y: f32, z: f32, id: PlayerId, tx: UnboundedSender<Message>) {
        self.broadcast(Message::PlayerJoinedRoom(id, x, y, z));
        for &(id, x, y, z, _) in &self.members {
            let _ = tx.unbounded_send(Message::PlayerJoinedRoom(id, x, y, z));
        }
        self.members.push((id, x, y, z, tx));
    }

    fn leave(&mut self, player_id: PlayerId) {
        self.members.retain(|&(id, ..)| id != player_id);
        self.broadcast(Message::PlayerLeftRoom(player_id));
    }

    fn send_others(&self, from_id: PlayerId, msg: Message) {
        for (id, _, _, _, tx) in &self.members {
            if *id != from_id {
                let _ = tx.unbounded_send(msg.clone());
            }
        }
    }

    fn broadcast(&self, msg: Message) {
        self.send_others(0, msg)
    }

    fn update_position(&mut self, player_id: PlayerId, x: f32, y: f32, z: f32) {
        let data = self.members.iter_mut()
            .filter(|(id, ..)| *id == player_id)
            .next()
            .unwrap();
        data.1 = x;
        data.2 = y;
        data.3 = z;

    }
}

struct Rooms {
    rooms: HashMap<String, Room>,
}

impl Rooms {
    fn new() -> Self {
        Rooms {
            rooms: HashMap::new(),
        }
    }

    fn join(&mut self, room: String, x: f32, y: f32, z: f32, player_id: PlayerId, tx: UnboundedSender<Message>) {
        let room = self.rooms.entry(room).or_insert_with(|| Room::new());
        room.join(x, y, z, player_id, tx);
    }

    fn leave(&mut self, room_key: &str, player_id: PlayerId) {
        let room = self.rooms.get_mut(room_key).unwrap();
        room.leave(player_id);
        if room.members.is_empty() {
            self.rooms.remove(room_key);
        }
    }

    fn send_others(&self, room: &str, from_id: PlayerId, msg: Message) {
        self.rooms.get(room).unwrap().send_others(from_id, msg)
    }

    fn update_position(&mut self, room: &str, player_id: PlayerId, x: f32, y: f32, z: f32) {
        self.rooms.get_mut(room).unwrap().update_position(player_id, x, y, z)
    }
}

fn main() {
    let rooms = Rc::new(RefCell::new(Rooms::new()));
    let free_ids = Rc::new(RefCell::new(VecDeque::new()));
    // 0 is a reserved id
    let mut max_id = 1;

    let addr = "0.0.0.0:6337".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming()
        .map_err(|e| eprintln!("accept failed: {:?}", e))
        .for_each(move |sock| {
            let current_player_id = match free_ids.borrow_mut().pop_front() {
                Some(id) => id,
                None => {
                    let id = max_id;
                    max_id += 1;
                    id
                },
            };
            eprintln!("connection from {:?} as {}", sock.peer_addr(), current_player_id);
            let current_player_room = Rc::new(RefCell::new(None));

            let (reader, writer) = sock.split();
            let (tx, rx) = mpsc::unbounded();

            let receive = receive(
                reader, current_player_id, tx, Rc::clone(&rooms), Rc::clone(&current_player_room),
            );
            let send = send(writer, rx);

            let current_player_room = Rc::clone(&current_player_room);
            let rooms = Rc::clone(&rooms);
            let free_ids = Rc::clone(&free_ids);
            let client = send.select(receive)
                .then(move |res| {
                    if let Some(room) = &*current_player_room.borrow() {
                        rooms.borrow_mut().leave(room, current_player_id);
                    }
                    free_ids.borrow_mut().push_back(current_player_id);
                    match res {
                        Ok(_) => eprintln!("No idea how I got here (client stopped with Ok)"),
                        Err((e, _)) => eprintln!("Client {} ceased existence: {:?}", current_player_id, e),
                    }
                    Ok(())
                });
            current_thread::spawn(client);
            Ok(())
        });
    current_thread::run(server);
}

fn send(
    writer: impl AsyncWrite, rx: UnboundedReceiver<Message>
) -> impl Future<Item = (), Error = Error> {
    rx.map_err(|()| Error::new(ErrorKind::BrokenPipe, "rx broke"))
    .fold((writer, Vec::with_capacity(100)), |(writer, mut vec), msg| {
        vec.clear();
        msg.serialize(&mut vec).unwrap();
        io::write_all(writer, vec)
    }).map(|_| eprintln!("ended send without error???"))
}

fn receive(
    reader: impl AsyncRead, current_player_id: PlayerId, tx: UnboundedSender<Message>,
    rooms: Rc<RefCell<Rooms>>, current_player_room: Rc<RefCell<Option<String>>>,
) -> impl Future<Item = (), Error = Error> {
    future::loop_fn((reader, Vec::with_capacity(100)), move |(reader, vec)| {
        let rooms = Rc::clone(&rooms);
        let current_player_room = Rc::clone(&current_player_room);
        let tx = tx.clone();
        io::read(reader, vec).and_then(move |(reader, vec, _len)| {
            let mut cursor = Cursor::new(vec);
            loop {
                let msg = match Message::deserialize(&mut cursor) {
                    Ok(msg) => msg,
                    // we need to read more
                    Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                        let consumed = cursor.position() as usize;
                        let mut vec = cursor.into_inner();
                        vec.drain(..consumed);
                        return Ok(Loop::Continue((reader, vec)));
                    },
                    Err(e) => Err(e)?,
                };
                match msg {
                    Message::JoinRoom(room, x, y, z) => {
                        eprintln!("{} joined {:?}", current_player_id, room);
                        let mut rooms = rooms.borrow_mut();

                        let old = mem::replace(&mut *current_player_room.borrow_mut(), Some(room.clone()));
                        if let Some(old) = old {
                            rooms.leave(&old, current_player_id);
                        }

                        rooms.join(room, x, y, z, current_player_id, tx.clone());
                    },
                    Message::MoveSelf(x, y, z) => {
                        let current_player_room = current_player_room.borrow();
                        let room = match &*current_player_room {
                            Some(room) => room,
                            None => return Err(Error::new(ErrorKind::InvalidInput, "client moved before being in room")),
                        };
                        rooms.borrow_mut().update_position(room, current_player_id, x, y, z);
                        let msg = Message::MoveOther(current_player_id, x, y, z);
                        rooms.borrow().send_others(room, current_player_id, msg);
                    },
                    Message::PlayerJoinedRoom(..)
                    | Message::PlayerLeftRoom(_)
                    | Message::MoveOther(..) => {
                        return Err(Error::new(ErrorKind::InvalidInput, format!("client sent invalid message: {:?}", msg)));
                    },
                }
            }
        })
    })
}
