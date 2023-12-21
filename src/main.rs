use std::any::{type_name};
use std::io::{self,};
use std::net::TcpStream;
use std::thread;
use serde_yaml::{from_str, Value};

use stblib::colors::*;
use stblib::strings::Strings;
use crate::config::{Config, config_open};


mod recv;
mod send;

mod config;
mod formatter;
mod constants;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn user_server_list(string_loader: &Strings, _config: &Config) -> i8 {
    let mut enable_auto_login = false;

    println!("{BOLD}--- {CYAN}Strawberry Chat ({}){C_RESET} ---", constants::VERSION);
    println!("{GREEN}{}{C_RESET}\n", string_loader.str("Welcome"));
    println!("{BOLD}{CYAN}{UNDERLINE}{}{C_RESET}", string_loader.str("YourChatServers"));

    let config_yml = config_open();
    let data: Value = from_str(&config_yml).unwrap();
    let server_data_length = data["server"].as_mapping().unwrap().len();

    for i in 0..server_data_length {
        println!(
            "{BOLD}{BLUE}[{}]{C_RESET}{BOLD} {}{C_RESET}",
            i + 1,
            data["server"][i]["name"].as_str().unwrap()
        );
    }

    println!("{BOLD}{BLUE}[{}]{C_RESET}{BOLD} {}{C_RESET}\n", server_data_length + 1, string_loader.str("Custom"));

    let mut line_reader = rustyline::DefaultEditor::new().unwrap();

    let prompt = format!("{}", string_loader.str("SelChatServer"));
    let aborted = format!("{BOLD}{YELLOW}{}{C_RESET}", string_loader.str("Aborted"));

    let server_selection: u8 = match line_reader.readline(&prompt) {
        Ok(line) => match line.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!(
                    "{BOLD}{RED}{}{C_RESET}",
                    format!("{}", string_loader.str("InvalidInput"))
                );
                std::process::exit(1);
            }
        },
        Err(_) => {
            eprintln!("{}", aborted.as_str());
            std::process::exit(1);
        }
    };

    if server_selection == (server_data_length + 1) as u8 {
        let host = line_reader.readline(string_loader.str("Ipaddr").as_str()).expect(aborted.as_str());
        let port: String = line_reader.readline(string_loader.str("Port").as_str()).expect(aborted.as_str());

        -1
    }

    else if server_selection > (server_data_length + 1) as u8 {
        eprintln!("{BOLD}{RED}{}{C_RESET}", string_loader.str("InvalidServerSelection"));
        std::process::exit(1);
    }

    else {
        (server_selection - 1) as i8
    }
}

fn main() -> io::Result<()> {
    let config = config::Config::new();
    let string_loader = stblib::strings::Strings::new(config.language.as_str(), "C:\\Users\\Julian\\Desktop\\stbchat-rust\\target\\debug\\lang.yml");

    let mut server_id: usize = 0;

    if config.autoserver.enabled {
        server_id = config.autoserver.server_id as usize;
    }
    else {
        server_id = user_server_list(&string_loader, &config) as usize;
    }

    if server_id as i8 == -1 {
        std::process::exit(0);
    }


    let server_config = config::Config::server_id(server_id);

    let send_config = config::Config::new();
    let send_server_config = config::Config::server_id(server_id);

    let host = format!("{}:{}", server_config.address, server_config.port);

    let stream = TcpStream::connect(host).expect(format!("{BOLD}{RED}{}{C_RESET}", string_loader.str("ErrNotReachable")).as_str());

    let send_stream = stream.try_clone().unwrap();

    let handler = thread::spawn(|| recv::recv(stream, config, server_config, string_loader));
    thread::spawn(|| send::send(send_stream, send_config, send_server_config));

    handler.join().unwrap();

    Ok(())
}