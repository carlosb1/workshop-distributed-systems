use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FromClientMessage {
    Ping,
    Read(String, Option<String>, Option<u64>),
    Write(String, String, u64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FromServerMessage {
    WrittenAt(u64),
    Pong(String), // Used for connection oriented protocols
    UnknownPong,  // Used for non-connection oriented protocols
}

pub enum Signal {
    Greet,
    // Any other app event here.
}
