use crate::proxy::set_proxy_connected;
use crate::state::{ConnectionStatus, NetworkState, PrivacyMode};
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::RwLock;

use super::health::{socks5_probe, wait_for_socks};
use super::locate::find_nym_binary;
use super::provider::get_provider;

pub async fn auto_start(network_state: Arc<RwLock<NetworkState>>) -> Result<(), String> {
    let mut network = network_state.write().await;

    if matches!(network.status, ConnectionStatus::Connected) {
        return Ok(());
    }

    if matches!(network.privacy_mode, PrivacyMode::Unsafe) {
        return Err("Cannot auto-start in Unsafe mode".into());
    }

    if socks5_probe(network.socks_addr).await.is_ok() {
        network.status = ConnectionStatus::Connected;
        network.bootstrap_progress = 100;
        set_proxy_connected(true);
        return Ok(());
    }

    network.status = ConnectionStatus::Connecting;
    let socks_addr = network.socks_addr;
    let data_dir = network.data_dir.clone();
    let use_fastmode = network.privacy_mode.use_fastmode();
    drop(network);

    tokio::fs::create_dir_all(&data_dir).await.ok();

    let nym_path = find_nym_binary().await?;
    let client_id = "nonos-client";
    let provider = get_provider().await;
    let port = socks_addr.port().to_string();

    init_client(&nym_path, client_id, &provider).await?;

    let mut network = network_state.write().await;
    network.status = ConnectionStatus::Bootstrapping;
    drop(network);

    let mut args = vec!["run", "--id", client_id, "--port", &port];
    if use_fastmode {
        args.push("--fastmode");
    }

    let child = Command::new(&nym_path)
        .args(&args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start Nym: {}", e))?;

    let mut network = network_state.write().await;
    network.nym_pid = child.id();
    drop(network);

    wait_for_socks(socks_addr, 30).await?;

    let mut network = network_state.write().await;
    network.status = ConnectionStatus::Connected;
    network.bootstrap_progress = 100;
    set_proxy_connected(true);
    Ok(())
}

async fn init_client(
    nym_path: &std::path::Path,
    client_id: &str,
    provider: &str,
) -> Result<(), String> {
    let config = dirs::home_dir()
        .unwrap_or_default()
        .join(".nym/socks5-clients")
        .join(client_id);
    if config.exists() {
        return Ok(());
    }

    let output = Command::new(nym_path)
        .args(["init", "--id", client_id, "--provider", provider])
        .output()
        .await
        .map_err(|e| format!("Init failed: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Init error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}
