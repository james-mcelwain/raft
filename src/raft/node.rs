use raft::Index;
use raft::NodeId;

bitflags! {
    pub struct Flags: u8 {
        const VOTED_FOR_ME = 0x01;
        const VOTING = 0x02;
        const HAS_SUFFICIENT_LOG = 0x03;
        const INACTIVE = 0x04;
        const VOTING_COMMITTED = 0x05;
        const ADDITION_COMMITTED = 0x06;
   }
}

pub struct Node {
    pub id: NodeId,
    pub flags: Flags,
    pub next_idx: Index,
    pub match_idx: Index,
}


impl Node {
    pub fn is_active(&self) -> bool {
        !self.flags.contains(Flags::INACTIVE)
    }

    pub fn is_voting(&self) -> bool {
        self.flags.contains(Flags::VOTING)
    }
}

impl Node {
    fn new(id: NodeId) -> Node {
        Node { id, flags: Flags::VOTING, next_idx: 1, match_idx: 0 }
    }
}
