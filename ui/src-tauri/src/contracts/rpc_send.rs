use super::client::build_client;
use super::config::Network;

pub async fn send_raw_transaction(network: Network, signed_tx: &str) -> Result<String, String> {
    let client = build_client()?;

    for endpoint in network.rpc_endpoints() {
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_sendRawTransaction",
            "params": [signed_tx],
            "id": 1
        });

        if let Ok(response) = client
            .post(*endpoint)
            .header("Content-Type", "application/json")
            .json(&payload)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
        {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                if let Some(result) = json.get("result").and_then(|r| r.as_str()) {
                    return Ok(result.to_string());
                }
                if let Some(error) = json.get("error") {
                    return Err(format!("Transaction failed: {}", error));
                }
            }
        }
    }
    Err(format!("Failed to broadcast on {}", network.name()))
}
