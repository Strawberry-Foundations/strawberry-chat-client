use std::io::Read;
use std::net::TcpStream;

#[allow(dead_code)]
pub fn legacy_recv(mut stream: TcpStream) -> eyre::Result<()> {
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
}

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