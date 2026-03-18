use super::types::NONOS_API_URL;
use crate::state::AppState;
use crate::types::ZkIdentityResponse;
use tauri::State;

#[tauri::command]
pub async fn privacy_generate_identity(
    state: State<'_, AppState>,
    name: Option<String>,
) -> Result<ZkIdentityResponse, String> {
    let nodes = state.nodes.read().await;
    if !nodes.embedded_running {
        return Err("NONOS node not running. Start the node first.".into());
    }

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/privacy/identity/register", NONOS_API_URL))
        .json(&serde_json::json!({ "name": name }))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to NONOS node: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to generate identity: {}",
            response.status()
        ));
    }

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(ZkIdentityResponse {
        identity_id: result["identity_id"]
            .as_str()
            .unwrap_or("unknown")
            .to_string(),
        commitment: result["commitment"].as_str().unwrap_or("").to_string(),
        merkle_root: result["merkle_root"].as_str().unwrap_or("").to_string(),
    })
}

#[tauri::command]
pub async fn privacy_get_identity_root(state: State<'_, AppState>) -> Result<String, String> {
    let nodes = state.nodes.read().await;
    if !nodes.embedded_running {
        return Err("NONOS node not running. Start the node first.".into());
    }

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/privacy/identity/root", NONOS_API_URL))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to NONOS node: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to get identity root: {}",
            response.status()
        ));
    }

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    Ok(result["root"].as_str().unwrap_or("").to_string())
}
