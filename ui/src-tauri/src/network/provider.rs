use serde::Deserialize;

const NYM_API: &str = "https://validator.nymtech.net/api/v1/gateways";

#[derive(Deserialize)]
struct Gateway {
    gateway: GatewayInfo,
}

#[derive(Deserialize)]
struct GatewayInfo {
    identity_key: String,
    sphinx_key: String,
    host: String,
}

pub async fn fetch_active_provider() -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let response = client
        .get(NYM_API)
        .send()
        .await
        .map_err(|e| format!("API request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API returned {}", response.status()));
    }

    let gateways: Vec<Gateway> = response
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))?;

    let gateway = gateways.first().ok_or("No active gateways found")?;

    Ok(format!(
        "{}.{}@{}",
        gateway.gateway.identity_key, gateway.gateway.sphinx_key, gateway.gateway.host
    ))
}

pub fn default_provider() -> &'static str {
    "4yRfauFzZnejJhG2FACTVQ7UnYEcFUYw3HzXrmuwLMaR.Bk85p86AEbkAR73wvJrqGKnWUq1okLPJatFwxsaDWpvE@EBT8jTD8o4tKng2NXrrcrzVhJiBnKpT1bJy5CMeArt2w"
}

pub async fn get_provider() -> String {
    match fetch_active_provider().await {
        Ok(p) => p,
        Err(_) => default_provider().to_string(),
    }
}
