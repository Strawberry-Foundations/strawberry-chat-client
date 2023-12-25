#![warn(clippy::all, clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]

use std::env;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::thread;
use lazy_static::lazy_static;

use crate::config::{get_lang_cfg, Config, ServerValues};
use stblib::strings::Strings;
use stblib::colors::{RED, BOLD, C_RESET};

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
        let exe_path = env::current_exe().expect(
            format!("{RED}Error: Could not get your Strawberry Chat Client Executable.{C_RESET}")
            .as_str()
        );

        let exe_dir = exe_path.parent().expect(
            format!("{RED}Error determining the directory of the executable file.{C_RESET}")
            .as_str()
        );

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
                eprintln!("{BOLD}{RED}{}{C_RESET}", STRING_LOADER.str("Aborted"));
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
    let stream = TcpStream::connect(host).unwrap_or_else(|_| {
        eprintln!("{BOLD}{RED}{}{C_RESET}", STRING_LOADER.str("Aborted"));
        std::process::exit(1);
    });

    let send_stream = stream.try_clone()?;

    if CONFIG.networking.keep_alive {
        let keep_alive_stream = stream.try_clone().unwrap();
        thread::spawn(|| keep_alive::keep_alive(keep_alive_stream));
    }

    let recv_handler = thread::spawn(|| recv::recv(stream).unwrap_or_else(|_| {
        eprintln!("{BOLD}{RED}{}{C_RESET}", STRING_LOADER.str("ErrorRecvThread"));
        std::process::exit(1);
    }));

    let send_handler = thread::spawn(|| send::send(send_stream).unwrap_or_else(|_| {
        eprintln!("{BOLD}{RED}{}{C_RESET}", STRING_LOADER.str("ErrorSendThread"));
        std::process::exit(1);
    }));

    recv_handler.join().unwrap();
    send_handler.join().unwrap();

    Ok(())
}