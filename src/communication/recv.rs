use std::sync::mpsc::Sender;
use owo_colors::OwoColorize;

use tokio::net::TcpStream;
use tokio::io::ReadHalf;

use stblib::stbm::stbchat::net::IncomingPacketStream;
use stblib::stbm::stbchat::packet::ClientPacket;
use stblib::colors::*;
use stblib::notifications::Notifier;
use stblib::notifications::os::OS;

use crate::fmt::formatter::MessageFormatter;
use crate::global::{CONFIG, STRING_LOADER};
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
                let mut notifier = Notifier::new(
                    username,
                    content,
                    title,
                    "normal",
                    CONFIG.notification.icon_path.clone(),
                    Some(String::from("SMS")),
                    false
                ).build();
                
                if CONFIG.notification.use_legacy_notifier {
                    match notifier.internal_notifier.system {
                        OS::Windows => notifier.internal_notifier.system = &OS::WindowsLegacy,
                        OS::Linux => notifier.internal_notifier.system = &OS::LinuxLibNotify,
                        _ => { }
                    }
                }
                
                notifier.send();
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