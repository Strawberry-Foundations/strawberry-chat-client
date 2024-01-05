use std::cmp::Ordering;
use std::ops::{Add, Sub};
use eyre::{bail, Context};
use owo_colors::OwoColorize;
use rustyline::error::ReadlineError;
use serde_yaml::{from_str, Value};

use crate::config::config_open;
use crate::{constants, STRING_LOADER};

pub fn user_server_list(config_path: &str) -> eyre::Result<i8> {
    println!("--- {} ({}) ---", "Strawberry Chat".cyan().bold(), constants::VERSION);
    println!("{}\n", STRING_LOADER.str("Welcome").green().bold());
    println!("{}", STRING_LOADER.str("YourChatServers").cyan().bold().underline());

    let config_yml = config_open(config_path);
    let data: Value = from_str(&config_yml).unwrap();
    let server_data_length = data["server"].as_mapping().unwrap().len();
    for i in 0..server_data_length {
        println!(
            "[{}] {}",
            i.add(1).blue().bold(),
            data["server"][i]["name"].as_str().unwrap().bold()
        );
    }

    println!("[{}] {}\n", server_data_length.add(1).blue().bold(), STRING_LOADER.str("Custom").bold());

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
            eprintln!("{}", STRING_LOADER.str("InvalidServerSelection").red().bold());
            std::process::exit(1);
        }
        Ordering::Less => {
            server_selection.sub(1) as i8
        }
    };

    Ok(server)
}
