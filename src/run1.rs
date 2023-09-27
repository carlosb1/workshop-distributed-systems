pub mod adapters;
pub mod core;
pub mod entity;

use crate::adapters::network::server::Server;

fn main() {
    env_logger::init();
    let server = Server::new(0, "green".to_string());
    let handle = server.start();

    handle.join().unwrap();
}
