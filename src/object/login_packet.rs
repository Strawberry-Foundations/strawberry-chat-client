use std::io::Write;
use std::net::TcpStream;
use serde::{Deserialize, Serialize};
use crate::types::{LOGIN_EVENT, STBCHAT_EVENT};

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

impl ServerLoginCredentialsPacketClient {
    pub fn new(username: String, password: String) -> ServerLoginCredentialsPacketClient {
        Self {
            packet_type: STBCHAT_EVENT.to_string(),
            event_type: LOGIN_EVENT.to_string(),
            credentials: Credentials {
                username,
                password,
            },
        }
    }

    pub fn write(&mut self, stream: &mut TcpStream) {
        stream.write_all(serde_json::to_string(self).unwrap().as_bytes()).unwrap_or_else(|_| eprintln!("Boo booh!"))
    }
}