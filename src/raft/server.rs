#[macro_use]
extern crate bitflags;

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

struct ServerInternal {

}

pub struct Server {
    // Public State

    pub id: NodeId,


    // IO Impl
    io: ServerIO,

    // Persistent State

    current_term: Term,
    voted_for: NodeId,
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

    // slice?
    current_leader: NodeId,

    connected: bool,
    snapshot_in_progress: bool,

    snapshot_last_idx: Index,
    snapshot_last_term: Term,
    voting_cfg_change_log_idx: Index,
}

trait ServerIO {
    fn send_vote_request(self, node: NodeId, msg: VoteRequest) -> Result<(), RaftErr>;
    fn send_append_entries_request(self, node: NodeId, msg: AppendEntriesRequest) -> Result<(), RaftErr>;
    fn send_snapshot(self, node: NodeId) -> Result<(), RaftErr>;

    fn node_has_sufficient_logs(self, node: NodeId) -> Result<(), RaftErr>;

    fn persist_vote(self, vote: NodeId) -> Result<(), RaftErr>;
    fn persist_term(self, term: Term, vote: NodeId) -> Result<(), RaftErr>;

    fn entry_event(self, entry: Entry, idx: Index) -> Result<(), RaftErr>;
    fn membership_event(self, membership: MembershipEvent) -> Result<(), RaftErr>;
}

impl Server {
    pub fn new() {
        let server: Server();


    }

    // election

    fn election_start(self) {
        println!("Election starting: {} {}, term: {} current_idx: {}", self.election_timeout, self.election_timeout_rand, self.current_term, self.get_current_idx());
        self.become_candidate()
    }

    fn get_current_idx(self) {

    }

    fn become_candidate(self) {
        println!("I am becoming a candidate");
        self.set_current_term(self.current_term + 1)?;
        for node in self.nodes.iter() {
            node.flags.remove(Flags::VOTED_FOR_ME);
        }

    }

    fn set_current_term(self, term: Term)  -> Result<(), RaftErr> {
        if self.current_term < term {
            self.io.persist_term(term, -1)?;
            self.current_term = term;
            self.voted_for = -1;
        }

        Ok(())
    }

    fn randomize_election_timeout(self) {

    }

    fn set_state(self, state: State) {

    }

    fn get_state(self) {

    }


    // messages

    fn send_vote_request(self, node: Node) {

    }

    fn send_append_entries_request(self, node: Node) {

    }

    fn broadcast_append_entries_request(self) {

    }

    // log

    fn apply_entry(self) {

    }

    fn append_entry(self, entry: Entry) {

    }

    fn set_last_applied_idx(self, idx: Index) {

    }

    fn get_commit_idx(self) -> Idx {

    }

    fn delete_entry_from_idx(self, idx: Index) -> Entry {
        assert!(self.get_commit_idx() < idx);

        if idx <= self.voting_cfg_change_log_idx {
            self.voting_cfg_change_log_idx = -1;
        }

       self.log.remove(idx)
    }
}