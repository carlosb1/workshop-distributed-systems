/// Lamport Clock implementation.
use std::cmp;

#[derive(Default, Clone)]
pub struct LamportClock {
    pub latest_time: u64,
}

impl LamportClock {
    pub fn new(latest_time: u64) -> Self {
        LamportClock { latest_time }
    }
    pub fn tick(&mut self, request_time: u64) -> u64 {
        self.latest_time = cmp::max(self.latest_time, request_time);
        self.latest_time += 1;
        return self.latest_time;
    }
}
