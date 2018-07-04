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

    pub fn next_idx(&self) -> &Index {
        &self.next_idx
    }

    pub fn set_next_idx(&mut self, idx: Index) {
        self.next_idx = idx;
    }

    pub fn match_idx(&self) -> &Index {
        &self.match_idx
    }

    pub fn set_match_idx(&mut self, idx: Index) {
        self.match_idx = idx;
    }
}

impl Node {
    fn new(id: NodeId) -> Node {
        Node { id, flags: Flags::VOTING, next_idx: 1, match_idx: 0 }
    }
}
