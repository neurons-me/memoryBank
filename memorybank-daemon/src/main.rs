use std::path::Path;
use std::sync::{Arc, Mutex};

mod keywatch;
mod config;
mod commands;
mod ipc;

use config::Config;
use keywatch::start_key_watcher;
use ipc::start_ipc_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let socket_path = "/tmp/memorybank.sock";

    if Path::new(socket_path).exists() {
        std::fs::remove_file(socket_path)?;
    }

    let initial_config: Config = confy::load("memorybank-daemon", None).unwrap_or_default();
    let config = Arc::new(Mutex::new(initial_config));

    start_key_watcher(config.clone());
    println!("📡 MemoryBank Daemon listening at {}", socket_path);

    start_ipc_server(socket_path, config).await?;
    Ok(())
}
