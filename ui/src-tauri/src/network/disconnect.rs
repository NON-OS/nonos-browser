use crate::proxy::set_proxy_connected;
use crate::state::{AppState, ConnectionStatus};
use crate::types::NetworkStatusResponse;
use tauri::{State, Window};

use super::status;

#[tauri::command]
pub async fn network_disconnect(state: State<'_, AppState>, window: Window) -> Result<(), String> {
    let mut network = state.network.write().await;

    if let Some(pid) = network.nym_pid.take() {
        #[cfg(unix)]
        {
            use std::process::Command as StdCommand;
            let _ = StdCommand::new("kill").arg(pid.to_string()).output();
        }
        #[cfg(windows)]
        {
            use std::process::Command as StdCommand;
            let _ = StdCommand::new("taskkill")
                .args(&["/PID", &pid.to_string(), "/F"])
                .output();
        }
    }

    network.status = ConnectionStatus::Disconnected;
    network.bootstrap_progress = 0;
    network.circuits = 0;
    network.error = None;
    set_proxy_connected(false);

    status::emit_status(&window, &network);
    Ok(())
}

#[tauri::command]
pub async fn network_get_status(
    state: State<'_, AppState>,
) -> Result<NetworkStatusResponse, String> {
    let network = state.network.read().await;
    Ok(status::create_response(&network))
}

#[tauri::command]
pub async fn network_new_identity(
    state: State<'_, AppState>,
    window: Window,
) -> Result<(), String> {
    let network = state.network.read().await;

    if !matches!(network.status, ConnectionStatus::Connected) {
        return Err("Not connected".into());
    }

    drop(network);

    network_disconnect(state.clone(), window.clone()).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    super::commands::network_connect(state, window.clone()).await?;

    window
        .emit("nonos://identity-changed", ())
        .map_err(|e| e.to_string())?;

    Ok(())
}
