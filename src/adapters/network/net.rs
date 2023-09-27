use message_io::network::{NetEvent, Transport};
use message_io::node::{self, NodeEvent};

use crate::adapters::network::message::{FromClientMessage, FromServerMessage, Signal};
use crate::adapters::network::server::Dispatcher;
use crate::entity::Node;

use std::sync::Arc;
use std::sync::Mutex;

pub fn run_server<T: Dispatcher + Clone>(node: Node, dispatcher: Arc<Mutex<T>>) {
    log::info!("Running server for address {:}", node.address);
    let (handler, listener) = node::split::<()>();
    handler
        .network()
        .listen(Transport::FramedTcp, node.address)
        .unwrap();

    listener.for_each(move |event| match event.network() {
        NetEvent::Connected(_, _) => unreachable!(), // Used for explicit connections.
        NetEvent::Accepted(_endpoint, _listener) => log::info!("Client connected"), // Tcp or Ws
        NetEvent::Message(endpoint, data) => {
            let message: FromClientMessage = bincode::deserialize(&data).unwrap();
            dispatcher.lock().unwrap().clone().dispatch(message.clone());
            let response =
                bincode::serialize(&FromServerMessage::Pong("This is my pong".to_string()))
                    .unwrap();
            log::info!("message:{:?}", message);
            handler.network().send(endpoint, &response);
        }
        NetEvent::Disconnected(_endpoint) => log::info!("Client disconnected"), //Tcp or Ws
    });
}

pub fn run_client(message: FromClientMessage) {
    let port = std::env::var("PORT").unwrap_or("3042".to_string());
    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let address = format!("{:}:{:}", host, port);

    let (handler, listener) = node::split();

    let (server, _) = handler
        .network()
        .connect(Transport::FramedTcp, address)
        .unwrap();

    listener.for_each(move |event| match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(_endpoint, _ok) => handler.signals().send(Signal::Greet),
            NetEvent::Accepted(_, _) => unreachable!(), // Only generated by listening
            NetEvent::Message(_endpoint, data) => {
                let message: FromServerMessage = bincode::deserialize(&data).unwrap();
                log::info!("Received deserialized: {:?}", message);
                log::info!("Received: {}", String::from_utf8_lossy(data));
            }
            NetEvent::Disconnected(_endpoint) => (),
        },
        NodeEvent::Signal(signal) => match signal {
            Signal::Greet => {
                handler
                    .network()
                    .send(server, &bincode::serialize(&message).unwrap());
                // computed every second
                //handler.network().send(server, "Hello server!".as_bytes());
                //handler
                //    .signals()
                //    .send_with_timer(Signal::Greet, Duration::from_secs(1));
            }
        },
    });
}
