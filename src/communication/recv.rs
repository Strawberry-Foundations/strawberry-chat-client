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
}