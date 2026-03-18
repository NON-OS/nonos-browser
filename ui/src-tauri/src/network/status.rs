use crate::state::{ConnectionStatus, NetworkState};
use crate::types::NetworkStatusResponse;
use tauri::Window;

pub fn create_response(network: &NetworkState) -> NetworkStatusResponse {
    NetworkStatusResponse {
        connected: matches!(network.status, ConnectionStatus::Connected),
        status: format!("{:?}", network.status),
        bootstrap_progress: network.bootstrap_progress,
        circuits: network.circuits,
        socks_port: network.socks_addr.port(),
        error: network.error.clone(),
    }
}

pub fn emit_status(window: &Window, network: &NetworkState) {
    let _ = window.emit("nonos://network-status", create_response(network));
}
