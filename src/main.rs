extern crate uuid;

use std::time::Duration;
use std::io::{Write};
use uuid::Uuid;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::thread;
use std::fs;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

fn main() {
    println!("Start:");
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let mut raft = Raft::new(UnixSocketIO::new(1, tx), rx);

    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let mut raft2 = Raft::new(UnixSocketIO::new(2, tx), rx);

    raft.listen();
    raft2.listen();

    let mut candidate = Raft::<Candidate, UnixSocketIO>::from(raft);
    candidate.request_vote(2);


    loop {
        match raft2.inbox.recv() {
            Ok(msg) => { println!("OK: {:?}", msg) },
            Err(e) => { println!("ERR: {:?}", e) },
        };
    }
}

type NodeId = u8;

#[derive(Debug)]
pub struct VoteRequest {
    id: Uuid,
    from: NodeId,
    to: NodeId,
}

impl VoteRequest {
    pub fn new(from: NodeId, to: NodeId) -> VoteRequest {
        VoteRequest {
            id: Uuid::new_v4(),
            from,
            to,
        }
    }
}

#[derive(Debug)]
pub struct VoteResponse {
    id: Uuid,
    from: NodeId,
    to: NodeId,
    response: bool,
}

impl VoteResponse {
    fn new(req: VoteRequest, response: bool) -> VoteResponse {
        VoteResponse {
            id: req.id,
            from: req.to,
            to: req.from,
            response,
        }
    }
}

pub trait RaftState {
    fn get_state(&self) -> &State;
    fn handle_message(&mut self, message: Message);
}

pub struct Raft<S: RaftState, IO: RaftIO> {
    id: NodeId,
    state: S,
    io: IO,
    inbox: Receiver<Message>,
}

#[derive(Debug)]
pub enum Message {
    VoteRequest(VoteRequest),
    VoteResponse(VoteResponse),
    Debug
}

impl<S: RaftState, IO: RaftIO> Raft<S, IO> {
    pub fn get_state(&self) -> &State {
        self.state.get_state()
    }

    pub fn listen(&mut self) {
        self.io.listen();
        thread::sleep(Duration::new(0,0));
    }
}

pub trait RaftIO {
    fn listen(&mut self);
    fn request_vote(&mut self, vote_request: VoteRequest) -> Result<VoteResponse, &str>;
}

struct NoopIO {}

impl NoopIO {
    pub fn new() -> NoopIO {
        NoopIO {}
    }
}

struct UnixSocketIO {
    node_id: NodeId,
    sockets: HashMap<NodeId, UnixStream>,
    sender: Sender<Message>,
}

impl UnixSocketIO {
    pub fn new(node_id: NodeId, sender: Sender<Message>) -> UnixSocketIO {
        UnixSocketIO {
            node_id,
            sockets: HashMap::new(),
            sender
        }
    }

    fn socket_path(&self, node_id: &NodeId) -> String {
        format!("/tmp/raft.{}.sock", &node_id)
    }

    fn connect(&mut self, node_id: NodeId) -> Result<&UnixStream, std::io::Error> {
        if self.sockets.contains_key(&node_id) {
            return Ok(self.sockets.get(&node_id).unwrap());
        }

        let path_name = self.socket_path(&node_id);
        let path = Path::new(&path_name);

        let socket = UnixStream::connect(path)?;
        self.sockets.insert(node_id, socket);
        Ok(self.sockets.get(&node_id).unwrap())
    }
}

impl RaftIO for UnixSocketIO {
    fn listen(&mut self) {
        let path_name = self.socket_path(&self.node_id);
        let tx = self.sender.clone();

        thread::spawn(move || {
            let path = Path::new(&path_name);

            // Cleanup socket if it already exists
            if path.exists() {
                fs::remove_file(path).unwrap();
            }

            let socket = UnixListener::bind(path).unwrap();

            for stream in socket.incoming() {
                match stream {
                    Ok(stream) => {
                        tx.send(Message::Debug);
                    },
                    Err(err) => panic!("Error!")
                }
            }
        });
    }

