// src/commands.rs
use crate::config::Config;
use serde::Deserialize;
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum DaemonCommand {
    #[serde(rename = "get_config")]
    GetConfig,
    #[serde(rename = "update_config")]
    UpdateConfig { data: Config },
}