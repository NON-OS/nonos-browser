use crate::helpers::parse_bootstrap_progress;
use crate::proxy::set_proxy_connected;
use crate::state::{AppState, ConnectionStatus};
use crate::types::NetworkStatusResponse;
use std::process::Stdio;
use tauri::{State, Window};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::process::Command;

use super::{binary::find_nym_binary, status};

#[tauri::command]
pub async fn network_connect(
    state: State<'_, AppState>,
    window: Window,
) -> Result<NetworkStatusResponse, String> {
    let mut network = state.network.write().await;

    if matches!(network.status, ConnectionStatus::Connected) {
        return Ok(status::create_response(&network));
    }

    network.status = ConnectionStatus::Connecting;
    network.error = None;
    status::emit_status(&window, &network);

    if !network.data_dir.exists() {
        tokio::fs::create_dir_all(&network.data_dir)
            .await
            .map_err(|e| format!("Failed to create data dir: {}", e))?;
    }

    let nym_path = find_nym_binary()
        .await
        .map_err(|e| format!("nym-socks5-client not found: {}", e))?;

    network.status = ConnectionStatus::Bootstrapping;
    status::emit_status(&window, &network);

    let client_id = format!(
        "nonos-{}",
        uuid::Uuid::new_v4()
            .to_string()
            .split('-')
            .next()
            .unwrap_or("client")
    );

    let mut child = Command::new(&nym_path)
        .args([
            "run",
            "--id",
            &client_id,
            "--port",
            &network.socks_addr.port().to_string(),
            "--two-hop",
            "--fastmode",
            "--no-cover",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to launch nym-socks5-client: {}", e))?;

    network.nym_pid = child.id();

    let window_clone = window.clone();
    let network_state = state.network.clone();

    if let Some(stdout) = child.stdout.take() {
        tokio::spawn(async move {
            let mut lines = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if let Some(progress) = parse_bootstrap_progress(&line) {
                    let mut net = network_state.write().await;
                    net.bootstrap_progress = progress;
                    if progress >= 100 {
                        net.status = ConnectionStatus::Connected;
                        net.circuits = 3;
                        set_proxy_connected(true);
                    }
                    status::emit_status(&window_clone, &net);
                    continue;
                }

                if line.contains("connected")
                    || line.contains("listening")
                    || line.contains("ready")
                {
                    let mut net = network_state.write().await;
                    net.status = ConnectionStatus::Connected;
                    net.bootstrap_progress = 100;
                    net.circuits = 3;
                    set_proxy_connected(true);
                    status::emit_status(&window_clone, &net);
                }
                if line.contains("error") || line.contains("failed") {
                    let mut net = network_state.write().await;
                    net.status = ConnectionStatus::Error;
                    net.error = Some(line.clone());
                    status::emit_status(&window_clone, &net);
                }
            }
        });
    }

    let socks_addr = network.socks_addr;
    drop(network);

    for _ in 0..60 {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let net = state.network.read().await;
        if matches!(net.status, ConnectionStatus::Connected) {
            return Ok(status::create_response(&net));
        }
        if matches!(net.status, ConnectionStatus::Error) {
            return Err(net.error.clone().unwrap_or_else(|| "Unknown error".into()));
        }
        drop(net);

        if TcpStream::connect(socks_addr).await.is_ok() {
            let mut net = state.network.write().await;
            net.status = ConnectionStatus::Connected;
            net.bootstrap_progress = 100;
            net.circuits = 3;
            set_proxy_connected(true);
            status::emit_status(&window, &net);
            return Ok(status::create_response(&net));
        }
    }

    Err("Nym connection timeout".into())
}
