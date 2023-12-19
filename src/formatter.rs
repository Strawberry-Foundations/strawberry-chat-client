use crate::constants;

pub struct MessageFormatter;


impl MessageFormatter {
    pub fn default_user(username: &str, nickname: &str, role_color: &str, badge: &str, message: &str) {
        let fmt = if nickname == username {
            format!("{}{}{}:{} {}{}", role_color, username, badge, constants::C_RESET, message, constants::C_RESET);
        }
        else {
            format!("{}{} (@{}){}:{} {}{}", role_color, nickname, username.to_lowercase(), badge, constants::C_RESET, message, constants::C_RESET);
        };

        fmt
    }

    pub fn default_system(message: &str) -> String {
        format!("{}", message)
    }
}

pub fn badge_handler(badge: &str) -> String {
    if badge != "" {
        format!(" [{}]", badge)
    }
    else {
        "".to_string()
    }
}