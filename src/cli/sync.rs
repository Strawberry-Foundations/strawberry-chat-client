use std::path::{Path, PathBuf};
use stblib::colors::{BOLD, C_RESET, RED};
use stblib::id::credentials::StrawberryIdCredentials;

use crate::constants::STRAWBERRY_CLOUD_API_URL;
use crate::global::STRINGS;
use crate::utilities::make_absolute_path;

pub async fn sync() -> eyre::Result<()> {
    let credentials = StrawberryIdCredentials::fetch().unwrap_or_else(|_| {
        println!("{RED}{BOLD}{}{C_RESET}", STRINGS.load("CredentialsFileNotExist"));
        std::process::exit(1);
    });

    let (username, auth_token) = (credentials.username, credentials.token);

    let client = reqwest::Client::new();

    let exe_path = std::env::current_exe().unwrap_or_else(|_| {
        println!("{RED}{BOLD}{}{C_RESET}", STRINGS.load("ExecutablePathNotGet"));
        std::process::exit(1);
    });

    let exe_dir = exe_path.parent().unwrap_or_else(|| {
        println!("{RED}{BOLD}{}{C_RESET}", STRINGS.load("ExecutableDirectoryNotFound"));
        std::process::exit(1);
    });

    let exe_dir_str = PathBuf::from(exe_dir).display().to_string();

    let mut config_path = format!("{exe_dir_str}/config.yml");

    if !Path::new(&config_path).exists() {
        config_path = String::from("./config.yml")
    }

    let file_path = make_absolute_path(config_path.as_str());

    let url = format!("{STRAWBERRY_CLOUD_API_URL}upload/{username}@{auth_token}?filename=config_stbchat.yml");

    let file_content = std::fs::read(file_path).unwrap_or_else(|_| {
        println!("{RED}{BOLD}{}{C_RESET}", STRINGS.load("ConfigNotAvailableSync"));
        std::process::exit(1);
    });

    let response = client.post(url)
        .header("Content-Type", "multipart/form-data")
        .body(file_content)
        .send()
        .await
        .unwrap_or_else(|_| {
            println!("{RED}{BOLD}{}{C_RESET}", STRINGS.load("SyncPostError"));
            std::process::exit(1);
        });

    println!("{}", response.text().await?);

    Ok(())
}