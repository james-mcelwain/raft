use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use raft::raft::RaftId;
use std::thread;
use std::io::prelude::*;
use std::fs;

fn handle_client(mut stream: UnixStream) {
    println!("handle_client");
    let mut buf: [u8; 16] = [0; 16];
    stream.read(&mut buf);
    println!("{:x}", buf[0]);
}

pub fn listen(id: RaftId) {
    thread::spawn(move || {
        let path_name = format!("/tmp/raft.{}.sock", id);
        let path = Path::new(&path_name);

        fs::remove_file(path).unwrap();

        let socket = UnixListener::bind(path).unwrap();

        for stream in socket.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| handle_client(stream));
                }
                Err(err) => {
                    break;
                }
            }
        }
    });
}