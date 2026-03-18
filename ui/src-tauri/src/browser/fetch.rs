use crate::state::{AppState, ConnectionStatus};
use crate::types::ProxyFetchResponse;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use tauri::State;

#[tauri::command]
pub async fn proxy_fetch(
    state: State<'_, AppState>,
    url: String,
    method: Option<String>,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> Result<ProxyFetchResponse, String> {
    let network = state.network.read().await;
    let socks_addr = network.socks_addr;
    let is_connected = matches!(network.status, ConnectionStatus::Connected);
    drop(network);

    {
        state
            .nodes
            .read()
            .await
            .total_requests
            .fetch_add(1, Ordering::Relaxed);
    }

    if !is_connected {
        return Err(
            "Network not connected. All requests must route through Anyone Network.".into(),
        );
    }

    let proxy = reqwest::Proxy::all(format!("socks5h://{}", socks_addr))
        .map_err(|e| format!("Failed to create proxy: {}", e))?;
    let client = reqwest::Client::builder()
        .proxy(proxy)
        .danger_accept_invalid_certs(false)
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| format!("Failed to build proxy client: {}", e))?;

    let method_str = method.unwrap_or_else(|| "GET".to_string());
    let method =
        reqwest::Method::from_bytes(method_str.as_bytes()).map_err(|_| "Invalid HTTP method")?;

    let mut request = client.request(method, &url);
    if let Some(hdrs) = headers {
        for (k, v) in hdrs {
            request = request.header(&k, &v);
        }
    }
    if let Some(b) = body {
        request = request.body(b);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    let status_code = response.status().as_u16();
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("text/html")
        .to_string();

    let mut resp_headers = HashMap::new();
    for (key, value) in response.headers() {
        if let Ok(v) = value.to_str() {
            resp_headers.insert(key.to_string(), v.to_string());
        }
    }
    let body_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(ProxyFetchResponse {
        success: (200..400).contains(&status_code),
        status_code,
        headers: resp_headers,
        body: body_text,
        content_type,
        via_proxy: is_connected,
        circuit_id: Some("anon-circuit-1".to_string()),
    })
}
