use super::{client::build_client, config::Network};

pub async fn eth_call(network: Network, to: &str, data: &str) -> Result<String, String> {
    let client = build_client()?;

    for endpoint in network.rpc_endpoints() {
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_call",
            "params": [{"to": to, "data": data}, "latest"],
            "id": 1
        });

        if let Ok(response) = client
            .post(*endpoint)
            .header("Content-Type", "application/json")
            .json(&payload)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                if let Some(result) = json.get("result").and_then(|r| r.as_str()) {
                    return Ok(result.to_string());
                }
                if json.get("error").is_some() {
                    continue;
                }
            }
        }
    }
    Err(format!("All {} RPC endpoints failed", network.name()))
}

pub async fn get_nonce(network: Network, address: &str) -> Result<u64, String> {
    let client = build_client()?;

    for endpoint in network.rpc_endpoints() {
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getTransactionCount",
            "params": [address, "pending"],
            "id": 1
        });

        if let Ok(response) = client
            .post(*endpoint)
            .header("Content-Type", "application/json")
            .json(&payload)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                if let Some(result) = json.get("result").and_then(|r| r.as_str()) {
                    let hex = result.trim_start_matches("0x");
                    return u64::from_str_radix(hex, 16).map_err(|e| format!("Parse error: {}", e));
                }
            }
        }
    }
    Err(format!("Failed to get nonce on {}", network.name()))
}

pub async fn get_gas_price(network: Network) -> Result<u128, String> {
    let client = build_client()?;

    for endpoint in network.rpc_endpoints() {
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_gasPrice",
            "params": [],
            "id": 1
        });

        if let Ok(response) = client
            .post(*endpoint)
            .header("Content-Type", "application/json")
            .json(&payload)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                if let Some(result) = json.get("result").and_then(|r| r.as_str()) {
                    let hex = result.trim_start_matches("0x");
                    return u128::from_str_radix(hex, 16)
                        .map_err(|e| format!("Parse error: {}", e));
                }
            }
        }
    }
    Err(format!("Failed to get gas price on {}", network.name()))
}

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
    Err(format!(
        "Failed to broadcast transaction on {}",
        network.name()
    ))
}
