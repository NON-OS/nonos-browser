use super::scoring::{
    calculate_entropy_score, calculate_mixer_score, calculate_registry_score,
    calculate_traffic_score, calculate_zk_score,
};
use super::types::{
    EpochInfo, WorkCategoryBreakdown, WorkDashboard, WorkMetrics, WorkMetricsResponse,
    DAEMON_API_URL,
};
use crate::state::AppState;
use serde::Deserialize;
use tauri::State;

async fn fetch_work_metrics() -> Result<WorkMetrics, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let url = format!("{}/api/v1/work/metrics", DAEMON_API_URL);
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to daemon: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Daemon returned error: {}", response.status()));
    }

    let data: WorkMetricsResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    Ok(data.data)
}

#[tauri::command]
pub async fn work_get_metrics(_state: State<'_, AppState>) -> Result<WorkMetrics, String> {
    fetch_work_metrics().await
}

#[tauri::command]
pub async fn work_get_dashboard(_state: State<'_, AppState>) -> Result<WorkDashboard, String> {
    let metrics = fetch_work_metrics().await?;

    let categories = vec![
        WorkCategoryBreakdown {
            name: "Traffic Relay".into(),
            weight: 30,
            score: calculate_traffic_score(&metrics.traffic_relay),
            raw_value: metrics.traffic_relay.bytes_relayed,
        },
        WorkCategoryBreakdown {
            name: "ZK Proofs".into(),
            weight: 25,
            score: calculate_zk_score(&metrics.zk_proofs),
            raw_value: metrics.zk_proofs.proofs_generated + metrics.zk_proofs.proofs_verified,
        },
        WorkCategoryBreakdown {
            name: "Mixer Operations".into(),
            weight: 20,
            score: calculate_mixer_score(&metrics.mixer_ops),
            raw_value: metrics.mixer_ops.deposits_processed + metrics.mixer_ops.spends_processed,
        },
        WorkCategoryBreakdown {
            name: "Entropy".into(),
            weight: 15,
            score: calculate_entropy_score(&metrics.entropy),
            raw_value: metrics.entropy.entropy_bytes_contributed,
        },
        WorkCategoryBreakdown {
            name: "Registry".into(),
            weight: 10,
            score: calculate_registry_score(&metrics.registry_ops),
            raw_value: metrics.registry_ops.registrations_processed
                + metrics.registry_ops.lookups_served,
        },
    ];

    let daily_emission: f64 = 54794.52;
    let node_share = 0.70;
    let estimated_reward = (metrics.total_work_score / 100.0) * daily_emission * 7.0 * node_share;

    Ok(WorkDashboard {
        metrics,
        categories,
        estimated_epoch_reward: format!("{:.2}", estimated_reward),
        network_rank: None,
        network_total_nodes: 0,
    })
}

#[tauri::command]
pub async fn work_get_epoch(_state: State<'_, AppState>) -> Result<EpochInfo, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let url = format!("{}/api/v1/work/epoch", DAEMON_API_URL);
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to daemon: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Daemon returned error: {}", response.status()));
    }

    #[derive(Deserialize)]
    struct EpochResponse {
        epoch: u64,
        epoch_start: u64,
        epoch_end: u64,
        submitted: bool,
    }

    let data: EpochResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    Ok(EpochInfo {
        current_epoch: data.epoch,
        epoch_start_timestamp: data.epoch_start,
        epoch_end_timestamp: data.epoch_end,
        submitted_to_oracle: data.submitted,
    })
}
