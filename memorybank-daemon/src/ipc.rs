use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{UnixListener, UnixStream};
use crate::commands::DaemonCommand;
use crate::config::{load_config, store_config, Config};
/// Start the IPC server on the given `socket_path`.  
/// This is an **async** function and must be `.await`‑ed from `main`.
pub async fn start_ipc_server(
    socket_path: &str,
    _shared_config: Arc<Mutex<Config>>, // reserved for future use
) -> anyhow::Result<()> {
    // Ensure stale socket is removed first.
    if Path::new(socket_path).exists() {
        std::fs::remove_file(socket_path)?;
    }

    let listener = UnixListener::bind(socket_path)?;
    println!("🛰️  IPC server listening on {}", socket_path);

    loop {
        let (stream, _) = match listener.accept().await {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Listener error: {e}");
                break Err(anyhow::anyhow!(e));
            }
        };

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                eprintln!("IPC client error: {e}");
            }
        });
    }
}

async fn handle_client(mut stream: UnixStream) -> anyhow::Result<()> {
    let (reader, writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        let bytes_read = reader.read_line(&mut buffer).await?;
        if bytes_read == 0 {
            // Client closed the connection
            break;
        }

        match serde_json::from_str::<DaemonCommand>(&buffer) {
            Ok(DaemonCommand::GetConfig) => {
                let cfg = load_config();
                let json = serde_json::to_string(&cfg)?;
                writer.write_all(format!("{}\n", json).as_bytes()).await?;
            }
            Ok(DaemonCommand::UpdateConfig { data }) => {
                store_config(&data);
                writer.write_all(b"{\"status\":\"ok\"}\n").await?;
            }
            Err(e) => {
                let msg = format!("{{\"error\":\"{}\"}}\n", e);
                writer.write_all(msg.as_bytes()).await?;
            }
        }
        writer.flush().await?;
    }

    Ok(())
}