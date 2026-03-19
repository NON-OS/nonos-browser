use super::types::{WorkMetrics, WorkMetricsResponse, DAEMON_API_URL};

pub async fn fetch_work_metrics() -> Result<WorkMetrics, String> {
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
