use crate::proxy::set_proxy_connected;
use crate::state::{AppState, ConnectionStatus};
use crate::types::NetworkStatusResponse;
use tauri::{Emitter, State, Window};

use super::health::socks5_probe;
use super::status;

#[tauri::command]
pub async fn network_disconnect(state: State<'_, AppState>, window: Window) -> Result<(), String> {
    let mut network = state.network.write().await;

    if let Some(pid) = network.nym_pid.take() {
        kill_nym_process(pid).await;
    }

    network.status = ConnectionStatus::Disconnected;
    network.bootstrap_progress = 0;
    network.error = None;
    set_proxy_connected(false);
    status::emit_status(&window, &network);

    Ok(())
}

async fn kill_nym_process(pid: u32) {
    #[cfg(unix)]
    {
        use std::process::Command;
        Command::new("kill")
            .args(["-15", &pid.to_string()])
            .output()
            .ok();
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        Command::new("kill")
            .args(["-9", &pid.to_string()])
            .output()
            .ok();
    }
    #[cfg(windows)]
    {
        use std::process::Command;
        Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string()])
            .output()
            .ok();
    }
}

#[tauri::command]
pub async fn network_get_status(
    state: State<'_, AppState>,
) -> Result<NetworkStatusResponse, String> {
    let mut network = state.network.write().await;

    if !matches!(network.status, ConnectionStatus::Connected)
        && socks5_probe(network.socks_addr).await.is_ok()
    {
        network.status = ConnectionStatus::Connected;
        network.bootstrap_progress = 100;
        set_proxy_connected(true);
    }

    Ok(status::create_response(&network))
}

#[tauri::command]
pub async fn network_new_identity(
    state: State<'_, AppState>,
    window: Window,
) -> Result<(), String> {
    let current_mode = {
        let network = state.network.read().await;
        if !matches!(network.status, ConnectionStatus::Connected) {
            return Err("Not connected".into());
        }
        network.privacy_mode
    };

    network_disconnect(state.clone(), window.clone()).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    super::commands::network_connect(state, window.clone(), Some(current_mode)).await?;

    window
        .emit("nonos://identity-changed", ())
        .map_err(|e| e.to_string())?;

    Ok(())
}
