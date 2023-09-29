/// Network server implementation for message-io
use crate::adapters::lamport::LamportClock;
use crate::adapters::network::net::run_server;
use crate::core::DistributedServerDispatcher;
use crate::entity::Node;
use log;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::{self, JoinHandle};

/// Server struturre with name, node information and dispatcher
#[derive(Clone, Default)]
pub struct Server {
    node: Node,
    name: String,
    dispatcher: DistributedServerDispatcher,
}

impl Server {
    pub fn new(latest_time: u64, name: String) -> Self {
        Server {
            name,
            dispatcher: DistributedServerDispatcher::new(
                LamportClock::new(latest_time),
                HashMap::new(),
            ),
            ..Default::default()
        }
    }

    pub fn start(self) -> JoinHandle<()> {
        //We protect the node and the dispatcher, first one with a shared thread reference. Second
        //one with new copy
        log::info!("Starting server with name: {:?}", self.name);
        let cloned_node = self.node;
        let cloned_dispatcher = Arc::new(Mutex::new(self.dispatcher));
        thread::spawn(move || {
            run_server(cloned_node, cloned_dispatcher);
        })
    }
}
