use std::os::unix::net::UnixStream;
use std::path::Path;
use raft::message::*;
use std::io::prelude::*;
use std::convert::TryFrom;
use std::collections::HashMap;

pub fn call(id: u8, message: &[u8]) {}

#[derive(Debug)]
pub struct Rpc {
    sockets: HashMap<u8, UnixStream>
}

impl Rpc {
    pub fn new() -> Rpc {
        Rpc {
            sockets: HashMap::new(),
        }
    }

    pub fn read_message(buf: [u8; 8]) -> Message {
        match Message::try_from(buf) {
            Err(e) => panic!("!"),
            Ok(message) => message
        }
    }

    pub fn connect(&mut self, id: u8) {
        let path_name = format!("/tmp/raft.{}.sock", id);
        let path = Path::new(&path_name);

        match UnixStream::connect(path) {
            Err(e) => panic!("server is not running"),
            Ok(socket) => self.sockets.insert(id,socket),
        };
    }

    pub fn call_one(&mut self, id: u8, message_buf: [u8; 8]) {
            match self.sockets.get_mut(&id) {
                Some(socket) => {
                    socket.write_all(&message_buf);
                },
                None => {}
            }
    }
}