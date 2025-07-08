// src/config.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Config {
    pub modifier_1: String,
    pub modifier_2: String,
    pub modifier_3: String,
    pub paste_modifier_1: String,
    pub paste_modifier_2: String,
    pub paste_modifier_3: String,
    pub is_enabled: bool,
}

pub fn load_config() -> Config {
    confy::load("memorybank-daemon", None).unwrap_or_default()
}

pub fn store_config(cfg: &Config) {
    let _ = confy::store("memorybank-daemon", None, cfg);
}