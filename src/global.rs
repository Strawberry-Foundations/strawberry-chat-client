use std::env;
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;

use stblib::strings::Strings;
use crate::core::auth::IdCredentials;

use crate::core::config::{Config, get_lang_cfg, ServerValues};
use crate::cli::user_server_list::user_server_list;
use crate::constants::{HEADLESS_CONFIG, STRAWBERRY_CLOUD_API_URL};

lazy_static! {
    pub static ref CONFIG: Config = {
        let exe_path = env::current_exe().expect("Could not get your Strawberry Chat Client Executable");

        let exe_dir = exe_path.parent().expect("Error determining the directory of the executable file.");

        let exe_dir_str = PathBuf::from(exe_dir).display().to_string();

        let mut config_path = format!("{exe_dir_str}/config.yml");

        if !Path::new(&config_path).exists() {
            config_path = String::from("./config.yml")
        }

        Config::new(config_path).unwrap_or_else(|_| {
            let credentials = match IdCredentials::new() {
                Ok(credentials) => credentials,
                Err(_) => return Config::new_from_content(String::from(HEADLESS_CONFIG))
            };

            let (username, auth_token) = (credentials.username, credentials.token);

            let url = format!("{STRAWBERRY_CLOUD_API_URL}fetch/{username}@{auth_token}/config_stbchat.yml");
            let content = futures::executor::block_on( async { reqwest::get(url).await.unwrap().text().await.unwrap() });

            Config::new_from_content(content)
        })
    };

    pub static ref STRING_LOADER: Strings = Strings::new(Config::load_language().as_str(), &get_lang_cfg());

    pub static ref SERVER_CONFIG: ServerValues = {
        match CONFIG.autoserver.enabled {
            true => Config::server_id(CONFIG.autoserver.server_id, &CONFIG.content),
            false => user_server_list(&CONFIG.content)
        }
    };

    pub static ref VERSION: String = env!("CARGO_PKG_VERSION").to_string();
}