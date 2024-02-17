use std::env;
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;

use stblib::strings::Strings;

use crate::config::{Config, get_lang_cfg, ServerValues};
use crate::cli::user_server_list::user_server_list;

lazy_static! {
    pub static ref CONFIG: Config = {
        let exe_path = env::current_exe().expect("Could not get your Strawberry Chat Client Executable");

        let exe_dir = exe_path.parent().expect("Error determining the directory of the executable file.");

        let exe_dir_str = PathBuf::from(exe_dir).display().to_string();

        let mut config_path = format!("{exe_dir_str}/config.yml");

        if !Path::new(&config_path).exists() {
            config_path = String::from("./config.yml")
        }

        Config::new(config_path)
    };

    pub static ref STRING_LOADER: Strings = Strings::new(CONFIG.language.as_str(), &get_lang_cfg());

    pub static ref SERVER_CONFIG: ServerValues = {
        match CONFIG.autoserver.enabled {
            true => return Config::server_id(CONFIG.autoserver.server_id, &CONFIG.path),
            false => user_server_list(&CONFIG.path)
        }
    };
}