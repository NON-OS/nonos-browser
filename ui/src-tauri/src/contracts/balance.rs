use super::{
    client::build_verified_client,
    config::{Network, BALANCE_OF_SELECTOR, NOX_TOKEN_ADDRESS_MAINNET, NOX_TOKEN_ADDRESS_SEPOLIA},
    rpc::eth_call,
};

pub async fn get_eth_balance(network: Network, address: &str) -> Result<u128, String> {
    let client = build_verified_client().await?;

    for endpoint in network.rpc_endpoints() {
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getBalance",
            "params": [address, "latest"],
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
    Err(format!("Failed to get ETH balance on {}", network.name()))
}

pub async fn get_token_balance(
    network: Network,
    token: &str,
    holder: &str,
) -> Result<u128, String> {
    let addr = holder.trim_start_matches("0x").to_lowercase();
    let padded = format!("{:0>64}", addr);
    let data = format!("0x{}{}", BALANCE_OF_SELECTOR, padded);

    let result = eth_call(network, token, &data).await?;
    let hex = result.trim_start_matches("0x");

    if hex.is_empty() || hex.chars().all(|c| c == '0') {
        return Ok(0);
    }
    u128::from_str_radix(hex, 16).map_err(|e| format!("Parse error: {}", e))
}

pub async fn fetch_mainnet_balances(address: &str) -> Result<(u128, u128), String> {
    let eth = get_eth_balance(Network::Mainnet, address).await?;
    let nox = get_token_balance(Network::Mainnet, NOX_TOKEN_ADDRESS_MAINNET, address).await?;
    Ok((eth, nox))
}

pub async fn fetch_sepolia_balances(address: &str) -> Result<(u128, u128), String> {
    let eth = get_eth_balance(Network::Sepolia, address).await?;
    let nox = get_token_balance(Network::Sepolia, NOX_TOKEN_ADDRESS_SEPOLIA, address).await?;
    Ok((eth, nox))
}
