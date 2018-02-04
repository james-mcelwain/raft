pub mod unix;

use std::os::unix::net::UnixStream;
use std::io::Error;

// interfaces for the main rpc operations in raft
pub trait RpcClient {
    fn new() -> Self;
    fn try_connect(&mut self, id: u8) -> Result<&UnixStream, RpcError>;
}

//    -->
pub trait RpcSender {
    fn request_vote(&mut self, id: u8, last_log_idx: u64, last_log_term: u64) -> Result<(), RpcError>;
    fn request_vote_reply(&mut self, id: u8, term: u64, vote_granted: bool) -> Result<(), RpcError>;
    fn append_entries(&mut self, id: u8, term: u64, prev_log_idx: u64, prev_log_term: u64, logs: Vec<u8>) -> Result<(), RpcError>;
    fn append_entries_reply(&mut self, id: u8, prev_log_idx: u64, entry_count: u64, applied: bool) -> Result<(), RpcError>;
}

//    <--
pub trait RpcHandler {
    fn on_request_vote(&self, id: u8, last_log_idx: u64, last_log_term: u64);
    fn on_request_vote_reply(&self, id: u8, term: u64, vote_granted: bool);
    fn on_append_entries(&self, id: u8, term: u64, prev_log_idx: u64, prev_log_term: u64, logs: Vec<u8>);
    fn on_append_entries_reply(&self, id: u8, prev_log_idx: u64, entry_count: u64, applied: bool);
}

// possible error conditions of protocol
#[derive(Debug)]
pub enum RpcError {
    UnknownHost(u8),
    InvalidMessage([u8; 8]),
    TransportError(Error),
}

