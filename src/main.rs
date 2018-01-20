#![feature(try_from)]
mod raft;
use raft::rpc::*;
use raft::server::listen;
use std::thread;
use std::time;

fn main() {
    let message = read_message([0x05, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00]);

    print!("message: {:?}", message);
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
