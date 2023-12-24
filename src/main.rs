use std::io::{self,};
use std::net::TcpStream;
use std::thread;
use std::env;
use std::path::PathBuf;

use stblib::colors::*;
use stblib::strings::Strings;
use crate::config::{Config, get_config};

mod recv;
mod send;

mod config;
mod formatter;
mod constants;
mod user_server_list;

fn main() -> io::Result<()> {
    let exe_path = env::current_exe().expect("Error when determining the path to the executable file.");
    let exe_dir = exe_path.parent().expect("Error determining the directory of the executable file.");
    let exe_dir_str = PathBuf::from(exe_dir).display().to_string();

    let config_path = format!("{}/config.yml", exe_dir_str);

    let config = Config::new(&config_path);
    let string_loader = Strings::new(config.language.as_str(), get_config().as_str());

    let server_id;

    if config.autoserver.enabled {
        server_id = config.autoserver.server_id as usize;
    }
    else {
        server_id = user_server_list::user_server_list(&string_loader, &config, &config_path) as usize;
    }

    if server_id as i8 == -1 {
        std::process::exit(0);
    }


    let server_config = Config::server_id(server_id, &config_path);

    let send_config = Config::new(&config_path);
    let send_server_config = Config::server_id(server_id, &config_path);

    let host = (server_config.address.clone(), server_config.port);

    let stream = TcpStream::connect(host).expect(format!("{BOLD}{RED}{}{C_RESET}", string_loader.str("ErrNotReachable")).as_str());

    let send_stream = stream.try_clone().unwrap();

    let handler = thread::spawn(|| recv::recv(stream, config, server_config));
    thread::spawn(|| send::send(send_stream, send_config, send_server_config));

    handler.join().unwrap();

    Ok(())
}