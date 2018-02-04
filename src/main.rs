#![feature(try_from)]

mod raft;

use raft::core::Raft;
use raft::rpc::RpcSender;
use std::thread;
use raft::timer::{Timer, CancellationReason};

fn main() {
    thread::spawn(|| {
        let mut raft = Raft::new(1, None);
        raft.init()
    });

    thread::spawn(|| {
        let mut raft = Raft::new(2, None);
        thread::sleep(std::time::Duration::from_millis(1000));
        raft.rpc.connect(1).unwrap();
        raft.rpc.request_vote(1, 0, 0).unwrap();
    });

    thread::sleep(std::time::Duration::from_millis(10000));
}
