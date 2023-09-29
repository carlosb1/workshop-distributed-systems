/// Core domain code. In this case, code for server and client.
use crate::adapters::lamport::LamportClock;
use crate::adapters::network::message::{FromClientMessage, FromServerMessage};
use crate::adapters::network::net::{ClientDispatcher, ServerDispatcher};
use crate::entity::{SerializedValue, VersionedValue};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct DistributedServerDispatcher {
    clock: LamportClock,
    mvvc_store: HashMap<VersionedValue, SerializedValue>,
}

impl DistributedServerDispatcher {
    pub fn new(
        clock: LamportClock,
        mvvc_store: HashMap<VersionedValue, SerializedValue>,
    ) -> DistributedServerDispatcher {
        DistributedServerDispatcher { clock, mvvc_store }
    }

    pub fn write(&mut self, key: String, value: String, request_time: u64) -> u64 {
        log::info!(
            "I received a new entry from a client: {:}={:}, time={:}",
            key,
            value,
            request_time
        );
        let time = self.clock.tick(request_time);
        self.mvvc_store.insert(
            VersionedValue::new(key.as_str(), time),
            SerializedValue::new(value),
        );
        return time;
    }
}

impl ServerDispatcher for DistributedServerDispatcher {
    fn dispatch(&mut self, received: FromClientMessage) -> Option<FromServerMessage> {
        log::info!("I received a message from the client  {:?}", received);
        match received {
            FromClientMessage::Write(key, value, clock) => {
                let time = self.write(key, value, clock);
                Some(FromServerMessage::WrittenAt(time))
            }
            _ => Some(FromServerMessage::UnknownPong),
        }
    }
}

#[derive(Clone, Default)]
pub struct DistributedClientDispatcher {}

impl DistributedClientDispatcher {
    pub fn new() -> Self {
        DistributedClientDispatcher {}
    }
}

impl ClientDispatcher for DistributedClientDispatcher {
    fn dispatch(&mut self, received: FromServerMessage) -> Option<FromClientMessage> {
        log::info!("I received a message from the server  {:?}", received);
        match received {
            FromServerMessage::WrittenAt(clock) => {
                log::info!("Received latest time from server: {:}", clock);
            }
            _ => {
                log::info!("Unknown response");
            }
        }
        None
    }
}
