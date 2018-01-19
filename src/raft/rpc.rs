extern crate byteorder;

use self::byteorder::*;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use raft::raft::RaftId;
use raft::message::*;
use std::thread;
use std::io::prelude::*;
use std::fs;
use std::convert::TryFrom;

pub fn call(id: RaftId, message: Vec<u8>) {}

pub fn read_message(buf: &[u8; 8]) -> Message {
    let message_type = match MessageType::try_from(buf.first().unwrap().to_owned()) {
        Ok(T) => match T {
            Vote => Vote
        }
        Err(e) => panic!(e)
    };


    let size = byteorder::BigEndian::read_u32(&buf[1..5].to_owned()).to_owned();

    println!("{:b}", size);
    Message {
        message_type,
        size
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