#![warn(clippy::all, clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]

use std::env;
use std::io::{self};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::thread;

use crate::config::{get_config, Config};
use stblib::colors::*;
use stblib::strings::Strings;

mod recv;
mod send;

mod client_meta;
mod config;
mod constants;
mod formatter;
mod keep_alive;
mod user_server_list;
mod utilities;

fn main() -> io::Result<()> {
    let exe_path = env::current_exe().expect("Error when determining the path to the executable file.");
    let exe_dir = exe_path.parent().expect("Error determining the directory of the executable file.");

    let exe_dir_str = PathBuf::from(exe_dir).display().to_string();

    let mut config_path = format!("{exe_dir_str}/config.yml");

    if !Path::new(&config_path).exists() {
        config_path = String::from("./config.yml")
    }

    let config = Config::new(&config_path);
    let string_loader = Strings::new(config.language.as_str(), get_config().as_str());

    let server_id = match config.autoserver.enabled {
        true => config.autoserver.server_id,
        false => user_server_list::user_server_list(&string_loader, &config, &config_path),
    };

    if server_id == -1 {
        std::process::exit(0);
    }

    let server_config = Config::server_id(server_id, &config_path);

    let send_config = Config::new(&config_path);
    let send_server_config = Config::server_id(server_id, &config_path);

    let host = (server_config.address.clone(), server_config.port);

    let stream = TcpStream::connect(host).unwrap_or_else(|_| {
        eprintln!("{BOLD}{RED}{}{C_RESET}", string_loader.str("ErrNotReachable"));
        std::process::exit(1);
    });

    let send_stream = stream.try_clone().unwrap();

    if config.networking.keep_alive {
        let keep_alive_stream = stream.try_clone().unwrap();
        thread::spawn(|| keep_alive::keep_alive(keep_alive_stream));
    }

    let handler = thread::spawn(|| recv::recv(stream, config, server_config));
    thread::spawn(|| send::send(send_stream, send_config, send_server_config));

    handler.join().unwrap();

    Ok(())
}
