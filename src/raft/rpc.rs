use std::os::unix::net::UnixStream;
use raft::core::Raft;
use std::path::Path;
use raft::message::*;
use std::io::prelude::*;
use std::convert::{TryFrom,TryInto};
use std::collections::HashMap;
use std::net::Shutdown;

pub fn call(id: u8, message: &[u8]) {}

#[derive(Debug)]
pub enum RpcError {
    UnknownHost(u8),
}

#[derive(Debug)]
pub struct UnixSocketRpc {
    sockets: HashMap<u8, UnixStream>
}

impl UnixSocketRpc {
    pub fn new() -> UnixSocketRpc {
        return UnixSocketRpc {
            sockets: HashMap::new()
        };
    }

    fn connect(&mut self, id: u8) -> Result<&UnixStream, RpcError> {
        let path_name = format!("/tmp/raft.{}.sock", id);
        let path = Path::new(&path_name);

        match UnixStream::connect(path) {
            Err(e) => Err(RpcError::UnknownHost(id)),
            Ok(socket) => {
                self.sockets.insert(id, socket);
                self.sockets.get(&id).ok_or(RpcError::UnknownHost(id))
            }
        }
    }

    pub fn try_connect(&mut self, id: u8) {
        if !self.sockets.contains_key(&id) {
            match self.connect(id) {
                Err(e) => println!("could not connect to {:?} {:?}", id, e),
                Ok(socket) => println!("connected to {:?}", id)
            };
        }
    }
}

impl RpcSender for UnixSocketRpc {
    fn request_vote(&mut self, id: u8, last_log_idx: u64, last_log_term: u64) {
        for mut socket in self.sockets.values() {
            let message = Message { message_type: MessageType::Vote, size: 0, flags: 0 };
            let buf: [u8; 8] = message.try_into().unwrap();
            socket.write_all(&buf);
        }
    }

    fn request_vote_reply(&mut self, id: u8, term: u64, vote_granted: bool) {
        unimplemented!()
    }

    fn append_entries(&mut self, id: u8, term: u64, prev_log_idx: u64, prev_log_term: u64, logs: Vec<u8>) {
        unimplemented!()
    }

    fn append_entries_reply(&mut self, id: u8, prev_log_idx: u64, entry_count: u64, applied: bool) {
        unimplemented!()
    }
}

trait RpcSender {
    fn request_vote(&mut self, id: u8, last_log_idx: u64, last_log_term: u64);
    fn request_vote_reply(&mut self, id: u8, term: u64, vote_granted: bool);
    fn append_entries(&mut self, id: u8, term: u64, prev_log_idx: u64, prev_log_term: u64, logs: Vec<u8>);
    fn append_entries_reply(&mut self, id: u8, prev_log_idx: u64, entry_count: u64, applied: bool);
}

trait RpcHandler {
    fn on_request_vote(&self, id: u8, last_log_idx: u64, last_log_term: u64);
    fn on_request_vote_reply(&self, id: u8, term: u64, vote_granted: bool);
    fn on_append_entries(&self, id: u8, term: u64, prev_log_idx: u64, prev_log_term: u64, logs: Vec<u8>);
    fn on_append_entries_reply(&self, id: u8, prev_log_idx: u64, entry_count: u64, applied: bool);
}