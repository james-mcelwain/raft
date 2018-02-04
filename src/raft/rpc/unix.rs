use std::os::unix::net::UnixStream;
use std::path::Path;
use std::io::prelude::*;
use std::convert::{TryFrom, TryInto};
use std::collections::HashMap;

use raft::rpc::{RpcClient, RpcSender, RpcHandler, RpcError};
use raft::message::*;

// impl: unix stream
#[derive(Debug)]
pub struct UnixSocketRpc {
    sockets: HashMap<u8, UnixStream>
}

impl UnixSocketRpc {
    fn validate_message(buf: [u8; 8]) -> Result<(), RpcError> {
        match Message::try_from(buf) {
            Ok(_) => Ok(()),
            Err(_) => return Err(RpcError::InvalidMessage(buf))
        }
    }


    fn broadcast(&mut self, message: Message) {
        for mut socket in self.sockets.values() {
            let buf: [u8; 8] = message.try_into().expect("invalid message");
            match socket.write_all(&buf) {
                Ok(_) => {}
                Err(e) => panic!(e)
            };
        }
    }

    pub fn connect(&mut self, id: u8) -> Result<&UnixStream, RpcError> {
        if self.sockets.contains_key(&id) {
            return self.sockets.get(&id).ok_or(RpcError::UnknownHost(id));
        }

        let path_name = format!("/tmp/raft.{}.sock", id);
        let path = Path::new(&path_name);

        match UnixStream::connect(path) {
            Err(_) => Err(RpcError::UnknownHost(id)),
            Ok(socket) => {
                self.sockets.insert(id, socket);
                self.sockets.get(&id).ok_or(RpcError::UnknownHost(id))
            }
        }
    }

    fn send_message_buf(&mut self, id: u8, buf: &[u8; 8]) -> Result<(), RpcError> {
        match self.sockets.get(&id) {
            Some(mut socket) => {
                match socket.write_all(buf) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(RpcError::TransportError(e))
                }
            }
            None => Err(RpcError::UnknownHost(id))
        }
    }
}


impl RpcClient for UnixSocketRpc {
    fn new() -> UnixSocketRpc {
        return UnixSocketRpc {
            sockets: HashMap::new()
        };
    }

    fn try_connect(&mut self, id: u8) -> Result<&UnixStream, RpcError> {
        self.connect(id)
    }
}

impl RpcSender for UnixSocketRpc {
    fn request_vote(&mut self, id: u8, last_log_idx: u64, last_log_term: u64) -> Result<(), RpcError> {
        let buf = [MessageType::Vote.into(), 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        UnixSocketRpc::validate_message(buf)?;
        self.send_message_buf(id, &buf)
    }

    fn request_vote_reply(&mut self, id: u8, term: u64, vote_granted: bool) -> Result<(), RpcError> {
        let buf = [MessageType::VoteReply.into(), 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        UnixSocketRpc::validate_message(buf)?;
        self.send_message_buf(id, &buf)
    }

    fn append_entries(&mut self, id: u8, term: u64, prev_log_idx: u64, prev_log_term: u64, logs: Vec<u8>) -> Result<(), RpcError> {
        let buf = [MessageType::Append.into(), 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        UnixSocketRpc::validate_message(buf)?;
        self.send_message_buf(id, &buf)
    }

    fn append_entries_reply(&mut self, id: u8, prev_log_idx: u64, entry_count: u64, applied: bool) -> Result<(), RpcError> {
        let buf = [MessageType::AppendReply.into(), 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        UnixSocketRpc::validate_message(buf)?;
        self.send_message_buf(id, &buf)
    }
}
