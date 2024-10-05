use stblib::colors::{BLUE, BOLD, CYAN, C_RESET, GREEN, MAGENTA, RESET, YELLOW};
use crate::constants::STRAWBERRY_API;
use crate::global::{CONFIG, STRING_LOADER, VERSION};
use crate::utilities::serializer;

pub async fn check_for_updates() -> eyre::Result<()> {
    if CONFIG.networking.online_mode {
        let request = reqwest::get(format!("{STRAWBERRY_API}versions")).await?;
        let body = request.text().await?;

        let version = serializer(body.as_str()).map_or_else(
            |_| String::default(),
            |data| data.get("stbchat")
                .and_then(|c| c.get("client"))
                .and_then(|c| c.get("stable"))
                .and_then(|s| s.as_str())
                .map_or_else(String::default, |stable| stable.to_string())
        );

        if format!("v{}", *VERSION) != version {
            println!("{BOLD}{GREEN}{}{C_RESET}", STRING_LOADER.load("UpdateAvailable"));
            println!("{BOLD}{CYAN}strawberry-chat{GREEN}@{MAGENTA}stable {BLUE}{version}{C_RESET}");
            println!("↳ {} {CYAN}{BOLD}strawberry-chat{GREEN}@{MAGENTA}stable {YELLOW}{} {RESET}-> {BLUE}{version}{C_RESET}\n", STRING_LOADER.load("UpgradingFrom"), *VERSION)
        }
    }

    Ok(())
}