use std::time::Duration;
use std::sync::{Arc, Mutex};

use raft::message::Message;
use raft::server::Server;
use raft::server::unix::UnixSocketServer;
use raft::rpc::RpcClient;
use raft::rpc::unix::UnixSocketRpc;
use raft::timer::Timer;

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
pub struct Inner {
    state: State,
    term: u32,
    id: u8,
    log: Vec<Log>,
    timeout: u64,
    server: UnixSocketServer,
    pub rpc: UnixSocketRpc,
}

impl Inner {
    pub fn new(id: u8) -> Inner {
        Inner {
            state: State::Follower(None),
            id,
            term: 0,
            log: Vec::new(),
            timeout: 750,
            server: UnixSocketServer::new(id),
            rpc: UnixSocketRpc::new(),
        }
    }

    pub fn init(&mut self) {
        self.server.listen();
    }
}

#[derive(Debug)]
pub struct Raft {
    pub inner: Arc<Mutex<Inner>>,
    config: RaftConfig,
}

impl Raft {
    pub fn new(id: u8, raft_config: Option<RaftConfig>) -> Raft {
        Raft {
            inner: Arc::new(Mutex::new(Inner::new(id))),
            config: match raft_config {
                Some(config) => config,
                None => RaftConfig::default(),
            },
        }
    }

    pub fn init(&mut self) {
        self.inner.lock().unwrap().init();
    }

    fn schedule_election_timeout(&mut self) {
        Timer::new(self.config.min_election_timeout, self.inner.clone(), |inner| {
            println!("i am seeking election");
            let mut inner = inner.lock().unwrap();
            inner.state = State::Candidate;
        }).start();
    }

    pub fn heartbeat(&mut self) {
        let inner = self.inner.lock().unwrap();
        match inner.state {
            State::Candidate => println!("i am candidate"),
            State::Follower(leader_id) => {
                match leader_id {
                    Some(id) => println!("i am following {}", id),
                    None => {
                        println!("i am looking for a leader");
//                        self.schedule_election_timeout();
                    }
                }
            }
            State::Leader => println!("i am the leader")
        };
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
        };
    }
}