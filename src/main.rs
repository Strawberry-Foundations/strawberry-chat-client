#![warn(clippy::all, clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]

use tokio::net::TcpStream;
use tokio::io::split;
use tokio::{select, spawn};

use stblib::stbm::stbchat::net::{IncomingPacketStream, OutgoingPacketStream};

use owo_colors::OwoColorize;
use std::sync::mpsc::channel;

use crate::cli::error_handler;
use crate::communication::keep_alive;

use crate::global::{CONFIG, SERVER_CONFIG, STRING_LOADER};

mod communication {
    pub mod recv;
    pub mod legacy_recv;
    pub mod send;
    pub mod keep_alive;
    pub mod login;
}

mod cli {
    pub mod user_server_list;
    pub mod formatter;
    pub mod error_handler;
}

mod object {
    pub mod client_meta;
    pub mod login_packet;
}

mod config;
mod constants;
mod utilities;
mod global;
mod types;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let (tx, rx) = channel::<String>();
    error_handler::install().unwrap();

    let host = (SERVER_CONFIG.address.clone(), SERVER_CONFIG.port);

    println!("{}", STRING_LOADER.str("TryConnection").yellow().bold());

    let mut stream = TcpStream::connect(host).await.unwrap_or_else(|_| {
        eprintln!("{}", STRING_LOADER.str("ErrNotReachable").red().bold());
        std::process::exit(1);
    });

    let (r_server, w_server) = split(stream);

    let r_server = IncomingPacketStream::wrap(r_server);
    let mut w_server = OutgoingPacketStream::wrap(w_server);

    /* if CONFIG.networking.keep_alive {
        let keep_alive_stream = stream.try_clone().unwrap();
        spawn(keep_alive::keep_alive(keep_alive_stream));
    } */

    let recv_handler = spawn(communication::recv::recv(r_server, tx));
    let send_handler = spawn(communication::send::send(w_server, rx));

    println!("{}", &STRING_LOADER.str("ConnectedToServer").replace("%s", SERVER_CONFIG.name.as_str()).green().bold());

    select! {
        _ = recv_handler => { std::process::exit(0) },
        _ = send_handler => { std::process::exit(0) }
    }
}