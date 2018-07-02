use raft::Term;
use raft::EntryId;
use raft::Index;
use raft::NodeId;
use raft::Entry;

/// Indicates if an entry was committed
struct EntryResponse {
    term: Term,
    entry_id: EntryId,
    idx: Index,
}

struct VoteRequest {
    term: Term,
    node_id: NodeId,
    last_idx: Index,
    last_term: Term,
}

struct VoteResponse {
    term: Term,
    granted: bool,
}

struct AppendEntriesRequest {
    term: Term,
    previous_idx: Index,
    previous_term: Term,
    leader_commit: Index,
    entry_count: u8,
    entries: Vec<Entry>,
}

struct AppendEntriesResponse {
    term: Term,
    committed: bool,
    current_idx: Index,
    first_id: Index,
}