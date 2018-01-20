use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use raft::core::RaftId;
use raft::message::*;
use std::thread;
use std::io::prelude::*;
use std::fs;
use std::convert::TryFrom;

pub fn call(id: RaftId, message: Vec<u8>) {}

pub fn read_message(buf: [u8; 8]) -> Message {
    match Message::try_from(buf) {
        Err(E) => panic!("!"),
        Ok(Message) => Message
    }
}

pub fn connect(id: RaftId) {
    let path_name = format!("/tmp/raft.{}.sock", id);
    let path = Path::new(&path_name);

    let mut stream = match UnixStream::connect(path) {
        Err(_) => panic!("server is not running"),
        Ok(stream) => stream,
    };

    // Send message
    match stream.write(&[0xff]) {
        Err(_) => panic!("couldn't send message"),
        Ok(_) => {}
    }
}