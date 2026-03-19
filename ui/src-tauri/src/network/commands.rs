use crate::proxy::set_proxy_connected;
use crate::state::{AppState, ConnectionStatus, PrivacyMode};
use crate::types::NetworkStatusResponse;
use tauri::{State, Window};

use super::health::{socks5_probe, wait_for_socks};
use super::locate::find_nym_binary;
use super::provider::get_provider;
use super::status;
use std::process::Stdio;
use tokio::process::Command;

#[tauri::command]
pub async fn network_connect(
    state: State<'_, AppState>,
    window: Window,
    privacy_mode: Option<PrivacyMode>,
) -> Result<NetworkStatusResponse, String> {
    let mode = privacy_mode.unwrap_or_default();

    if matches!(mode, PrivacyMode::Unsafe) {
        return Err("Unsafe mode (clearnet) is disabled for security".into());
    }

    let mut network = state.network.write().await;
    network.privacy_mode = mode;

    if matches!(network.status, ConnectionStatus::Connected) {
        return Ok(status::create_response(&network));
    }

    if socks5_probe(network.socks_addr).await.is_ok() {
        network.status = ConnectionStatus::Connected;
        network.bootstrap_progress = 100;
        set_proxy_connected(true);
        status::emit_status(&window, &network);
        return Ok(status::create_response(&network));
    }

    network.status = ConnectionStatus::Connecting;
    network.error = None;
    status::emit_status(&window, &network);

    let socks_addr = network.socks_addr;
    let data_dir = network.data_dir.clone();
    let use_fastmode = mode.use_fastmode();
    drop(network);

    tokio::fs::create_dir_all(&data_dir).await.ok();

    let nym_path = find_nym_binary().await?;

    let mut network = state.network.write().await;
    network.status = ConnectionStatus::Bootstrapping;
    status::emit_status(&window, &network);
    drop(network);

    let client_id = "nonos-client";
    let provider = get_provider().await;
    let port = socks_addr.port().to_string();

    init_client(&nym_path, client_id, &provider).await?;

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

    let mut network = state.network.write().await;
    network.nym_pid = child.id();
    drop(network);

    wait_for_socks(socks_addr, 30).await?;

    let mut network = state.network.write().await;
    network.status = ConnectionStatus::Connected;
    network.bootstrap_progress = 100;
    set_proxy_connected(true);
    status::emit_status(&window, &network);

    Ok(status::create_response(&network))
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

#[tauri::command]
pub async fn network_get_privacy_mode(state: State<'_, AppState>) -> Result<PrivacyMode, String> {
    let network = state.network.read().await;
    Ok(network.privacy_mode)
}

#[tauri::command]
pub async fn network_set_privacy_mode(
    state: State<'_, AppState>,
    mode: PrivacyMode,
) -> Result<(), String> {
    if matches!(mode, PrivacyMode::Unsafe) {
        return Err("Unsafe mode is disabled for security".into());
    }

    let mut network = state.network.write().await;
    network.privacy_mode = mode;
    Ok(())
}
