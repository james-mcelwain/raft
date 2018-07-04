extern crate rand;

use raft::node::Flags;
use raft::Term;
use raft::NodeId;
use raft::Entry;
use raft::Index;
use raft::State;
use raft::node::Node;
use raft::Timeout;
use raft::MembershipEvent;
use raft::RaftErr;
use raft::message::VoteRequest;
use raft::message::AppendEntriesRequest;


pub struct Server<T: ServerIO> {
    // Public State

    pub id: NodeId,

    // Persistent State

    current_term: Term,
    voted_for: Option<NodeId>,
    log: Vec<Entry>,

    // Volatile State

    commit_idx: Index,
    last_applied_idx: Index,

    state: State,

    timeout_elapsed: Timeout,

    nodes: Vec<Node>,

    election_timeout: Timeout,
    election_timeout_rand: Timeout,
    request_timeout: Timeout,

    current_leader: Option<NodeId>,

    connected: bool,
    snapshot_in_progress: bool,

    snapshot_last_idx: Index,
    snapshot_last_term: Term,
    voting_cfg_change_log_idx: Option<Index>,

    // IO Impl
    io: T,
}

pub trait ServerIO {
    fn send_vote_request(self, node: NodeId, msg: VoteRequest) -> Result<(), RaftErr>;
    fn send_append_entries_request(self, node: NodeId, msg: AppendEntriesRequest) -> Result<(), RaftErr>;
    fn send_snapshot(self, node: NodeId) -> Result<(), RaftErr>;

    fn node_has_sufficient_logs(self, node: NodeId) -> Result<(), RaftErr>;

    fn persist_vote(&mut self, vote: NodeId) -> Result<(), RaftErr>;
    fn persist_term(&mut self, term: Term, vote: Option<NodeId>) -> Result<(), RaftErr>;

    fn entry_event(self, entry: Entry, idx: Index) -> Result<(), RaftErr>;
    fn membership_event(self, membership: MembershipEvent) -> Result<(), RaftErr>;
}

impl <T: ServerIO> Server<T> {
    pub fn new(io: T) -> Server<T> {
        Server {
            id: 1,
            current_term: 0,
            voted_for: None,
            log: Vec::new(),
            commit_idx: 0,
            last_applied_idx: 0,
            state: State::Follower,
            timeout_elapsed: 0,
            nodes: Vec::new(),
            election_timeout: 0,
            election_timeout_rand: 0,
            request_timeout: 0,
            current_leader: None,
            connected: false,
            snapshot_in_progress: false,
            snapshot_last_idx: 0,
            snapshot_last_term: 0,
            voting_cfg_change_log_idx: None,
            io,
        }
    }

    // election

    fn start_election(&mut self) -> Result<(), RaftErr> {
        println!("Election starting: {} {}, term: {} current_idx: {}", self.election_timeout, self.election_timeout_rand, self.current_term, self.current_idx());
        self.become_candidate();

        Ok(())
    }

    fn vote(&mut self, node_id: NodeId) -> Result<(), RaftErr> {
        self.io.persist_vote(node_id)?;
        self.voted_for = Some(node_id);
        Ok(())
    }

    fn current_idx(&self) -> Index {
        0
    }

    fn become_candidate(&mut self) -> Result<(), RaftErr> {
        println!("I am becoming a candidate");
        let new_term = self.current_term + 1;
        self.set_current_term(new_term)?;

        self.nodes.iter_mut()
            .for_each(|x| x.flags.remove(Flags::VOTED_FOR_ME));

        let own_id = self.id;
        self.vote(own_id);
        self.current_leader = None;
        self.set_state(State::Candidate);

        self.randomize_election_timeout();
        self.timeout_elapsed = 0;

        self.nodes.iter()
            .filter(|x| !x.is_voting())
            .filter(|x| !x.is_active())
            .for_each(|x| self.send_vote_request(x));

        Ok(())
    }

    fn become_leader(&mut self) -> Result<(), RaftErr> {
        println!("I am becoming the leader");
        self.set_state(State::Leader);
        self.timeout_elapsed = 0;

        let current_idx = self.current_idx();
        self.nodes.iter_mut()
            .filter(|x| !x.is_active())
            .for_each(|x| {
                x.set_next_idx(current_idx + 1);
                x.set_match_idx(0);
            });

        self.nodes.iter()
            .for_each(|x| self.send_append_entries_request(x));

        Ok(())
    }

