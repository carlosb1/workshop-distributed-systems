use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum FromClientMessage {
    Ping,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FromServerMessage {
    Pong(String), // Used for connection oriented protocols
    UnknownPong,  // Used for non-connection oriented protocols
}

pub enum Signal {
    Greet,
    // Any other app event here.
}
