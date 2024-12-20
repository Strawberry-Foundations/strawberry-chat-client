#![warn(clippy::all, clippy::nursery)]
#![allow(clippy::missing_const_for_fn, dead_code, clippy::redundant_pub_crate)]

use tokio::net::TcpStream;
use tokio::io::split;
use tokio::{select, spawn};

use stblib::stbchat::net::{IncomingPacketStream, OutgoingPacketStream};
use stblib::colors::{BOLD, C_RESET, GREEN, LIGHT_BLUE, ITALIC, YELLOW, RED};

use std::sync::mpsc::channel;
use std::time::Duration;

use crate::cli::error_handler;
use crate::global::{SERVER_CONFIG, STRINGS};

pub mod communication;
pub mod core;
pub mod cli;
pub mod constants;
pub mod utilities;
pub mod global;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let def = &String::new();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().unwrap_or(def);

    match cmd.as_str() {
        "login" | "auth" => { return cli::sid_auth::login().await },
        "sync" => { return cli::sync::sync().await },
        _ => {}
    }

    let (tx, rx) = channel::<String>();
    error_handler::install()?;

    let host = (SERVER_CONFIG.address.clone(), SERVER_CONFIG.port);

    println!("{YELLOW}{BOLD}{}{C_RESET}", STRINGS.load("TryConnection"));

    let stream = TcpStream::connect(host).await.unwrap_or_else(|_| {
        eprintln!("{RED}{BOLD}{}{C_RESET}", STRINGS.load("ErrNotReachable"));
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

    println!(
        "{GREEN}{BOLD}{}{C_RESET}",
        &STRINGS.load("ConnectedToServer")
            .replace("%s", &format!("{LIGHT_BLUE}{ITALIC}{}{C_RESET}{GREEN}{BOLD}", SERVER_CONFIG.name))
    );

    select! {
        _ = recv_handler => { std::process::exit(0) },
        _ = send_handler => { std::process::exit(0) }
    }

}