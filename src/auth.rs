use serde::{Deserialize, Serialize};
use stblib::colors::{BOLD, C_RESET, RED, RESET};
use crate::global::STRING_LOADER;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdCredentials {
    pub username: String,
    pub token: String,
}

impl IdCredentials {
    pub fn new() -> Result<IdCredentials, Box<dyn std::error::Error>> {
        if let Some(home_dir) = dirs::home_dir() {
            let config_dir = home_dir.join(".config").join("strawberry-id");
            let credentials_path = config_dir.join("credentials.yml");

            if credentials_path.exists() {
                let credentials_str = std::fs::read_to_string(&credentials_path)?;

                let credentials: IdCredentials = serde_yaml::from_str(&credentials_str)?;

                Ok(credentials)
            } else {
                Err(format!("{RED}{BOLD}{}{RESET} {}{C_RESET}", STRING_LOADER.load("ErrorReadingCredentials"), STRING_LOADER.load("CredentialsFileNotExist")).into())
            }
        } else {
            Err(format!("{RED}{BOLD}{}{RESET} {}{C_RESET}", STRING_LOADER.load("ErrorReadingCredentials"), STRING_LOADER.load("HomeDirectoryNotFound")).into())
        }
    }
}