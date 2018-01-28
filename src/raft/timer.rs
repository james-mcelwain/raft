use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct Timer {
    timeout: Duration,
    pub cancelled: Arc<Mutex<bool>>,
    handler: fn() -> (),
}

impl Timer {
    pub fn new(time_in_ms: u64, handler: fn() -> ()) -> Timer {
        Timer {
            timeout: Duration::from_millis(time_in_ms),
            cancelled: Arc::new(Mutex::new(false)),
            handler
        }
    }

    pub fn start(&self) {
        let timeout = self.timeout.clone();
        let cancelled = self.cancelled.clone();
        let handler = self.handler.clone();
        thread::spawn(move || {
            thread::sleep(timeout);
            let c = *cancelled.lock().unwrap();
            if !c {
                handler()
            }
        });
    }

    pub fn cancel(&self) {
        let cancelled = self.cancelled.clone();
        let mut c = cancelled.lock().unwrap();
        *c = true;
    }
}
