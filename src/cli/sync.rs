use std::path::Path;

pub async fn sync() -> eyre::Result<()> {
    let client = reqwest::Client::new();

    let parser: Vec<String> = std::env::args().skip(2).collect();
    let file = parser.clone().first().unwrap().to_string();

    let file_path = make_absolute_path(file.as_str());
    let path = Path::new(&file_path);

    let filename = if let Some(file_name) = path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            file_name_str
        } else {
            eprintln!("Invalid filename");
            std::process::exit(1)
        }
    } else {
        eprintln!("No filename found");
        std::process::exit(1)
    };

    println!("{}", path.to_str().unwrap());

    let url = format!("{API_URL}upload/{username}@{auth_token}?filename={filename}");

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