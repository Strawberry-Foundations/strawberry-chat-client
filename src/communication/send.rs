use std::process::exit;
use std::time::Duration;
use std::sync::mpsc::Receiver;

use tokio::net::TcpStream;
use tokio::time::sleep;
use tokio::io::WriteHalf;

use rustyline::error::ReadlineError;

use stblib::colors::*;
use stblib::stbchat::net::OutgoingPacketStream;
use stblib::stbchat::packet::ServerPacket;

use crate::{SERVER_CONFIG, STRING_LOADER};
use crate::communication::login::{login, PreAuthEvent};

use crate::utilities::delete_last_line;


pub async fn send(mut w_server: OutgoingPacketStream<WriteHalf<TcpStream>>, rx: Receiver<String>) {
    if SERVER_CONFIG.autologin {
        println!("{GREEN}{BOLD}{}{C_RESET}\n", STRING_LOADER.load("AutologinActive"));
    } else {
        println!("{GREEN}{BOLD}{}{C_RESET}\n", STRING_LOADER.load("AutologinNotActive"));
    }

    if !SERVER_CONFIG.compatibility_mode {
        loop {
            let tx_data = rx.recv().unwrap_or_else(|_| {
                println!("{BOLD}{YELLOW}{}{C_RESET}", STRING_LOADER.load("UnsuccessfulConnection"));
                exit(1);
            });

            if tx_data == "event.login" {
                let auth = if SERVER_CONFIG.autologin {
                    PreAuthEvent::Login(SERVER_CONFIG.credentials.username.clone(), SERVER_CONFIG.credentials.password.clone())
                } else {
                    login()
                };
                
                match auth {
                    PreAuthEvent::Login(username, password) => {
                        if username != "register" && password != "register" {
                            w_server.write(ServerPacket::Login {
                                username,
                                password
                            }).await.unwrap_or_else(|_| { panic!("{}", STRING_LOADER.load("StreamWriteError")) });
                            break;
                        }
                    }
                    PreAuthEvent::Register(username, password, role_color) => {
                        w_server.write(ServerPacket::Register {
                            username,
                            password,
                            role_color
                        }).await.unwrap_or_else(|_| { panic!("{}", STRING_LOADER.load("StreamWriteError")) });
                        break;
                    }
                }
            }
        }
    }

    let mut line_reader = rustyline::DefaultEditor::new().unwrap();

    if SERVER_CONFIG.autologin && SERVER_CONFIG.compatibility_mode {
        w_server.write(
            SERVER_CONFIG.credentials.username.as_bytes())
            .await
            .unwrap_or_else(|_| { panic!("{}", STRING_LOADER.load("StreamWriteError")) });

        stblib::utilities::ms_sleep(500);

        w_server.write(
            SERVER_CONFIG.credentials.password.as_bytes())
            .await
            .unwrap_or_else(|_| { panic!("{}", STRING_LOADER.load("StreamWriteError")) });
    }

    loop {
        let input: String = match line_reader.readline("") {
            Ok(i) => i,
            Err(ReadlineError::Interrupted) => {
                w_server.write(
                    ServerPacket::Message {
                        message: "/exit".to_string()
                    }
                ).await.unwrap_or_else(|_| { panic!("{}", STRING_LOADER.load("StreamWriteError")) });

                sleep(Duration::from_millis(300)).await;
                exit(0);
            }
            Err(ReadlineError::Eof) => exit(0),
            Err(_e) => {
                eprintln!("{}", STRING_LOADER.load("StreamWriteError"));
                exit(1);
            },
        };

        line_reader.add_history_entry(&input).unwrap();

        w_server.write(
            ServerPacket::Message {
                message: input
            })
            .await
            .unwrap_or_else(|_| { panic!("{}", STRING_LOADER.load("StreamWriteError")) });

        delete_last_line();
    }
}
