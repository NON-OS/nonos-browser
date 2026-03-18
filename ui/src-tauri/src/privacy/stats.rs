use super::types::NONOS_API_URL;
use crate::state::AppState;
use crate::types::PrivacyStatsResponse;
use tauri::State;

#[tauri::command]
pub async fn privacy_get_stats(state: State<'_, AppState>) -> Result<PrivacyStatsResponse, String> {
    let nodes = state.nodes.read().await;
    if !nodes.embedded_running {
        return Err("NONOS node not running. Start the node first.".into());
    }

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/privacy/stats", NONOS_API_URL))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to NONOS node: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("NONOS node returned error: {}", response.status()));
    }

    let stats: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(PrivacyStatsResponse {
        zk_proofs_issued: stats["zk_proofs_issued"].as_u64().unwrap_or(0),
        zk_verifications: stats["zk_verifications"].as_u64().unwrap_or(0),
        cache_hits: stats["cache_hits"].as_u64().unwrap_or(0),
        cache_misses: stats["cache_misses"].as_u64().unwrap_or(0),
        cache_hit_rate: stats["cache_hit_rate"].as_f64().unwrap_or(0.0),
        tracking_blocked: stats["tracking_blocked"].as_u64().unwrap_or(0),
        tracking_total: stats["tracking_total"].as_u64().unwrap_or(0),
        block_rate: stats["block_rate"].as_f64().unwrap_or(0.0),
        stealth_payments: stats["stealth_payments"].as_u64().unwrap_or(0),
        stealth_scanned: stats["stealth_scanned"].as_u64().unwrap_or(0),
    })
}
