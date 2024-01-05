use crate::constants;
use stblib::colors::*;
pub struct MessageFormatter;

impl MessageFormatter {
    pub fn default_user(username: &str, nickname: &str, role_color: &str, badge: &str, message: &str) -> String {

        match nickname {
            _ if username == nickname => format!(
                "{C_RESET}[{}] {role_color}{username}{badge}:{C_RESET} {message}{C_RESET}",
                stblib::utilities::current_time("%H:%M"),
            ),
            _ => format!(
                "{C_RESET}[{}] {role_color}{nickname} (@{}){badge}:{C_RESET} {message}{C_RESET}",
                stblib::utilities::current_time("%H:%M"),
                username.to_lowercase(),
            ),
        }
    }

    pub fn default_system(message: &str) -> String {
        format!(
            "{C_RESET}[{}] {message}",
            stblib::utilities::current_time("%H:%M"),
        )
    }
}

pub fn badge_handler(badge: &str) -> String {
    if !badge.is_empty() {
        format!(" [{}]", badge)
    } else {
        String::new()
    }
}
