#![feature(try_from)]
mod raft;
use raft::raft::*;
use raft::rpc::*;
use raft::server::listen;
use std::thread;
use std::time;

fn main() {
    let message = read_message(&[0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    print!("{:?}", message);
//    thread::spawn(|| {
//        listen(0xff);
//    });
//
//    thread::sleep(time::Duration::from_millis(1000));
//    thread::spawn(|| {
//        connect(0xff);
//    });
//
//    loop {}
}
