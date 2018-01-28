use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use raft::message::*;
use std::thread;
use std::io::prelude::*;
use std::fs;
use std::convert::TryFrom;

fn handle_client(mut stream: UnixStream) {
    let mut buf: [u8; 8] = [0; 8];

    loop {
        match stream.read(&mut buf) {
            Ok(size) => if size == 0 { break },
            Err(e) => panic!(e),
        };
        println!("{:?}", Message::try_from(buf));
    }
}

#[derive(Debug)]
pub struct Server {
    id: u8,
}

impl Server {
    pub fn new(id: u8) -> Server {
        Server {
            id
        }
    }

    pub fn listen(&self) {
        println!("listening");
        let id = self.id;
        thread::spawn(move || {
            let path_name = format!("/tmp/raft.{}.sock", id);
            let path = Path::new(&path_name);

            if path.exists() {
                fs::remove_file(path).unwrap();
            }

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
}