use super::types::{KNOWN_TRACKERS, NONOS_API_URL};
use crate::state::AppState;
use crate::types::TrackingCheckResponse;
use tauri::State;

#[tauri::command]
pub async fn privacy_check_tracking(
    state: State<'_, AppState>,
    domain: String,
) -> Result<TrackingCheckResponse, String> {
    let nodes = state.nodes.read().await;

    if !nodes.embedded_running {
        let blocked = KNOWN_TRACKERS.iter().any(|t| domain.contains(t));
        return Ok(TrackingCheckResponse {
            domain: domain.clone(),
            blocked,
            reason: if blocked {
                Some("Known tracker domain".into())
            } else {
                None
            },
        });
    }

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/privacy/tracking/check", NONOS_API_URL))
        .json(&serde_json::json!({ "domain": domain }))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to NONOS node: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("NONOS node returned error: {}", response.status()));
    }

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(TrackingCheckResponse {
        domain: domain.clone(),
        blocked: result["blocked"].as_bool().unwrap_or(false),
        reason: result["reason"].as_str().map(String::from),
    })
}

#[tauri::command]
pub async fn privacy_block_domain(
    state: State<'_, AppState>,
    domain: String,
) -> Result<(), String> {
    let nodes = state.nodes.read().await;
    if !nodes.embedded_running {
        return Err("NONOS node not running. Start the node first.".into());
    }

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/privacy/tracking/block", NONOS_API_URL))
        .json(&serde_json::json!({ "domain": domain }))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to NONOS node: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to block domain: {}", response.status()));
    }

    Ok(())
}
