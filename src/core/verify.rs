use crate::constants::STRAWBERRY_API;
use crate::global::CONFIG;
use libstrawberry::colors::{GREEN, RESET};
use reqwest::Client;
use serde_yaml::Value;

pub async fn verify_server(data: &Value) -> Vec<String> {
    let mut verified_servers: Vec<String> = Vec::new();
    let client = Client::new();

    if CONFIG.networking.online_mode {
        let server_data = data.get("server").and_then(|s| s.as_mapping());
        if let Some(server_map) = server_data {
            for (_, server_data) in server_map {
                if let Some(address) = server_data.get("address").and_then(|a| a.as_str()) {
                    let Ok(request) = client
                        .get(format!("{STRAWBERRY_API}server/verified?addr={address}"))
                        .send()
                        .await
                    else {
                        continue;
                    };

                    let Ok(body) = request.text().await else {
                        continue;
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

#[must_use]
pub fn is_in_verified_servers(address: &str, verified_servers: &[String]) -> String {
    if verified_servers.contains(&address.to_string()) {
        format!(" {GREEN}✓{RESET}")
    } else {
        String::default()
    }
}
