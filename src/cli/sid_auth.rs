use tokio::time::{self, Duration};
use stblib::colors::{BLUE, BOLD, C_RESET, CYAN, GREEN, RED, RESET, YELLOW};
use crate::auth::IdCredentials;
use crate::constants::STRAWBERRY_ID_API;
use crate::global::STRING_LOADER;
use crate::utilities::serializer;


pub async fn login() -> eyre::Result<()> {
    println!("--- {CYAN}{BOLD}Strawberry ID {}{C_RESET} ---", STRING_LOADER.load("Auth"));

    let request = reqwest::get(format!("{STRAWBERRY_ID_API}api/request")).await?;
    let code = if request.status().is_success() {
        match request.text().await {
            Ok(code) => code,
            Err(err) => {
                eprintln!("{BOLD}{RED} ! {RESET} {} {err}{C_RESET}", STRING_LOADER.load("ErrorRequestingLoginCode"));
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("{BOLD}{RED} ! {RESET} {} {}{C_RESET}", STRING_LOADER.load("ErrorRequestingLoginCode"), STRING_LOADER.load("ApiServerNotReachable"));
        std::process::exit(1);
    };

    println!("{} \n{BOLD}{BLUE}{STRAWBERRY_ID_API}de/login/oauth_dialog/stbchat?code={code}{C_RESET}", STRING_LOADER.load("ContinueSidLogin"));

    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        let response = reqwest::get(format!("{STRAWBERRY_ID_API}api/oauth/callback?code={code}")).await?;
        let body = response.text().await?;

        if let Ok(data) = serializer(body.as_str()) {
            if data["data"]["status"] != "Invalid Code" && data["data"]["status"] != "Not authenticated" {
                println!("{GREEN}{BOLD}{}{C_RESET}", STRING_LOADER.load("AuthSuccess"));

                let _email = data["data"]["user"]["email"].as_str().unwrap().to_string();
                let full_name = data["data"]["user"]["full_name"].as_str().unwrap().to_string();
                let _profile_picture = data["data"]["user"]["profile_picture_url"].as_str().unwrap().to_string();
                let username = data["data"]["user"]["username"].as_str().unwrap().to_string();
                let token = data["data"]["user"]["token"].as_str().unwrap().to_string();

                println!("{GREEN}{BOLD}{} {} (@{})", STRING_LOADER.load("LoggedInAs"), full_name, username);

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
                            username,
                            token,
                        };

                        match serde_yaml::to_string(&credentials) {
                            Ok(credentials_str) => {
                                if let Err(err) = std::fs::write(&credentials_path, credentials_str) {
                                    eprintln!("{RED}{BOLD}Error while writing file:{RESET} {}{C_RESET}", err);
                                } else {
                                    // println!("{GREEN}{BOLD}Credentials saved successfully to {:?}{C_RESET}", credentials_path);
                                }
                            }
                            Err(err) => eprintln!("{RED}{BOLD}Error while serializing data:{RESET} {}{C_RESET}", err),
                        }
                    } else {
                        println!("{YELLOW}{BOLD}credentials.yml already exists at {:?}{C_RESET}", credentials_path);
                    }

                } else {
                    eprintln!("{RED}{BOLD}Error while creating config directory:{RESET} Home directory not found.{C_RESET}");
                }

                break
            }
        }

        interval.tick().await;
    }

    Ok(())
}