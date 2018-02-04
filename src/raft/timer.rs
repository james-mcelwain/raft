use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread;

/// raft.Timer
///
/// A simple timer that executes a callback after it expires
/// and can be cancelled.
#[derive(Debug)]
pub enum CancellationReason {
    Unknown
}

#[derive(Debug)]
pub struct Timer {
    timeout: Duration,
    state: Arc<Mutex<TimerState>>,
    callback: fn() -> (),
}

impl Timer {
    pub fn new(timeout: Duration, handler: fn() -> ()) -> Timer {
        Timer {
            timeout,
            state: Arc::new(Mutex::new(TimerState { cancelled: false, reason: None })),
            callback: handler,
        }
    }

    pub fn start(&self) -> CancellationToken {
        let timeout = self.timeout.clone();
        let state = Arc::clone(&self.state);
        let callback = self.callback.clone();
        thread::spawn(move || {
            thread::sleep(timeout);
            let c = state.lock().unwrap().cancelled;
            if !c {
                callback()
            }
        });

        CancellationToken { state: Arc::clone(&self.state) }
    }

    pub fn cancel(&self) {
        &self.state.lock().unwrap().cancel(CancellationReason::Unknown);
    }
}

#[derive(Debug)]
pub struct TimerState {
    cancelled: bool,
    reason: Option<CancellationReason>,
}

impl TimerState {
    pub fn cancel(&mut self, reason: CancellationReason) {
        self.cancelled = true;
        self.reason = Some(reason);
    }
}

#[derive(Debug)]
pub struct CancellationToken {
    state: Arc<Mutex<TimerState>>
}

impl CancellationToken {
    pub fn cancel(self, reason: CancellationReason) {
        self.state.lock().unwrap().cancel(reason);
    }
}
