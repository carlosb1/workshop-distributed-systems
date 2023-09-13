pub mod common;
use common::{FromClientMessage, FromServerMessage, Signal};
use message_io::network::{NetEvent, Transport};
use message_io::node::{self, NodeEvent};

pub fn run_server() {
    let port = std::env::var("PORT").unwrap_or("3042".to_string());
    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let address = format!("{:}:{:}", host, port);

    let (handler, listener) = node::split::<()>();
    handler
        .network()
        .listen(Transport::FramedTcp, address)
        .unwrap();

    listener.for_each(move |event| match event.network() {
        NetEvent::Connected(_, _) => unreachable!(), // Used for explicit connections.
        NetEvent::Accepted(_endpoint, _listener) => println!("Client connected"), // Tcp or Ws
        NetEvent::Message(endpoint, data) => {
            let message: FromClientMessage = bincode::deserialize(&data).unwrap();
            let response =
                bincode::serialize(&FromServerMessage::Pong("This is my pong".to_string()))
                    .unwrap();
            println!("message:{:?}", message);
            handler.network().send(endpoint, &response);
        }
        NetEvent::Disconnected(_endpoint) => println!("Client disconnected"), //Tcp or Ws
    });
}

pub fn run_client() {
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
                println!("Received deserialized: {:?}", message);
                println!("Received: {}", String::from_utf8_lossy(data));
            }
            NetEvent::Disconnected(_endpoint) => (),
        },
        NodeEvent::Signal(signal) => match signal {
            Signal::Greet => {
                let output_data = FromClientMessage::Ping;
                handler
                    .network()
                    .send(server, &bincode::serialize(&output_data).unwrap());
                // computed every second
                //handler.network().send(server, "Hello server!".as_bytes());
                //handler
                //    .signals()
                //    .send_with_timer(Signal::Greet, Duration::from_secs(1));
            }
        },
    });
}