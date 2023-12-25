#![warn(clippy::all, clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]

use std::env;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::thread;

use lazy_static::lazy_static;
use owo_colors::OwoColorize;
use stblib::colors::*;
use stblib::strings::Strings;

use crate::config::{Config, get_lang_cfg, ServerValues};

mod recv;
mod send;

mod client_meta;
mod config;
mod constants;
mod formatter;
mod keep_alive;
mod user_server_list;
mod utilities;
mod error_handler;

lazy_static! {
    pub static ref CONFIG: Config = {
        let exe_path = env::current_exe().expect("Could not get your Strawberry Chat Client Executable");

        let exe_dir = exe_path.parent().expect("Error determining the directory of the executable file.");

        let exe_dir_str = PathBuf::from(exe_dir).display().to_string();

        let mut config_path = format!("{exe_dir_str}/config.yml");

        if !Path::new(&config_path).exists() {
            config_path = String::from("./config.yml")
        }

        Config::new(config_path)
    };

    pub static ref STRING_LOADER: Strings = Strings::new(CONFIG.language.as_str(), &get_lang_cfg());

    pub static ref SERVER_CONFIG: ServerValues = {
        let server_id = match CONFIG.autoserver.enabled {
            true => CONFIG.autoserver.server_id,
            false => user_server_list::user_server_list(&CONFIG.path).unwrap_or_else(|_| {
                eprintln!("{}", STRING_LOADER.str("Aborted").red().bold());
                std::process::exit(1);
            }),
        };

        if server_id == -1 {
            std::process::exit(0);
        }

        Config::server_id(server_id, &CONFIG.path)
    };

}

fn main() -> eyre::Result<()> {
    error_handler::install().unwrap();

    let host = (SERVER_CONFIG.address.clone(), SERVER_CONFIG.port);

    println!("{}", STRING_LOADER.str("TryConnection").yellow().bold());
    let stream = TcpStream::connect(host).unwrap_or_else(|_| {
        eprintln!("{}", STRING_LOADER.str("ErrNotReachable").red().bold());
        std::process::exit(1);
    });

    let send_stream = stream.try_clone()?;

    if CONFIG.networking.keep_alive {
        let keep_alive_stream = stream.try_clone().unwrap();
        thread::spawn(|| keep_alive::keep_alive(keep_alive_stream));
    }

    let recv_handler = thread::spawn(|| recv::recv(stream).unwrap_or_else(|_| {
        eprintln!("{}", STRING_LOADER.str("ErrorRecvThread").red().bold());
        std::process::exit(1);
    }));

    let send_handler = thread::spawn(|| send::send(send_stream).unwrap_or_else(|_| {
        eprintln!("{}", STRING_LOADER.str("ErrorSendThread").red().bold());
        std::process::exit(1);
    }));

    println!("{BOLD}{}", &STRING_LOADER.str("ConnectedToServer").replace("%s", SERVER_CONFIG.name.as_str()).green().bold());

    recv_handler.join().unwrap();
    send_handler.join().unwrap();

    Ok(())
}