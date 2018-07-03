pub mod log;
pub mod server;
pub mod node;
pub mod config;
pub mod message;

/// Errors returned by actions taken by a node
pub enum RaftErr {
    NotLeader,
    OneVotingChangeOnly,
    Shutdown,
    OOM,
    NeedsSnapshot,
    SnapshotInProgress,
    SnapshotAlreadyLoaded,
}

/// The state of a node
pub enum State {
    // Initial state of a node, prior to joining the cluster
    None,
    // The node is following a leader
    Follower,
    // The node is seeking election to become the leader
    Candidate,
    // Consensus has been reached that this node is the leader
    Leader,
}

/// Messages sent during a heartbeat
pub enum LogType {
    Normal,
    AddNonvotingNode,
    AddNode,
    DemoteNode,
    RemoveNode,
}

///
pub enum MembershipEvent {
    ADD(NodeId),
    REMOVE(NodeId)
}

type EntryId = u8;
type Term = u8;
type Index = usize;
type NodeId = u8;
type Timeout = u64;

struct Entry {
    term: Term,
    entry_id: EntryId,
    log_type: LogType,
    data: Vec<u8>,
}

