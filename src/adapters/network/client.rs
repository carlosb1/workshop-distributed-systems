use crate::adapters::lamport::LamportClock;
use crate::adapters::network::message::FromClientMessage;
use crate::adapters::network::net::run_client;

pub struct Client {
    clock: LamportClock,
}

impl Client {
    pub fn new(latest_time: u64) -> Self {
        Client {
            clock: LamportClock::new(latest_time),
        }
    }
    pub fn write(self, key: String, value: String) {
        let message = FromClientMessage::Write(key, value, self.clock.latest_time);
        run_client(message);
    }
}
