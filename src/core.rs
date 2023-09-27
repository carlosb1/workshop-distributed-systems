use crate::adapters::lamport::LamportClock;
use crate::adapters::network::message::{FromClientMessage, FromServerMessage};
use crate::adapters::network::server::Dispatcher;
use crate::entity::{SerializedValue, VersionedValue};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct ServerDispatcher {
    clock: LamportClock,
    mvvc_store: HashMap<VersionedValue, SerializedValue>,
}

impl ServerDispatcher {
    pub fn new(
        clock: LamportClock,
        mvvc_store: HashMap<VersionedValue, SerializedValue>,
    ) -> ServerDispatcher {
        ServerDispatcher { clock, mvvc_store }
    }

    pub fn write(&mut self, key: String, value: String, request_time: u64) -> u64 {
        let time = self.clock.tick(request_time);
        self.mvvc_store.insert(
            VersionedValue::new(key.as_str(), time),
            SerializedValue::new(value),
        );
        return time;
    }
}

impl Dispatcher for ServerDispatcher {
    fn dispatch(&mut self, received: FromClientMessage) -> Option<FromServerMessage> {
        match received {
            FromClientMessage::Write(key, value, clock) => {
                let time = self.write(key, value, clock);
                Some(FromServerMessage::WrittenAt(time))
            }
            _ => Some(FromServerMessage::UnknownPong),
        }
    }
}
