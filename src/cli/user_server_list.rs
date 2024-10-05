use std::cmp::Ordering;
use std::ops::{Add, Sub};

use serde_yaml::{from_str, Value};
use stblib::colors::*;

use crate::core::config::{Config, ServerValues};
use crate::{global, STRING_LOADER};
use crate::core::update::check_for_updates;
use crate::global::CONFIG;

pub fn user_server_list(config_content: &str) -> ServerValues {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    println!("--- {CYAN}{BOLD}Strawberry Chat{C_RESET} ({LIGHT_BLUE}v{}{RESET}) ---", *global::VERSION);
    println!("{GREEN}{BOLD}{}{C_RESET}\n", STRING_LOADER.load("Welcome"));

    futures::executor::block_on(async {
        check_for_updates().await.unwrap();
    });

    println!("{CYAN}{BOLD}{UNDERLINE}{}{C_RESET}", STRING_LOADER.load("YourChatServers"));

    let data: Value = from_str(config_content).unwrap();
    let server_data_length = data["server"].as_mapping().unwrap().len();

    for i in 0..server_data_length {
        let mut format = format!(
            "{BOLD}[{BLUE}{}{RESET}] {}", i.add(1),
            data["server"][i]["name"].as_str().unwrap()
        );

        if CONFIG.ui.serverlist_show_type {
            format = format!("{format} - {YELLOW}{}{RESET}", data["server"][i]["type"].as_str().unwrap())
        }
        if CONFIG.ui.serverlist_show_address {
            format = format!("{format} - {MAGENTA}{}", data["server"][i]["address"].as_str().unwrap())
        }

        println!("{format}{C_RESET}");
    }

    println!("[{BLUE}{BOLD}{}{RESET}] {}{C_RESET}\n", server_data_length.add(1), STRING_LOADER.load("Custom"));

    let prompt = format!("{CYAN}{BOLD}{}{C_RESET}", STRING_LOADER.load("SelChatServer"));

    let server_selection: u8 = rprompt::prompt_reply_from_bufread(&mut stdin.lock(), &mut stdout.lock(), &prompt).unwrap().parse().unwrap_or_else(|_| {
        eprintln!("{RED}{BOLD}{}{C_RESET}", STRING_LOADER.load("InvalidInput"));
        std::process::exit(1);
    });

    let server_data_length = server_data_length as u8;

    match server_selection.cmp(&server_data_length.add(1)) {
        Ordering::Equal => {
            let prompt_host = format!("{CYAN}{BOLD}{}{C_RESET}", STRING_LOADER.load("Ipaddr"));
            let prompt_port = format!("{CYAN}{BOLD}{}{C_RESET}", STRING_LOADER.load("Port"));

            let address: String = rprompt::prompt_reply_from_bufread(&mut stdin.lock(), &mut stdout.lock(), &prompt_host).unwrap().parse().unwrap_or_else(|_| {
                eprintln!("{RED}{BOLD}{}{C_RESET}", STRING_LOADER.load("InvalidInput"));
                std::process::exit(1);
            });


            let port: u16 = rprompt::prompt_reply_from_bufread(&mut stdin.lock(), &mut stdout.lock(), &prompt_port).unwrap().parse().unwrap_or_else(|_| {
                eprintln!("{RED}{BOLD}{}{C_RESET}", STRING_LOADER.load("InvalidInput"));
                std::process::exit(1);
            });


            ServerValues {
                address,
                port,
                ..Default::default()
            }
        }

        Ordering::Greater => {
            eprintln!("{RED}{BOLD}{}{C_RESET}", STRING_LOADER.load("InvalidServerSelection"));
            std::process::exit(1);
        }
        Ordering::Less => {
            let server_id = server_selection.sub(1) as i8;

            Config::server_id(server_id, config_content)
        }
    }
}
