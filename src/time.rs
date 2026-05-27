use std::thread;
use std::time::{Duration, Instant, SystemTime};

pub fn now() -> SystemTime {
    SystemTime::now()
}

pub fn elapsed_init() -> Instant {
    Instant::now()
}

pub fn elapsed_final() -> Instant {
    Instant::now()
}

pub fn delay(duration: u64) {
    thread::sleep(Duration::from_millis(duration));
}

pub fn elapsed_time(init: Instant, final_: Instant) -> Duration {
    final_.duration_since(init)
}
