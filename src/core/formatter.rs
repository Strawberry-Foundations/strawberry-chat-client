use std::collections::HashMap;
use libstrawberry::colors::{C_RESET, GRAY, RESET};

use crate::global::CONFIG;

#[derive(Clone, Copy, Default)]
pub enum MessageFormats {
    #[default]
    Default,
    Gray
}

pub type MessageFormat = HashMap<&'static str, (String, String, String)>;

pub struct MessageFormatter {
    pub format: MessageFormats,
    pub format_str: String,
    pub formats: MessageFormat,
    pub current_format: (String, String, String),
}

impl Default for MessageFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageFormatter {
    #[must_use] pub fn load_formats() -> MessageFormat {
        let mut formats = HashMap::new();

        formats.insert("default", (
            format!("{C_RESET}[[%<time>%]] [%<message>%]"),
            format!("{C_RESET}[[%<time>%]] [%<role_color>%][%<username>%][%<badge>%]:{C_RESET} [%<message>%]{C_RESET}"),
            format!("{C_RESET}[[%<time>%]] [%<role_color>%][%<nickname>%] (@[%<username>%])[%<badge>%]:{C_RESET} [%<message>%]{C_RESET}")
        ));

        formats.insert("gray", (
            format!("{C_RESET}[{GRAY}[%<time>%]{RESET}] [%<message>%]"),
            format!("{C_RESET}[{GRAY}[%<time>%]{RESET}] [%<role_color>%][%<username>%][%<badge>%]:{C_RESET} [%<message>%]{C_RESET}"),
            format!("{C_RESET}[{GRAY}[%<time>%]{RESET}] [%<role_color>%][%<nickname>%] (@[%<username>%])[%<badge>%]:{C_RESET} [%<message>%]{C_RESET}")
        ));

        formats
    }

    #[must_use] pub fn new() -> Self {
        let format = match CONFIG.ui.message_format.as_str() {
            "gray" => MessageFormats::Gray,
            _ => MessageFormats::Default
        };

        let format_str = match format {
            MessageFormats::Default => String::from("default"),
            MessageFormats::Gray => String::from("gray")
        };

        let formats = Self::load_formats();

        let current_format = formats.get_key_value(format_str.as_str()).unwrap().1.to_owned();

        Self {
            format,
            format_str,
            formats,
            current_format
        }
    }

    #[must_use] pub fn user(&self, username: &str, nickname: &str, role_color: &str, badge: &str, message: &str) -> String {
        match nickname {
            _ if username == nickname => self.current_format.1
                .replace("[%<time>%]", &libstrawberry::time::current_time("%H:%M"))
                .replace("[%<role_color>%]", role_color)
                .replace("[%<username>%]", username)
                .replace("[%<role_color>%]", nickname)
                .replace("[%<badge>%]", badge)
                .replace("[%<message>%]", message)
            ,
            _ => self.current_format.2
                .replace("[%<time>%]", &libstrawberry::time::current_time("%H:%M"))
                .replace("[%<role_color>%]", role_color)
                .replace("[%<username>%]", username)
                .replace("[%<nickname>%]", nickname)
                .replace("[%<role_color>%]", nickname)
                .replace("[%<badge>%]", badge)
                .replace("[%<message>%]", message)
        }
    }

    #[must_use] pub fn system(&self, message: String) -> String {
        self.current_format.0
            .replace("[%<time>%]", &libstrawberry::time::current_time("%H:%M"))
            .replace("[%<message>%]", &message)
    }
}

#[must_use] pub fn badge_handler(badge: String) -> String {
    if badge.is_empty() {
        String::new()
    } else {
        format!(" [{badge}]")
    }
}
