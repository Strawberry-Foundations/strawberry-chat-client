use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use eyre::bail;

use owo_colors::OwoColorize;
use rustyline::error::ReadlineError;
use serde_json::{Deserializer, Value};
use stblib::colors::*;
use std::sync::mpsc::Sender;

use crate::{CONFIG, STRING_LOADER};
use crate::client_meta::ClientMeta;
use crate::formatter::MessageFormatter;

pub fn recv(stream: &mut TcpStream, tx: Sender<()>) -> eyre::Result<()> {
    let mut client_meta = ClientMeta::new();
    let iter_stream = stream.try_clone().unwrap();

    let json_iter = Deserializer::from_reader(iter_stream).into_iter::<Value>();

    for json in json_iter {
        let msg = match json {
            Ok(j) => j,
            Err(e) => {
                eprintln!("Failed to deserialize json: {e}");
                continue
            },
        };

        match msg["message_type"].as_str() {
            Some("system_message") => {
                let fmt = match CONFIG.message_format.as_str() {
                    "default" => MessageFormatter::default_system(
                        msg["message"]["content"].as_str().unwrap(),
                    ),
                    _ => MessageFormatter::default_system(
                        msg["message"]["content"].as_str().unwrap(),
                    ),
                };

                println!("{}", fmt);
            }
            Some("user_message") => {
                let fmt = match CONFIG.message_format.as_str() {
                    "default" => MessageFormatter::default_user(
                        msg["username"].as_str().unwrap(),
                        msg["nickname"].as_str().unwrap(),
                        msg["role_color"].as_str().unwrap(),
                        crate::formatter::badge_handler(msg["badge"].as_str().unwrap()).as_str(),
                        msg["message"]["content"].as_str().unwrap(),
                    ),
                    _ => MessageFormatter::default_user(
                        msg["username"].as_str().unwrap(),
                        msg["nickname"].as_str().unwrap(),
                        msg["role_color"].as_str().unwrap(),
                        msg["badge"].as_str().unwrap(),
                        msg["message"]["content"].as_str().unwrap(),
                    ),
                };

                println!("{}", fmt);
            }

            Some("stbchat_backend") => {
                client_meta.username = msg["user_meta"]["username"].as_str().unwrap().trim().to_string();
            }

            Some("stbchat_event") => {
                match msg["event_type"].as_str() {
                    Some("event.login") => {
                        let mut line_reader = rustyline::DefaultEditor::new().unwrap();

                        let username: String = match line_reader.readline("Username: ") {
                            Ok(i) => i,
                            Err(ReadlineError::Interrupted) => {
                                stream.write_all(b"exit")?;
                                sleep(Duration::from_millis(300));
                                exit(0);
                            }
                            Err(ReadlineError::Eof) => exit(0),
                            Err(e) => bail!(e),
                        };

                        let _password: String = match line_reader.readline("Password: ") {
                            Ok(i) => i,
                            Err(ReadlineError::Interrupted) => {
                                stream.write_all(b"exit")?;
                                sleep(Duration::from_millis(300));
                                exit(0);
                            }
                            Err(ReadlineError::Eof) => exit(0),
                            Err(e) => bail!(e),
                        };

                        stream.write_all(username.as_bytes()).unwrap();
                        tx.send(()).unwrap();
                    },
                    None => unreachable!(),
                    Some(&_) => unreachable!()
                }
            }

            None => unreachable!(),
            m => println!(
                "{} {YELLOW}{BOLD}{} ({})",
                "[UImp] ".red().bold(),
                STRING_LOADER.str("UnimplementedPacket"),
                m.unwrap(),
            ),
        }
    }
    println!("{}", STRING_LOADER.str("CloseApplication").yellow());
    println!("{}", STRING_LOADER.str("PressCtrlDToExit"));

    Ok(())
}

/* pub fn legacy_recv(mut stream: TcpStream) -> eyre::Result<()> {
    let mut buffer = String::new();

    loop {
        match stream.read_to_string(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                println!("{}", buffer);
                buffer.clear();
            }
            Err(e) => {
                eprintln!("err while reading: {}", e);
                break;
            }
        }
    }

    Ok(())
} */