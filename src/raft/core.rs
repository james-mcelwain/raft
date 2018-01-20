#[derive(Debug)]
pub struct Log {

}


#[derive(Debug)]
pub struct Raft {
    pub term: u32,
    pub id: u8,
    pub log: Vec<Log>
}

impl Raft {
    pub fn new(id: u8) -> Raft {
        Raft {
            id,
            term: 0,
            log: Vec::new(),
        }
    }
}