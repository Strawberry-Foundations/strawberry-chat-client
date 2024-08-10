use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdCredentials {
    pub username: String,
    pub token: String,
}

impl IdCredentials {
    pub fn new() -> Result<Self, ()> {
        dirs::home_dir().map_or_else(|| { std::process::exit(1); }, |home_dir| {
            let config_dir = home_dir.join(".config").join("strawberry-id");
            let credentials_path = config_dir.join("credentials.yml");

            if credentials_path.exists() {
                let credentials_str = std::fs::read_to_string(&credentials_path).unwrap();
                let credentials: Self = serde_yaml::from_str(&credentials_str).unwrap();

                Ok(credentials)
            } else {
                Err(())
            }
        })
    }
}