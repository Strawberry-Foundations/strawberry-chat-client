use serde::Deserialize;
use serde_yaml::{from_str, Value};
use std::fs;
use std::path::{Path, PathBuf};
use stblib::colors::{BOLD, C_RESET, RED};
use crate::core::auth::IdCredentials;
use crate::constants::{HEADLESS_CONFIG, STRAWBERRY_CLOUD_API_URL};
use crate::global::STRING_LOADER;

#[derive(Debug, Deserialize)]
pub struct UserInterface {
    pub message_format: String,
    pub enable_notifications: bool,
    pub enable_terminal_bell: bool,
}

#[derive(Debug, Deserialize)]
pub struct Networking {
    pub online_mode: bool,
    pub keep_alive: bool,
    pub latency_mode: bool,
    pub latency_mode_time: u8,
    pub recv_allowed_bytes: u32,
}

#[derive(Debug, Deserialize)]
pub struct Autoserver {
    pub enabled: bool,
    pub server_id: i8,
}

#[derive(Debug, Deserialize)]
pub struct Notification {
    pub use_legacy_notifier: bool,
    pub icon_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub language: String,
    pub update_channel: String,
    pub detect_same_system_messages: bool,
    pub experimental_debug_mode: bool,
    pub extreme_debug_mode: bool,
    pub config_ver: u8,
    pub ui: UserInterface,
    pub notification: Notification,
    pub networking: Networking,
    pub autoserver: Autoserver,
    #[serde(skip)]
    pub path: String,
    #[serde(skip)]
    pub content: String,
}

#[derive(Clone, Default)]
pub struct ServerValuesCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Default)]
pub struct ServerValues {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub server_type: String,
    pub autologin: bool,
    pub compatibility_mode: bool,
    pub credentials: ServerValuesCredentials,
}

pub fn config_open(config_path: &str) -> eyre::Result<String> {
    Ok(fs::read_to_string(config_path)?)
}

pub fn get_lang_cfg() -> String {
    include_str!("../lang.yml").to_string()
}

impl Config {
    pub fn new(config_path: String) -> eyre::Result<Self> {
        let config_yml = config_open(&config_path)?;
        let mut cfg: Self = from_str(&config_yml)?;
        cfg.path = config_path;
        cfg.content = config_yml;
        Ok(cfg)
    }

    pub fn new_from_content(mut content: String) -> Self {
        if content == "Invalid filename" {
            println!("{BOLD}{RED}{}{C_RESET}", STRING_LOADER.load("ConfigNotAvailable"));
            content = String::from(HEADLESS_CONFIG);
        } 
        
        let mut cfg: Self = from_str(&content).unwrap();
        cfg.content = content;

        cfg
    }

    pub fn load_language() -> String {
        let exe_path = std::env::current_exe().expect("Could not get your Strawberry Chat Client Executable");
        let exe_dir = exe_path.parent().expect("Error determining the directory of the executable file.");
        let exe_dir_str = PathBuf::from(exe_dir).display().to_string();
        let mut config_path = format!("{exe_dir_str}/config.yml");

        if !Path::new(&config_path).exists() {
            config_path = String::from("./config.yml")
        }

        let config_yml = match config_open(&config_path) {
            Ok(s) => s,
            Err(_) => {
                let credentials = match IdCredentials::new() {
                    Ok(cred) => cred,
                    Err(_) => return String::from("en_US")
                };

                let (username, auth_token) = (credentials.username, credentials.token);

                let url = format!("{STRAWBERRY_CLOUD_API_URL}fetch/{username}@{auth_token}/config_stbchat.yml");
                
                futures::executor::block_on( async { reqwest::get(url).await.unwrap().text().await.unwrap() })
            }
        };

        let config: Self = match from_str(&config_yml) {
            Ok(value) => value,
            Err(_) => return String::from("en_US")
        };

        config.language
    }

    #[allow(dead_code)]
    pub fn get_language(&self) -> &str {
        if ["de_DE", "en_US"].contains(&self.language.as_str()) {
           self.language.as_str()
        }
        else {
            "en_US"
        }
    }

    pub fn server_id(server_id: i8, config_content: &str) -> ServerValues {
        let server_id = server_id as usize;
        let config: Value = from_str(config_content).unwrap();

        let s_name = config["server"][server_id]["name"]
            .as_str()
            .unwrap()
            .to_string();

        let s_host = config["server"][server_id]["address"]
            .as_str()
            .unwrap()
            .to_string();

        let s_port = config["server"][server_id]["port"].as_u64().unwrap() as u16;

        let s_type = config["server"][server_id]["type"]
            .as_str()
            .unwrap()
            .to_string();

        let s_autologin = config["server"][server_id]["autologin"].as_bool().unwrap_or_default();

        let s_compatibility_mode = config["server"][server_id]["compatibility_mode"]
            .as_bool()
            .unwrap_or(false);

        let s_credentials_username = config["server"][server_id]["credentials"]["username"]
            .as_str()
            .unwrap_or("none")
            .to_string();

        let s_credentials_password = config["server"][server_id]["credentials"]["password"]
            .as_str()
            .unwrap_or("none")
            .to_string();

        ServerValues {
            name: s_name,
            address: s_host,
            port: s_port,
            server_type: s_type,
            autologin: s_autologin,
            compatibility_mode: s_compatibility_mode,
            credentials: ServerValuesCredentials {
                username: s_credentials_username,
                password: s_credentials_password,
            },
        }
    }
}
