use std::path::{Path, PathBuf};
use crate::auth::IdCredentials;
use crate::constants::SCLOUD_API_URL;
use crate::utilities::make_absolute_path;

pub async fn sync() -> eyre::Result<()> {
    let credentials = IdCredentials::new().unwrap();

    let (username, auth_token) = (credentials.username, credentials.token);

    let client = reqwest::Client::new();

    let exe_path = std::env::current_exe().expect("Could not get your Strawberry Chat Client Executable");

    let exe_dir = exe_path.parent().expect("Error determining the directory of the executable file.");

    let exe_dir_str = PathBuf::from(exe_dir).display().to_string();

    let mut config_path = format!("{exe_dir_str}/config.yml");

    if !Path::new(&config_path).exists() {
        config_path = String::from("./config.yml")
    }

    let file_path = make_absolute_path(config_path.as_str());
    let path = Path::new(&file_path);

    /* let filename = path.file_name().map_or_else(|| {
        eprintln!("No filename found");
        std::process::exit(1)
    }, |file_name| file_name.to_str().map_or_else(|| {
            eprintln!("Invalid filename");
            std::process::exit(1)
        }, |file_name_str| file_name_str)); */

    println!("{}", path.to_str().unwrap());

    let url = format!("{SCLOUD_API_URL}upload/{username}@{auth_token}?filename=config_stbchat.yml");

    // Lese den Inhalt der Datei
    let file_content = std::fs::read(file_path).unwrap();

    // Sende den Dateiinhalt an den Server
    let response = client.post(url)
        .header("Content-Type", "multipart/form-data")
        .body(file_content)
        .send()
        .await?;

    // Gib die Serverantwort aus
    println!("{}", response.text().await?);

    Ok(())
}