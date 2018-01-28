#![feature(try_from)]
mod raft;
use raft::core::Raft;
use raft::rpc::RpcSender;
use std::thread;
use std::time;
use raft::timer::Timer;

fn main() {

    thread::spawn(|| {
        let mut raft = Raft::new(1);
        raft.server.listen()
    });

    thread::spawn(|| {
        let mut raft = Raft::new(2);
        thread::sleep(std::time::Duration::from_millis(1000));
        raft.rpc.connect(1);
        raft.rpc.request_vote(1, 0, 0)
//        raft.rpc.call_one(1, [0x00, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
//        raft.rpc.call_one(1, [0x01, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
//        raft.rpc.disconnect(1);
//        raft.rpc.call_one(1, [0x01, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    });

    let timer = Timer::new(100, || {
        println!("done!")
    });

    let t= timer.start();
    t.cancel();

    thread::sleep(std::time::Duration::from_millis(10000));
}
