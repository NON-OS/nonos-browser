use crate::proxy::set_proxy_connected;
use crate::state::{ConnectionStatus, NetworkState};
use std::net::SocketAddr;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::sync::RwLock;

use super::binary::find_nym_binary;

pub async fn auto_start(network_state: Arc<RwLock<NetworkState>>) -> Result<(), String> {
    let mut network = network_state.write().await;

    if matches!(
        network.status,
        ConnectionStatus::Connected | ConnectionStatus::Connecting
    ) {
        return Ok(());
    }

    if TcpStream::connect(network.socks_addr).await.is_ok() {
        network.status = ConnectionStatus::Connected;
        network.bootstrap_progress = 100;
        network.circuits = 3;
        set_proxy_connected(true);
        return Ok(());
    }

    network.status = ConnectionStatus::Connecting;

    if !network.data_dir.exists() {
        tokio::fs::create_dir_all(&network.data_dir)
            .await
            .map_err(|e| format!("Failed to create data dir: {}", e))?;
    }

    let nym_path = match find_nym_binary().await {
        Ok(path) => path,
        Err(e) => {
            network.status = ConnectionStatus::Disconnected;
            network.error = Some("nym-socks5-client not installed".into());
            return Err(e);
        }
    };

    let client_id = "nonos-client".to_string();
    network.status = ConnectionStatus::Bootstrapping;

    let config_path = dirs::home_dir()
        .unwrap_or_default()
        .join(".nym/socks5-clients")
        .join(&client_id);

    if !config_path.exists() {
        drop(network);
        let _ = Command::new(&nym_path)
            .args([
                "init",
                "--id",
                &client_id,
                "--provider",
                "4yRfauFzZnejJhG2FACTVQ7UnYEcFUYw3HzXrmuwLMaR.Bk85p86AEbkAR73wvJrqGKnWUq1okLPJatFwxsaDWpvE@EBT8jTD8o4tKng2NXrrcrzVhJiBnKpT1bJy5CMeArt2w",
            ])
            .output()
            .await;
        network = network_state.write().await;
    }

    let mut child = Command::new(&nym_path)
        .args([
            "run",
            "--id",
            &client_id,
            "--port",
            &network.socks_addr.port().to_string(),
            "--fastmode",
            "--no-cover",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to launch nym-socks5-client: {}", e))?;

    network.nym_pid = child.id();
    let socks_addr = network.socks_addr;
    drop(network);

    spawn_output_reader(child.stdout.take(), network_state.clone());
    spawn_connection_checker(socks_addr, network_state);

    Ok(())
}

fn spawn_output_reader(
    stdout: Option<tokio::process::ChildStdout>,
    network_state: Arc<RwLock<NetworkState>>,
) {
    if let Some(stdout) = stdout {
        tokio::spawn(async move {
            let mut lines = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if line.contains("connected")
                    || line.contains("listening")
                    || line.contains("ready")
                {
                    let mut net = network_state.write().await;
                    net.status = ConnectionStatus::Connected;
                    net.bootstrap_progress = 100;
                    net.circuits = 3;
                    set_proxy_connected(true);
                }
            }
        });
    }
}

fn spawn_connection_checker(socks_addr: SocketAddr, network_state: Arc<RwLock<NetworkState>>) {
    tokio::spawn(async move {
        for _ in 0..30 {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            let net = network_state.read().await;
            if matches!(net.status, ConnectionStatus::Connected) {
                return;
            }
            drop(net);

            if TcpStream::connect(socks_addr).await.is_ok() {
                let mut net = network_state.write().await;
                if !matches!(net.status, ConnectionStatus::Connected) {
                    net.status = ConnectionStatus::Connected;
                    net.bootstrap_progress = 100;
                    net.circuits = 3;
                    set_proxy_connected(true);
                }
                return;
            }
        }
    });
}
