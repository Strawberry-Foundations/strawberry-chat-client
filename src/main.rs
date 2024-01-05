#![warn(clippy::all, clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]

use std::net::TcpStream;
use std::thread;

use owo_colors::OwoColorize;
use std::sync::mpsc::channel;
use crate::communication::keep_alive;

use crate::global::{CONFIG, SERVER_CONFIG, STRING_LOADER};

mod communication {
    pub mod recv;
    pub mod legacy_recv;
    pub mod send;
    pub mod keep_alive;
}

mod client_meta;
mod config;
mod constants;
mod formatter;
mod user_server_list;
mod utilities;
mod error_handler;
mod global;

fn main() -> eyre::Result<()> {
    let (tx, rx) = channel::<()>();
    error_handler::install().unwrap();

    let host = (SERVER_CONFIG.address.clone(), SERVER_CONFIG.port);

    println!("{}", STRING_LOADER.str("TryConnection").yellow().bold());
    let mut stream = TcpStream::connect(host).unwrap_or_else(|_| {
        eprintln!("{}", STRING_LOADER.str("ErrNotReachable").red().bold());
        std::process::exit(1);
    });

    let send_stream = stream.try_clone()?;

    if CONFIG.networking.keep_alive {
        let keep_alive_stream = stream.try_clone().unwrap();
        thread::spawn(|| keep_alive::keep_alive(keep_alive_stream));
    }

    let recv_handler = thread::spawn(move || communication::recv::recv(&mut stream, tx).unwrap_or_else(|_| {
        eprintln!("{}", STRING_LOADER.str("ErrorRecvThread").red().bold());
        std::process::exit(1);
    }));

    let send_handler = thread::spawn(|| communication::send::send(send_stream, rx).unwrap_or_else(|_| {
        eprintln!("{}", STRING_LOADER.str("ErrorSendThread").red().bold());
        std::process::exit(1);
    }));

    println!("{}", &STRING_LOADER.str("ConnectedToServer").replace("%s", SERVER_CONFIG.name.as_str()).green().bold());

    recv_handler.join().unwrap();
    send_handler.join().unwrap();

    Ok(())
}