    fn request_vote(&mut self, vote_request: VoteRequest) -> Result<VoteResponse, &str> {
        println!("Requesting vote {}", vote_request.to);
        let mut socket = self.connect(vote_request.to).unwrap();
        socket.write_all(b"wow");
        Ok(VoteResponse::new(vote_request, true))
    }
}

impl RaftIO for NoopIO {
    fn listen(&mut self) {
        println!("Listening...");
    }

    fn request_vote(&mut self, vote_request: VoteRequest) -> Result<VoteResponse, &str> {
        println!("Requesting vote from {:?}", vote_request.to);
        Ok(VoteResponse::new(vote_request, true))
    }
}

struct Leader {
    state: State
}

#[derive(Debug)]
pub enum State {
    Leader,
    Follower,
    Candidate,
}

impl RaftState for Leader {
    fn get_state(&self) -> &State {
        &self.state
    }

    fn handle_message(&mut self, message: Message) {
        match message {
            Message::VoteRequest(_) => {}
            Message::VoteResponse(_) => {}
            Message::Debug => {}
        }
    }
}

struct Candidate {
    state: State,
}

impl RaftState for Candidate {
    fn get_state(&self) -> &State {
        &self.state
    }
    fn handle_message(&mut self, message: Message) {
        match message {
            Message::VoteRequest(_) => {}
            Message::VoteResponse(_) => {}
            Message::Debug => {}
        }
    }
}

struct Follower {
    leader_id: Option<NodeId>,
    state: State,
}

impl RaftState for Follower {
    fn get_state(&self) -> &State {
        &self.state
    }

    fn handle_message(&mut self, message: Message) {
        match message {
            Message::VoteRequest(_) => {}
            Message::VoteResponse(_) => {}
            Message::Debug => {}
        }
    }
}

// Raft starts in the Follower state
impl<IO: RaftIO> Raft<Follower, IO> {
    fn new(io: IO, inbox: Receiver<Message>) -> Self {
        Raft {
            id: 1,
            io,
            state: Follower { leader_id: None, state: State::Follower },
            inbox
        }
    }
}

impl<IO: RaftIO> Raft<Candidate, IO> {
    fn request_vote(&mut self, node_id: NodeId) -> Result<VoteResponse, &str> {
        self.io.request_vote(VoteRequest::new(self.id, node_id))
    }
}

impl<IO: RaftIO> Raft<Leader, IO> {}

impl<IO: RaftIO> From<Raft<Follower, IO>> for Raft<Candidate, IO> {
    fn from(it: Raft<Follower, IO>) -> Raft<Candidate, IO> {
        Raft {
            id: it.id,
            io: it.io,
            state: Candidate { state: State::Candidate },
            inbox: it.inbox,
        }
    }
}

impl<IO: RaftIO> From<Raft<Candidate, IO>> for Raft<Follower, IO> {
    fn from(it: Raft<Candidate, IO>) -> Raft<Follower, IO> {
        Raft {
            id: it.id,
            io: it.io,
            state: Follower { leader_id: None, state: State::Follower },
            inbox: it.inbox,
        }
    }
}

impl<IO: RaftIO> From<Raft<Candidate, IO>> for Raft<Leader, IO> {
    fn from(it: Raft<Candidate, IO>) -> Raft<Leader, IO> {
        Raft {
            id: it.id,
            io: it.io,
            state: Leader { state: State::Leader },
            inbox: it.inbox,
        }
    }
}

impl<IO: RaftIO> From<Raft<Leader, IO>> for Raft<Follower, IO> {
    fn from(it: Raft<Leader, IO>) -> Raft<Follower, IO> {
        Raft {
            id: it.id,
            io: it.io,
            state: Follower { leader_id: None, state: State::Follower },
            inbox: it.inbox,
        }
    }
}
