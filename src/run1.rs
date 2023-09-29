/// First use case applying the lamport example.
pub mod adapters;
pub mod core;
pub mod entity;

use std::thread::JoinHandle;

use adapters::network::client::Client;

use crate::adapters::network::server::Server;

fn main() {
    env_logger::init();
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let server_green = Server::new(0, "green".to_string());
    let handle = server_green.start();
    let client = Client::new(1);
    client.write("name".to_string(), "alice".to_string());
    handles.push(handle);
    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}
