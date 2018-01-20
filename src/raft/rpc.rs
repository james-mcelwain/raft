use std::os::unix::net::UnixStream;
use std::path::Path;
use raft::message::*;
use std::io::prelude::*;
use std::convert::TryFrom;

pub fn call(id: u8, message: &[u8]) {}

pub fn read_message(buf: [u8; 8]) -> Message {
    match Message::try_from(buf) {
        Err(e) => panic!("!"),
        Ok(message) => message
    }
}

pub fn connect(id: u8) {
    let path_name = format!("/tmp/raft.{}.sock", id);
    let path = Path::new(&path_name);

    let mut stream = match UnixStream::connect(path) {
        Err(e) => panic!("server is not running"),
        Ok(stream) => stream,
    };

    // Send message

    if let Err(e) = stream.write(&[0xff]) {
        panic!("couldn't write message");
    }
}