#![warn(clippy::all, clippy::nursery)]
#![allow(clippy::missing_const_for_fn, dead_code, clippy::redundant_pub_crate)]

use tokio::net::TcpStream;
use tokio::io::split;
use tokio::{select, spawn};

use stblib::stbchat::net::{IncomingPacketStream, OutgoingPacketStream};

use owo_colors::OwoColorize;
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::cli::error_handler;
use crate::global::{SERVER_CONFIG, STRING_LOADER};

pub mod communication {
    pub mod recv;
    pub mod legacy_recv;
    pub mod send;
    pub mod keep_alive;
    pub mod login;
    pub mod register;
}

pub mod cli {
    pub mod user_server_list;
    pub mod error_handler;
    pub mod sid_auth;
    pub mod sync;
}

pub mod object {
    pub mod client_meta;
    pub mod login_packet;
}

pub mod fmt {
    pub mod formatter;
}

pub mod config;
pub mod constants;
pub mod utilities;
pub mod global;
pub mod auth;


#[tokio::main]
async fn main() -> eyre::Result<()> {
    let def = &String::new();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().unwrap_or(def);

    match cmd.as_str() {
        "login" => { return cli::sid_auth::login().await },
        "sync" => { return cli::sync::sync().await },
        _ => {}
    }
    
    let (tx, rx) = channel::<String>();
    error_handler::install().unwrap();

    let host = (SERVER_CONFIG.address.clone(), SERVER_CONFIG.port);

    println!("{}", STRING_LOADER.load("TryConnection").yellow().bold());

    let stream = TcpStream::connect(host).await.unwrap_or_else(|_| {
        eprintln!("{}", STRING_LOADER.load("ErrNotReachable").red().bold());
        std::process::exit(1);
    });

    let sock_ref = socket2::SockRef::from(&stream);

    let mut ka = socket2::TcpKeepalive::new();
    ka = ka.with_time(Duration::from_secs(20));
    ka = ka.with_interval(Duration::from_secs(20));

    sock_ref.set_tcp_keepalive(&ka)?;

    let (r_server, w_server) = split(stream);

    let r_server = IncomingPacketStream::wrap(r_server);
    let w_server = OutgoingPacketStream::wrap(w_server);

    let recv_handler = spawn(communication::recv::recv(r_server, tx));
    let send_handler = spawn(communication::send::send(w_server, rx));

    println!("{}", &STRING_LOADER.load("ConnectedToServer").replace("%s", SERVER_CONFIG.name.as_str()).green().bold());
    
    select! {
        _ = recv_handler => { std::process::exit(0) },
        _ = send_handler => { std::process::exit(0) }
    }

}