use serde::{Serialize, Deserialize};
use tokio::net::UnixListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use std::sync::{Arc, Mutex};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct Config {
    modifier_1: String,
    modifier_2: String,
    modifier_3: String,
    is_enabled: bool,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum DaemonCommand {
    #[serde(rename = "get_config")]
    GetConfig,
    #[serde(rename = "update_config")]
    UpdateConfig { data: Config },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let socket_path = "/tmp/memorybank.sock";

    if Path::new(socket_path).exists() {
        std::fs::remove_file(socket_path)?;
    }

    let initial_config: Config = confy::load("memorybank-daemon", None).unwrap_or_default();
    let config = Arc::new(Mutex::new(initial_config));

    println!("📡 MemoryBank Daemon listening at {}", socket_path);
    let listener = UnixListener::bind(socket_path)?;

    loop {
        let (stream, _) = listener.accept().await?;
        let config = config.clone();

        tokio::spawn(async move {
            let (reader, writer) = stream.into_split();
            let mut reader = BufReader::new(reader);
            let mut writer = BufWriter::new(writer);
            let mut buffer = String::new();

            if reader.read_line(&mut buffer).await.is_err() {
                eprintln!("⚠️ Error reading from client");
                return;
            }

            println!("📥 Raw input: {}", buffer.trim());

            let cmd: Result<DaemonCommand, _> = serde_json::from_str(&buffer.trim());
            match cmd {
                Ok(DaemonCommand::GetConfig) => {
                    let cfg = config.lock().unwrap().clone();
                    let response = serde_json::to_string(&cfg).unwrap();
                    let _ = writer.write_all(response.as_bytes()).await;
                    let _ = writer.write_all(b"\n").await;
                    let _ = writer.flush().await;
                }
                Ok(DaemonCommand::UpdateConfig { data }) => {
                    println!("✏️ Config updated: {:?}", data);
                    *config.lock().unwrap() = data.clone();
                    let _ = confy::store("memorybank-daemon", None, &data);
                    let _ = writer.write_all(b"{\"status\":\"ok\"}\n").await;
                    let _ = writer.flush().await;
                }
                Err(e) => {
                    let _ = writer
                        .write_all(format!("{{\"error\":\"{}\"}}\n", e).as_bytes())
                        .await;
                    let _ = writer.flush().await;
                }
            }
        });
    }
}
