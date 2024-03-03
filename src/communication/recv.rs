use std::sync::mpsc::Sender;
use owo_colors::OwoColorize;

use tokio::net::TcpStream;
use tokio::io::ReadHalf;

use stblib::stbm::stbchat::net::IncomingPacketStream;
use stblib::stbm::stbchat::packet::ClientPacket;
use stblib::colors::*;
use stblib::notifications::Notifier;

use crate::fmt::formatter::MessageFormatter;
use crate::global::STRING_LOADER;
use crate::object::client_meta::ClientMeta;


pub async fn recv(mut r_server: IncomingPacketStream<ReadHalf<TcpStream>>, tx: Sender<String>) {
    let _client_meta = ClientMeta::new();
    let formatter = MessageFormatter::new();

    loop {
        match r_server.read::<ClientPacket>().await {
            Ok(ClientPacket::SystemMessage { message}) => {
                println!("{}", formatter.system(message));
            },

            Ok(ClientPacket::UserMessage { author, message }) => {
                println!("{}", formatter.user(
                    author.username,
                    author.nickname,
                    author.role_color,
                    crate::fmt::formatter::badge_handler(author.badge),
                    message,
                ));
            },

            Ok(ClientPacket::Event { event_type}) => {
                if event_type == "event.login" {
                    tx.send("event.login".parse().unwrap()).unwrap();
                }
            },

            Ok(ClientPacket::Notification { title, username, avatar_url: _avatar_url, content, bell: _bell }) => {
                let content = strip_ansi_escapes::strip_str(content);

                Notifier::new(
                    username,
                    content,
                    title,
                    "normal",
                    "/home/julian/Projekte/stbchat-rust/sf_logo_small.ico",
                    Some(String::from("SMS")),
                    false
                ).build().send();
            }

            Err(_) => break,
            _ => println!(
                "{RED}{BOLD}[UImp] {YELLOW}{BOLD}{}",
                STRING_LOADER.load("UnimplementedPacket"),
            )
        }
    }

    println!("{}", STRING_LOADER.load("CloseApplication").yellow().bold());
    println!("{}", STRING_LOADER.load("PressCtrlDToExit").bold());

    /*
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

    Ok(()) */
}