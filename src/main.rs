#![feature(try_from)]

mod raft;

use raft::core::Raft;
use raft::rpc::RpcSender;
use std::thread;
use raft::timer::{Timer, CancellationReason};

fn main() {
    thread::spawn(|| {
        let mut raft = Raft::new(1);
        raft.server.listen()
    });

    thread::spawn(|| {
        let mut raft = Raft::new(2);
        thread::sleep(std::time::Duration::from_millis(1000));
        raft.rpc.connect(1).unwrap();
        raft.rpc.request_vote(1, 0, 0).unwrap();
    });

    let timer = Timer::new(100, || {
        println!("done!")
    });

    let t = timer.start();
    t.cancel(CancellationReason::Unknown);

    thread::sleep(std::time::Duration::from_millis(10000));
}
