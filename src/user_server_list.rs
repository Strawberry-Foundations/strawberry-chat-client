use std::cmp::Ordering;
use std::ops::Add;
use eyre::{bail, Context};
use rustyline::error::ReadlineError;
use serde_yaml::{from_str, Value};

use stblib::colors::*;

use crate::config::config_open;
use crate::{constants, STRING_LOADER};

pub fn user_server_list(config_path: &str) -> eyre::Result<i8> {
    println!("{BOLD}--- {CYAN}Strawberry Chat ({}){C_RESET} ---", constants::VERSION);
    println!("{BOLD}{GREEN}{}{C_RESET}\n", STRING_LOADER.str("Welcome"));
    println!("{BOLD}{CYAN}{UNDERLINE}{}{C_RESET}", STRING_LOADER.str("YourChatServers"));

    let config_yml = config_open(config_path);
    let data: Value = from_str(&config_yml).unwrap();
    let server_data_length = data["server"].as_mapping().unwrap().len();
    for i in 0..server_data_length {
        println!(
            "{BOLD}{BLUE}[{}]{C_RESET}{BOLD} {}{C_RESET}",
            i + 1,
            data["server"][i]["name"].as_str().unwrap()
        );
    }

    println!("{BOLD}{BLUE}[{}]{C_RESET}{BOLD} {}{C_RESET}\n", server_data_length + 1, STRING_LOADER.str("Custom"));

    let mut line_reader = rustyline::DefaultEditor::new().unwrap();

    let prompt = STRING_LOADER.str("SelChatServer");
    let aborted = STRING_LOADER.str("Aborted");

    let server_selection: u8 = match line_reader.readline(&prompt) {
        Ok(i) => i.trim().parse().context(STRING_LOADER.str("InvalidInput"))?,
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => bail!(aborted),
        Err(e) => bail!(e),
    };

    let server_data_length = server_data_length as u8;

    let server = match server_selection.cmp(&server_data_length.add(1)) {
        Ordering::Equal => {
            //let host = line_reader.readline(STRING_LOADER.str("Ipaddr").as_str()).expect(aborted.as_str());
            //let port: String = line_reader.readline(STRING_LOADER.str("Port").as_str()).expect(aborted.as_str());

            -1
        }
        Ordering::Greater => {
            eprintln!("{BOLD}{RED}{}{C_RESET}", STRING_LOADER.str("InvalidServerSelection"));
            std::process::exit(1);
        }
        Ordering::Less => {
            (server_selection - 1) as i8
        }
    };

    Ok(server)
}
