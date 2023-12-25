use crate::client_meta::ClientMeta;
use owo_colors::OwoColorize;
use serde_json::Value;
use stblib::colors::*;
use stblib::strings::Strings;
use std::io::Read;
use std::net::TcpStream;

use crate::config::{get_config, Config, ServerValues};
use crate::formatter::MessageFormatter;

pub fn recv(mut stream: TcpStream, config: Config, _server_config: ServerValues) {
    let string_loader = Strings::new(config.language.as_str(), get_config().as_str());
    let mut client_meta = ClientMeta::new();

    loop {
        let mut buffer = [0u8; 1];
        let mut str_buf = String::new();
        let mut wraps = 0;

        loop {
            let stream_reader = stream.read(&mut buffer).unwrap_or_else(|_| {
                eprintln!("{BOLD}{RED}{}{C_RESET}", string_loader.str("ConnectionInterrupt"));
                std::process::exit(1);
            });

            if stream_reader == 0 {
                println!("{YELLOW}{}{C_RESET}", string_loader.str("CloseApplication"));
                std::process::exit(0)
            }

            match buffer[0] as char {
                '{' => {
                    wraps += 1;
                    str_buf.push('{');
                }
                '}' => {
                    wraps -= 1;
                    str_buf.push('}');
                }
                c => str_buf.push(c),
            }

            if wraps == 0 {
                break;
            }
        }

        let msg: Value = match serde_json::from_str(&str_buf) {
            Ok(ok) => ok,
            Err(e) => {
                println!("{} error desering packet ({str_buf}): {e}", "[err]".red());
                continue;
            }
        };

        match msg["message_type"].as_str() {
            Some("system_message") => {
                let fmt = match config.message_format.as_str() {
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
                let fmt = match config.message_format.as_str() {
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

            None => unreachable!(),
            m => println!(
                "{} {YELLOW}{BOLD}{} ({})",
                "[UImp] ".red().bold(),
                string_loader.str("UnimplementedPacket"),
                m.unwrap(),
            ),
        }
    }
}
