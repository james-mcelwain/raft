extern crate uuid;
use uuid::Uuid;

fn main() {
    println!("Start:");
    let mut raft = Raft::new(NoopIO::new());
    raft.listen();
    println!("{:?}", raft.get_state());
    let candidate = Raft::<Candidate, NoopIO>::from(raft);
    candidate.request_vote(2);
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
}

pub struct Raft<S: RaftState, IO: RaftIO> {
    id: NodeId,
    state: S,
    io: IO,
}

impl <S: RaftState, IO: RaftIO> Raft<S, IO> {
    pub fn get_state(&self) -> &State {
        self.state.get_state()
    }

    pub fn listen(&mut self){
        self.io.listen();
    }
}

pub trait RaftIO {
    fn listen(&mut self);
    fn request_vote(&self, vote_request: VoteRequest) -> Result<VoteResponse, &str> ;
}

struct NoopIO {

}

impl NoopIO {
    pub fn new() -> NoopIO {
        NoopIO {

        }
    }
}

impl RaftIO for NoopIO {
    fn listen(&mut self) {
        println!("Listening...");
    }

    fn request_vote(&self, vote_request: VoteRequest) -> Result<VoteResponse, &str> {
        println!("Requesting vote from {:?}", vote_request.to);
        Ok(VoteResponse::new(vote_request, true))
    }
}

// The three cluster states a Raft node can be in

// If the node is the Leader of the cluster services requests and replicates its state.
struct Leader  {
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
}

// If it is a Candidate it is attempting to become a leader due to timeout or initialization.
struct Candidate {
    state: State,
}

impl RaftState for Candidate {
    fn get_state(&self) -> &State {
        &self.state
    }
}

// Otherwise the node is a follower and is replicating state it receives.
struct Follower {
    leader_id: Option<NodeId>,
    state: State,
}

impl RaftState for Follower {
        fn get_state(&self) -> &State {
        &self.state
    }
}

// Raft starts in the Follower state
impl <IO: RaftIO> Raft<Follower, IO> {
    fn new(io: IO) -> Self {
        Raft {
            id: 1,
            io,
            state: Follower { leader_id: None, state: State::Follower }
        }
    }
}

impl <IO: RaftIO> Raft<Candidate, IO> {
    fn request_vote(&self, node_id: NodeId) -> Result<VoteResponse, &str> {
        self.io.request_vote(VoteRequest::new(self.id, node_id))
    }
}

// The following are the defined transitions between states.

// When a follower timeout triggers it begins to campaign
impl <IO: RaftIO> From<Raft<Follower, IO>> for Raft<Candidate, IO> {
    fn from(it: Raft<Follower, IO>) -> Raft<Candidate, IO> {
        // ... Logic prior to transition
        Raft {
            id: it.id,
            io: it.io,
            state: Candidate { state: State::Candidate }
        }
    }
}

// If it doesn't receive a majority of votes it loses and becomes a follower again.
impl <IO: RaftIO> From<Raft<Candidate, IO>> for Raft<Follower, IO> {
    fn from(it: Raft<Candidate, IO>) -> Raft<Follower, IO> {
        // ... Logic prior to transition
        Raft {
            id: it.id,
            io: it.io,
            state: Follower { leader_id: None, state: State::Follower }
        }
    }
}

// If it wins it becomes the leader.
impl <IO: RaftIO> From<Raft<Candidate, IO>> for Raft<Leader, IO> {
    fn from(val: Raft<Candidate, IO>) -> Raft<Leader, IO> {
        // ... Logic prior to transition
        Raft {
            id: val.id,
            io: val.io,
            state: Leader { state: State::Leader }
        }
    }
}

// If the leader becomes disconnected it may rejoin to discover it is no longer leader
impl <IO: RaftIO> From<Raft<Leader, IO>> for Raft<Follower, IO> {
    fn from(it: Raft<Leader, IO>) -> Raft<Follower, IO> {
        Raft {
            id: it.id,
            io: it.io,
            state: Follower { leader_id: None, state: State::Follower }
        }
    }
}