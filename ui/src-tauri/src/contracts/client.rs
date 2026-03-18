use super::config::SOCKS_PROXY;

pub fn build_client() -> Result<reqwest::Client, String> {
    let proxy =
        reqwest::Proxy::all(SOCKS_PROXY).map_err(|e| format!("Failed to create proxy: {}", e))?;
    reqwest::Client::builder()
        .proxy(proxy)
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to build proxied client: {}", e))
}
