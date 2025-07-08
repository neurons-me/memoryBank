

use serde::{Serialize, Deserialize};
use std::os::unix::net::UnixStream;
use std::io::{Write, BufReader, BufRead};

const SOCKET_PATH: &str = "/tmp/memorybank.sock";

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Config {
    pub modifier_1: String,
    pub modifier_2: String,
    pub modifier_3: String,
    pub paste_modifier_1: String,
    pub paste_modifier_2: String,
    pub paste_modifier_3: String,
    pub is_enabled: bool,
}

pub fn fetch_config_from_daemon() -> Option<Config> {
    let mut stream = UnixStream::connect(SOCKET_PATH).ok()?;
    let mut reader = BufReader::new(stream.try_clone().ok()?);
    let _ = stream.write_all(b"{\"type\":\"get_config\"}\n").ok()?;

    let mut response = String::new();
    reader.read_line(&mut response).ok()?;
    serde_json::from_str::<Config>(&response).ok()
}

pub fn send_config_to_daemon(cfg: &Config) {
    if let Ok(mut stream) = UnixStream::connect(SOCKET_PATH) {
        if let Ok(json) = serde_json::to_string(&serde_json::json!({
            "type": "update_config",
            "data": cfg
        })) {
            let _ = stream.write_all(json.as_bytes());
            let _ = stream.write_all(b"\n");
        }
    }
}