use std::net::TcpStream;
use std::sync::mpsc::Sender;

use serde_json::{Deserializer, Value};

use owo_colors::OwoColorize;

use stblib::colors::*;

use crate::{CONFIG, STRING_LOADER};
use crate::cli::formatter::MessageFormatter;
use crate::communication::login::login;
use crate::global::SERVER_CONFIG;
use crate::object::client_meta::ClientMeta;
use crate::object::login_packet::ServerLoginCredentialsPacketClient;


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
                        crate::cli::formatter::badge_handler(msg["badge"].as_str().unwrap()).as_str(),
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
                        let (username, password) = if SERVER_CONFIG.autologin {
                            (SERVER_CONFIG.credentials.username.clone(), SERVER_CONFIG.credentials.password.clone())
                        } else {
                            login()
                        };

                        let mut login_packet = ServerLoginCredentialsPacketClient::new(username, password);
                        login_packet.write(stream);

                        tx.send(()).unwrap();
                    },
                    _ => unreachable!()
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
    println!("{}", STRING_LOADER.str("CloseApplication").yellow().bold());
    println!("{}", STRING_LOADER.str("PressCtrlDToExit").bold());

    Ok(())
}