#![feature(try_from)]
mod raft;
use raft::rpc::Rpc;
use raft::core::Raft;
use std::thread;
use std::time;

fn main() {

    thread::spawn(|| {
        let mut raft = Raft::new(1);
        raft.server.listen()
    });

    thread::spawn(|| {
        let mut raft = Raft::new(2);
        thread::sleep(std::time::Duration::from_millis(1000));
        raft.rpc.connect(1);
        raft.rpc.call_one(1, [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
    });

    loop {}
}
