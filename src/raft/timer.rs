use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread;

/// raft.Timer
///
/// A simple timer.
///
///

#[derive(Debug)]
pub struct Timer {
    timeout: Duration,
    cancelled: Arc<Mutex<bool>>,
    callback: fn() -> (),
}


pub struct CancellationToken {
    cancelled: Arc<Mutex<bool>>,
}

impl CancellationToken {
    pub fn cancel(&self) {
        let cancelled = self.cancelled.clone();
        let mut c = cancelled.lock().unwrap();
        *c = true;
    }
}

impl Timer {
    pub fn new(time_in_ms: u64, handler: fn() -> ()) -> Timer {
        Timer {
            timeout: Duration::from_millis(time_in_ms),
            cancelled: Arc::new(Mutex::new(false)),
            callback: handler
        }
    }

    pub fn start(&self) -> CancellationToken {
        let timeout = self.timeout.clone();
        let cancelled = self.cancelled.clone();
        let handler = self.callback.clone();
        thread::spawn(move || {
            thread::sleep(timeout);
            let c = *cancelled.lock().unwrap();
            if !c {
                handler()
            }
        });

        CancellationToken { cancelled: self.cancelled.clone() }
    }

    pub fn cancel(&self) {
        let cancelled = self.cancelled.clone();
        let mut c = cancelled.lock().unwrap();
        *c = true;
    }
}
