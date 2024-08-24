use lazy_static::lazy_static;

pub const STRAWBERRY_ID_API: &str = "https://id.strawberryfoundations.org/v2/";
pub const STRAWBERRY_CLOUD_API_URL: &str = "https://cloud.strawberryfoundations.org/";
// pub const STRAWBERRY_CLOUD_API_URL: &str = "http://localhost:8000/";

lazy_static! {
    pub static ref VERSION: String = env!("CARGO_PKG_VERSION").to_string();
}

pub const HEADLESS_CONFIG: &str = r#"language: en_US
update_channel: "stable"
detect_same_system_messages: true
message_format: default
enable_notifications: true
enable_terminal_bell: true
experimental_debug_mode: false
extreme_debug_mode: false
recv_allowed_bytes: 8192
config_ver: 6

notification:
  use_legacy_notifier: false
  icon_path: ""


networking:
  online_mode: true
  keep_alive: true
  latency_mode: true
  latency_mode_time: 1

autoserver:
  enabled: false
  server_id: 0

server:
  0:
    name: strawberryfoundations.org
    address: 45.131.109.170
    port: 52800
    type: Main"#;