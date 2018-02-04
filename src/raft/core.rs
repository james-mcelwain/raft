use std::time::Duration;

use raft::message::Message;
use raft::server::*;
use raft::rpc::*;
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
    state: State,
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
            state: State::Follower(None),
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
        self.state = State::Candidate;
    }

    fn schedule_election_timeout(&mut self) {
        Timer::new(self.config.min_election_timeout, move || {
            self.become_candidate();
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