    fn become_follower(&mut self) -> Result<(), RaftErr> {
        println!("I am becoming a follower");

        self.set_state(State::Follower);
        self.randomize_election_timeout();
        self.timeout_elapsed = 0;

        Ok(())
    }

    fn is_single_node(&self) -> bool {
        self.nodes.iter()
            .filter(|x| !x.is_voting())
            .count() == 1
    }

    fn node(&self) -> &Node {
        self.nodes.iter()
            .find(|x| x.id == self.id)
            .unwrap()
    }

    fn get_entry(&self, idx: Index) -> Option<&Entry> {
        self.log.get(idx)
    }

    fn is_leader(&self) -> bool {
        self.state() == &State::Leader
    }

    fn is_snapshot_in_progress(&self) -> bool {
        self.snapshot_in_progress
    }

    fn heartbeat(&mut self, msec_since_last_heartbeat: u64) -> Result<(), RaftErr> {
        self.timeout_elapsed = self.timeout_elapsed + msec_since_last_heartbeat;

        if self.is_single_node() && self.node().is_voting() && !self.is_leader() {
            self.become_leader();
        }

        if self.is_leader() {
            if self.request_timeout <= self.timeout_elapsed {
                self.broadcast_append_entries_request();
            }
        } else if self.election_timeout_rand <= self.timeout_elapsed && !self.is_snapshot_in_progress() {
            if !self.is_single_node() && self.node().is_voting() {
                self.start_election()?;
            }
        }

        if self.last_applied_idx < self.commit_idx() && !self.is_snapshot_in_progress() {
            self.apply_all()?;
        }

        Ok(())
    }

    fn apply_all(&mut self) -> Result<(), RaftErr> {
        Ok(())
    }

    fn set_current_term(&mut self, term: Term)  -> Result<(), RaftErr> {
        if self.current_term < term {
            self.io.persist_term(term, None)?;
            self.current_term = term;
            self.voted_for = None;
        }

        Ok(())
    }

    fn randomize_election_timeout(&mut self) {
        let jitter = rand::random::<u64>();
        self.election_timeout_rand = self.election_timeout + jitter % self.election_timeout;
    }

    fn set_state(&mut self, state: State) {
        self.state = state;
    }

    fn state(&self) -> &State {
        &self.state
    }


    // messages

    fn send_vote_request(&self, node: &Node) {

    }

    fn send_append_entries_request(&self, node: &Node) {

    }

    fn broadcast_append_entries_request(&mut self) {

    }

    // log

    fn apply_entry(self) {

    }

    fn append_entry(self, entry: Entry) {

    }

    fn set_last_applied_idx(self, idx: Index) {

    }

    fn commit_idx(&self) -> Index {
        0
    }

    fn delete_entry_from_idx(&mut self, idx: Index) -> Entry {
        assert!(self.commit_idx() < idx);

        if let Some(i) = self.voting_cfg_change_log_idx {
            if idx <= i {
                self.voting_cfg_change_log_idx = None;
            }
        }

       self.log.remove(idx)
    }
}

struct NoopServer {}

impl super::server::ServerIO for NoopServer {
    fn send_vote_request(self, node: u8, msg: VoteRequest) -> Result<(), RaftErr> {
        Ok(())
    }

    fn send_append_entries_request(self, node: u8, msg: AppendEntriesRequest) -> Result<(), RaftErr> {
        Ok(())
    }

    fn send_snapshot(self, node: u8) -> Result<(), RaftErr> {
        Ok(())
    }

    fn node_has_sufficient_logs(self, node: u8) -> Result<(), RaftErr> {
        Ok(())
    }

    fn persist_vote(&mut self, vote: u8) -> Result<(), RaftErr> {
        Ok(())
    }

    fn persist_term(&mut self, term: u8, vote: Option<u8>) -> Result<(), RaftErr> {
        Ok(())
    }

    fn entry_event(self, entry: Entry, idx: usize) -> Result<(), RaftErr> {
        Ok(())
    }

    fn membership_event(self, membership: MembershipEvent) -> Result<(), RaftErr> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new() {
        let server = super::Server::new(super::NoopServer {});
    }

    #[test]
    fn set_state() {
        let mut server = super::Server::new(super::NoopServer {});
        server.set_state(super::State::Leader);
        assert_eq!(&super::State::Leader, server.state());
    }
}