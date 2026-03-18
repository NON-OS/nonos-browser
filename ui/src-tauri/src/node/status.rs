use crate::state::AppState;
use crate::types::NodeStatusResponse;
use std::sync::atomic::Ordering;
use tauri::State;

#[tauri::command]
pub async fn node_get_status(state: State<'_, AppState>) -> Result<NodeStatusResponse, String> {
    let nodes = state.nodes.read().await;
    Ok(NodeStatusResponse {
        running: nodes.embedded_running,
        connected_nodes: nodes.nodes.len(),
        quality: nodes.embedded_quality,
        total_requests: nodes.total_requests.load(Ordering::Relaxed),
    })
}
