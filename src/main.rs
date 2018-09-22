extern crate uuid;

use std::io::{Write};
use uuid::Uuid;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::thread;
use std::fs;
use std::collections::HashMap;

fn main() {
    println!("Start:");
    let mut raft = Raft::new(UnixSocketIO::new(1));
    let mut raft2 = Raft::new(UnixSocketIO::new(2));
    raft.listen();
    raft2.listen();
    println!("{:?}", raft.get_state());
    let mut candidate = Raft::<Candidate, UnixSocketIO>::from(raft);
    candidate.request_vote(2);
    loop {
        
    }
}

type NodeId = u8;

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
}

pub enum Message {
    VoteRequest(VoteRequest),
    VoteResponse(VoteResponse),
}

impl<S: RaftState, IO: RaftIO> Raft<S, IO> {
    pub fn get_state(&self) -> &State {
        self.state.get_state()
    }

    pub fn listen(&mut self) {
        self.io.listen();
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
}

impl UnixSocketIO {
    pub fn new(node_id: NodeId) -> UnixSocketIO {
        UnixSocketIO {
            node_id,
            sockets: HashMap::new(),
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
                        thread::spawn(|| println!("WOW"));
                    }
                    Err(err) => panic!("Error!")
                }
            }
        });
    }

    fn request_vote(&mut self, vote_request: VoteRequest) -> Result<VoteResponse, &str> {
        print!("Requesting vote {}", vote_request.to);
        let mut socket = self.connect(vote_request.to).unwrap();
        socket.write_all(b"wow").unwrap();
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
        }
    }
}

// Raft starts in the Follower state
impl<IO: RaftIO> Raft<Follower, IO> {
    fn new(io: IO) -> Self {
        Raft {
            id: 1,
            io,
            state: Follower { leader_id: None, state: State::Follower },
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
        }
    }
}

impl<IO: RaftIO> From<Raft<Candidate, IO>> for Raft<Follower, IO> {
    fn from(it: Raft<Candidate, IO>) -> Raft<Follower, IO> {
        Raft {
            id: it.id,
            io: it.io,
            state: Follower { leader_id: None, state: State::Follower },
        }
    }
}

impl<IO: RaftIO> From<Raft<Candidate, IO>> for Raft<Leader, IO> {
    fn from(val: Raft<Candidate, IO>) -> Raft<Leader, IO> {
        Raft {
            id: val.id,
            io: val.io,
            state: Leader { state: State::Leader },
        }
    }
}

impl<IO: RaftIO> From<Raft<Leader, IO>> for Raft<Follower, IO> {
    fn from(it: Raft<Leader, IO>) -> Raft<Follower, IO> {
        Raft {
            id: it.id,
            io: it.io,
            state: Follower { leader_id: None, state: State::Follower },
        }
    }
}
