use std::cmp::Ordering;
use std::ops::{Add, Sub};
use owo_colors::OwoColorize;

use serde_yaml::{from_str, Value};
use stblib::colors::{BOLD, C_RESET, CYAN, RED};

use crate::config::{Config, ServerValues};
use crate::{constants, STRING_LOADER};


pub fn user_server_list(config_content: &str) -> ServerValues {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    println!("--- {} ({}) ---", "Strawberry Chat".cyan().bold(), constants::VERSION);
    println!("{}\n", STRING_LOADER.load("Welcome").green().bold());
    println!("{}", STRING_LOADER.load("YourChatServers").cyan().bold().underline());

    let data: Value = from_str(config_content).unwrap();
    let server_data_length = data["server"].as_mapping().unwrap().len();

    for i in 0..server_data_length {
        println!(
            "[{}] {}",
            i.add(1).blue().bold(),
            data["server"][i]["name"].as_str().unwrap().bold()
        );
    }

    println!("[{}] {}\n", server_data_length.add(1).blue().bold(), STRING_LOADER.load("Custom").bold());

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
            eprintln!("{}", STRING_LOADER.load("InvalidServerSelection").red().bold());
            std::process::exit(1);
        }
        Ordering::Less => {
            let server_id = server_selection.sub(1) as i8;

            Config::server_id(server_id, &config_content)
        }
    }
}
