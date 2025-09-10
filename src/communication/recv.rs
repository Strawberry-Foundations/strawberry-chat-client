use std::sync::mpsc::Sender;

use tokio::net::TcpStream;
use tokio::io::ReadHalf;

use libstrawberry::stbchat::net::IncomingPacketStream;
use libstrawberry::stbchat::packet::ClientPacket;
use libstrawberry::colors::{RED, BOLD, YELLOW, C_RESET};
use libstrawberry::notifications::Notifier;
use libstrawberry::notifications::os::OS;

use crate::core::formatter::MessageFormatter;
use crate::global::{CONFIG, STRINGS};
use crate::core::meta::ClientMeta;


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
                    crate::core::formatter::badge_handler(author.badge),
                    message,
                ));
            },

            Ok(ClientPacket::Event { event_type}) => {
                if event_type == "event.login" {
                    tx.send("event.login".parse().unwrap()).unwrap();
                }
            },

            Ok(ClientPacket::Notification { title, username, avatar_url: _avatar_url, content, bell: _bell }) => {
                if CONFIG.notification.enabled {
                    let mut notifier = Notifier::new(
                        username,
                        content,
                        title,
                        "normal",
                        CONFIG.notification.icon_path.clone(),
                        Some(String::from("SMS")),
                        5,
                        false,
                    ).build();
                    
                    if CONFIG.notification.use_legacy_notifier {
                        match *notifier.platform_notifier.os {
                            OS::Windows => notifier.platform_notifier.os = &OS::WindowsLegacy,
                            OS::Linux => notifier.platform_notifier.os = &OS::LinuxLibNotify,
                            _ => { }
                        }
                    }
                    
                    notifier.send().unwrap_or_else(|_| {
                        println!("{RED}{BOLD}[UImp] {YELLOW}{BOLD}{}",
                            STRINGS.get("NotificationFailed")
                        );
                        false
                    });
                }
            }

            Err(_) => break,
            _ => println!(
                "{RED}{BOLD}[UImp] {YELLOW}{BOLD}{}",
                STRINGS.get("UnimplementedPacket"),
            )
        }
    }

    println!("{YELLOW}{BOLD}{}{C_RESET}", STRINGS.get("CloseApplication"));
    println!("{BOLD}{}{C_RESET}", STRINGS.get("PressCtrlDToExit"));
}