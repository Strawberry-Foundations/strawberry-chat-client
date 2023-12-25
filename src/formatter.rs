use crate::constants;
use stblib::colors::*;
pub struct MessageFormatter;

impl MessageFormatter {
    pub fn default_user(
        username: &str,
        nickname: &str,
        role_color: &str,
        badge: &str,
        message: &str,
    ) -> String {
        let fmt = match nickname {
            _ if username == nickname => format!(
                "{C_RESET}[{}] {}{}{}:{} {}{}",
                stblib::utilities::current_time("%H:%M"),
                role_color,
                username,
                badge,
                constants::C_RESET,
                message,
                constants::C_RESET
            ),
            _ => format!(
                "{C_RESET}[{}] {}{} (@{}){}:{} {}{}",
                stblib::utilities::current_time("%H:%M"),
                role_color,
                nickname,
                username.to_lowercase(),
                badge,
                constants::C_RESET,
                message,
                constants::C_RESET
            ),
        };

        fmt
    }

    pub fn default_system(message: &str) -> String {
        format!(
            "{C_RESET}[{}] {}",
            stblib::utilities::current_time("%H:%M"),
            message
        )
    }
}

pub fn badge_handler(badge: &str) -> String {
    if badge != "" {
        format!(" [{}]", badge)
    } else {
        "".to_string()
    }
}
