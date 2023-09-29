//! A Network client implementation for message-io.

use crate::adapters::lamport::LamportClock;
use crate::adapters::network::message::FromClientMessage;
use crate::adapters::network::net::run_client;
use crate::core::DistributedClientDispatcher;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// Client implementation with its internal version clock (Lamport Clock).
pub struct Client {
    clock: LamportClock,
    dispatcher: DistributedClientDispatcher,
}

impl Client {
    /// Constructor function with initial clock
    pub fn new(latest_time: u64) -> Self {
        Client {
            clock: LamportClock::new(latest_time),
            dispatcher: DistributedClientDispatcher::new(),
        }
    }

    /// Exposed write function
    pub fn write(self, key: String, value: String) -> JoinHandle<()> {
        let message = FromClientMessage::Write(key, value, self.clock.latest_time);
        let cloned_dispatcher = Arc::new(Mutex::new(self.dispatcher));
        thread::spawn(move || {
            run_client(message, cloned_dispatcher);
        })
    }
}
