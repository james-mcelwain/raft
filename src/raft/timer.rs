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
    pub fn new(time_in_ms: u64, handler: fn() -> ()) -> Timer {
        Timer {
            timeout: Duration::from_millis(time_in_ms),
            state: Arc::new(Mutex::new(TimerState { cancelled: false, reason: None })),
            callback: handler
        }
    }

    pub fn start(&self) -> CancellationToken {
        let timeout = self.timeout.clone();
        let state = Arc::clone(&self.state);
        let handler = self.callback.clone();
        thread::spawn(move || {
            thread::sleep(timeout);
            let c = state.lock().unwrap().cancelled;
            if !c {
                handler()
            }
        });

        CancellationToken { state: Arc::clone(&self.state) }
    }

    pub fn cancel(&self) {
        let state = self.state.clone();
        state.lock().unwrap().cancel(CancellationReason::Unknown);
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
