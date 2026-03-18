use crate::state::{AppState, NodeInfo};
use tauri::State;

#[tauri::command]
pub async fn node_get_connected(state: State<'_, AppState>) -> Result<Vec<NodeInfo>, String> {
    let nodes = state.nodes.read().await;
    Ok(nodes.nodes.clone())
}
