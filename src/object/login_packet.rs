

use serde::{Deserialize, Serialize};




#[derive(Debug, Serialize, Deserialize)]
pub struct EventPacket {
    pub packet_type: String,
    pub event_type: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerLoginCredentialsPacketClient {
    pub packet_type: String,
    pub event_type: String,
    pub credentials: Credentials
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}