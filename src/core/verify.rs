use crate::constants::STRAWBERRY_API;
use crate::global::CONFIG;
use serde_yaml::Value;
use stblib::colors::{GREEN, RESET};
use reqwest::Client;

pub async fn verify_server(data: &Value) -> Vec<String> {
    let mut verified_servers: Vec<String> = Vec::new();
    let client = Client::new();

    if CONFIG.networking.online_mode {
        let server_data = data.get("server").and_then(|s| s.as_mapping());
        if let Some(server_map) = server_data {
            for (_, server_data) in server_map.iter() {
                if let Some(address) = server_data.get("address").and_then(|a| a.as_str()) {

                    let request = match client
                        .get(format!("{STRAWBERRY_API}server/verified?addr={}", address))
                        .send()
                        .await
                    {
                        Ok(response) => response,
                        Err(_) => {
                            continue
                        },
                    };

                    let body = match request.text().await {
                        Ok(text) => text,
                        Err(_) => {
                            continue
                        }
                    };

                    if body == "True" {
                        verified_servers.push(address.to_string());
                    }
                }
            }
        }
    }

    verified_servers
}

pub fn is_in_verified_servers(address: &str, verified_servers: &[String]) -> String {
    if verified_servers.contains(&address.to_string()) {
        format!(" {GREEN}✓{RESET}")
    } else {
        String::default()
    }
}
