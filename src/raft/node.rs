#[macro_use]
extern crate bitflags;

use raft::Index;
use raft::NodeId;

bitflags! {
    pub flags Flags: u8 {
        NODE_VOTED_FOR_ME = 0x00,
        NODE_VOTING = 0x01,
        NODE_HAS_SUFFICIENT_LOG = 0x02,
        NODE_INACTIVE = 0x03,
        NODE_VOTING_COMMITTED = 0x04,
        NODE_ADDITION_COMMITTED = 0x05,
   }
}

pub struct Node {
    pub id: NodeId,
    pub flags: Flags,
    pub next_idx: Index,
    pub match_idx: Index,
}

impl Node {
    fn new(id: NodeId) -> Node {
        Node { id, flags: Flags::NODE_VOTING, next_idx: 1, match_idx: 0 }
    }
}
