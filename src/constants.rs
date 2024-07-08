pub const VERSION: &str = "1.3.5";
pub const STRAWBERRY_ID_API: &str = "https://id.strawberryfoundations.org/v2/";
pub const SCLOUD_API_URL: &str = "https://cloud.strawberryfoundations.org/";
// pub const SCLOUD_API_URL: &str = "http://localhost:8000/";


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
    name: strawberryfoundations.xyz
    address: 45.131.109.170
    port: 49200
    type: Main"#;