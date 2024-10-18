use tokio::time::{self, Duration};
use stblib::colors::{BLUE, BOLD, C_RESET, CYAN, GREEN, RED, RESET, YELLOW};
use stblib::id::StrawberryId;

use crate::core::auth::IdCredentials;
use crate::constants::STRAWBERRY_ID_API;
use crate::global::STRINGS;


pub async fn login() -> eyre::Result<()> {
    println!("--- {CYAN}{BOLD}Strawberry ID {}{C_RESET} ---", STRINGS.load("Auth"));

    let code = StrawberryId::request_code().await?;

    println!("{} \n{BOLD}{BLUE}{STRAWBERRY_ID_API}de/login/oauth_dialog/stbchat?code={code}{C_RESET}", STRINGS.load("ContinueSidLogin"));

    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        match StrawberryId::callback(code.clone()).await {
            Ok(id) => {
                if let Some(id) = id {
                    println!("{GREEN}{BOLD}{}{C_RESET}", STRINGS.load("AuthSuccess"));
                    println!("{GREEN}{BOLD}{} {} (@{}){C_RESET}", STRINGS.load("LoggedInAs"), id.full_name, id.username);

                    if let Some(home_dir) = dirs::home_dir() {
                        let config_dir = home_dir.join(".config").join("strawberry-id");
                        let credentials_path = config_dir.join("credentials.yml");

                        if !config_dir.exists() {
                            if let Err(err) = std::fs::create_dir_all(&config_dir) {
                                eprintln!("{RED}{BOLD}Error while creating config directory:{RESET} {}{C_RESET}", err);
                            }
                        }

                        if !credentials_path.exists() {
                            let credentials = IdCredentials {
                                username: id.username,
                                token: id.token,
                            };

                            match serde_yaml::to_string(&credentials) {
                                Ok(credentials_str) => {
                                    if let Err(err) = std::fs::write(&credentials_path, credentials_str) {
                                        eprintln!("{RED}{BOLD}Error while writing file:{RESET} {}{C_RESET}", err);
                                    }
                                }
                                Err(err) => eprintln!("{RED}{BOLD}Error while serializing data:{RESET} {}{C_RESET}", err),
                            }
                        } else {
                            println!("{YELLOW}{BOLD}{}{C_RESET}", STRINGS.load_with_params("AlreadyLoggedIn", &[&credentials_path.parent().unwrap().display()]));
                        }

                    } else {
                        eprintln!("{RED}{BOLD}Error while creating config directory:{RESET} Home directory not found.{C_RESET}");
                    }
                    break
                }
            },
            Err(..) => {
                std::process::exit(1);
            }
        };

        interval.tick().await;
    }

    Ok(())
}