pub const STRAWBERRY_API: &str = "https://api.strawberryfoundations.org/v1/";
pub const STRAWBERRY_ID_API: &str = "https://id.strawberryfoundations.org/v2/";
pub const STRAWBERRY_CLOUD_API_URL: &str = "https://cloud.strawberryfoundations.org/";
pub const CONFIG_VERSION: u8 = 7;

pub const HEADLESS_CONFIG: &str = r#"language: en_US
update_channel: "stable"
detect_same_system_messages: true
experimental_debug_mode: false
extreme_debug_mode: false
config_ver: 7

ui:
  message_format: gray
  enable_terminal_bell: true
  serverlist_show_type: false
  serverlist_show_address: false

notification:
  enabled: true
  use_legacy_notifier: false
  icon_path: ""

networking:
  online_mode: true
  keep_alive: true
  latency_mode: false
  latency_mode_time: 1
  recv_allowed_bytes: 8192

autoserver:
  enabled: false
  server_id: 0

server:
  0:
    name: strawberryfoundations.org
    address: 45.131.109.170
    port: 52800
    type: Main"#;

// pub const STRAWBERRY_CLOUD_API_URL: &str = "http://localhost:8000/";