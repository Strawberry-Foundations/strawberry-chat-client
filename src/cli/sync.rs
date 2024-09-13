use std::path::{Path, PathBuf};
use stblib::colors::{BOLD, C_RESET, RED};
use crate::auth::IdCredentials;
use crate::constants::STRAWBERRY_CLOUD_API_URL;
use crate::global::STRING_LOADER;
use crate::utilities::make_absolute_path;

pub async fn sync() -> eyre::Result<()> {
    let credentials = IdCredentials::new().unwrap_or_else(|_| {
        println!("{RED}{BOLD}{}{C_RESET}", STRING_LOADER.load("CredentialsFileNotExist"));
        std::process::exit(1);
    });

    let (username, auth_token) = (credentials.username, credentials.token);

    let client = reqwest::Client::new();

    let exe_path = std::env::current_exe().unwrap_or_else(|_| {
        println!("{RED}{BOLD}{}{C_RESET}", STRING_LOADER.load("ExecutablePathNotGet"));
        std::process::exit(1);
    });

    let exe_dir = exe_path.parent().unwrap_or_else(|| {
        println!("{RED}{BOLD}{}{C_RESET}", STRING_LOADER.load("ExecutableDirectoryNotFound"));
        std::process::exit(1);
    });

    let exe_dir_str = PathBuf::from(exe_dir).display().to_string();

    let mut config_path = format!("{exe_dir_str}/config.yml");

    if !Path::new(&config_path).exists() {
        config_path = String::from("./config.yml")
    }

    let file_path = make_absolute_path(config_path.as_str());
    
    /* let path = Path::new(&file_path);
    println!("{}", path.to_str().unwrap()); */

    let url = format!("{STRAWBERRY_CLOUD_API_URL}upload/{username}@{auth_token}?filename=config_stbchat.yml");

    let file_content = std::fs::read(file_path)?;

    let response = client.post(url)
        .header("Content-Type", "multipart/form-data")
        .body(file_content)
        .send()
        .await
        .unwrap_or_else(|_| {
            println!("{RED}{BOLD}{}{C_RESET}", STRING_LOADER.load("SyncPostError"));
            std::process::exit(1);
        });

    println!("{}", response.text().await?);

    Ok(())
}