use std::time::Duration;
use std::sync::{Arc,Mutex};

use raft::message::Message;
use raft::server::Server;
use raft::server::unix::UnixSocketServer;
use raft::rpc::RpcClient;
use raft::rpc::unix::UnixSocketRpc;
use raft::timer::{Timer};

#[derive(Debug)]
pub struct Log {
    message: Message,
    data: Vec<u8>,
}

#[derive(Debug)]
pub enum State {
    Follower(Option<u8>),
    Candidate,
    Leader,
}

#[derive(Debug)]
pub struct Raft {
    state: Arc<Mutex<State>>,
    term: u32,
    id: u8,
    log: Vec<Log>,
    timeout: u64,
    server: UnixSocketServer,
    pub rpc: UnixSocketRpc,
    config: RaftConfig,
}

impl Raft {
    pub fn new(id: u8, raft_config: Option<RaftConfig>) -> Raft {
        Raft {
            state: Arc::new(Mutex::new(State::Follower(None))),
            id,
            term: 0,
            log: Vec::new(),
            timeout: 750,
            server: UnixSocketServer::new(id),
            rpc: UnixSocketRpc::new(),
            config: match raft_config {
                Some(config) => config,
                None => RaftConfig::default(),
            },
        }
    }

    pub fn init(&mut self) {
        self.server.listen();
        self.schedule_election_timeout()
    }

    pub fn become_candidate(&mut self) {
        println!("[{}] becoming candidate", self.id);
        let mut s = self.state.lock().unwrap();
        *s = State::Candidate;
    }

    fn schedule_election_timeout(&mut self) {
        Timer::new(self.config.min_election_timeout, self.state.clone(), |data| {
            let mut state = data.lock().unwrap();
            *state = State::Candidate;
        }).start();
    }
}

#[derive(Debug)]
pub struct RaftConfig {
    connection_timeout: Duration,
    min_election_timeout: Duration,
    heartbeat_interval: Duration,
}

impl RaftConfig {
    fn default() -> RaftConfig {
        return RaftConfig {
            connection_timeout: Duration::from_millis(10000),
            min_election_timeout: Duration::from_millis(1000),
            heartbeat_interval: Duration::from_millis(1000),
        }
    }
}