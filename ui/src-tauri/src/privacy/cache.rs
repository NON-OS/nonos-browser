use super::types::NONOS_API_URL;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn privacy_cache_store(
    state: State<'_, AppState>,
    content: String,
) -> Result<String, String> {
    let nodes = state.nodes.read().await;
    if !nodes.embedded_running {
        return Err("NONOS node not running. Start the node first.".into());
    }

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/privacy/cache/store", NONOS_API_URL))
        .json(&serde_json::json!({ "content": content }))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to NONOS node: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to store in cache: {}", response.status()));
    }

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    Ok(result["commitment"].as_str().unwrap_or("").to_string())
}
