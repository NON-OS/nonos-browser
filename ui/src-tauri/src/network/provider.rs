use serde::Deserialize;
use std::collections::HashSet;

const HARBOURMASTER_API: &str = "https://harbourmaster.nymtech.net/v2/services";
const NYM_NODES_API: &str = "https://validator.nymtech.net/api/v1/nym-nodes/described";

#[derive(Deserialize)]
struct HarbourmasterResponse {
    items: Vec<ServiceProvider>,
}

#[derive(Deserialize)]
struct ServiceProvider {
    service_provider_client_id: String,
    gateway_identity_key: String,
}

#[derive(Deserialize)]
struct NymNodesResponse {
    data: Vec<NymNode>,
}

#[derive(Deserialize)]
struct NymNode {
    description: NodeDescription,
}

#[derive(Deserialize)]
struct NodeDescription {
    host_information: HostInfo,
}

#[derive(Deserialize)]
struct HostInfo {
    keys: NodeKeys,
}

#[derive(Deserialize)]
struct NodeKeys {
    ed25519: String,
}

async fn fetch_active_gateways(client: &reqwest::Client) -> Result<HashSet<String>, String> {
    let response = client
        .get(NYM_NODES_API)
        .send()
        .await
        .map_err(|e| format!("Nodes API request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Nodes API returned {}", response.status()));
    }

    let nodes: NymNodesResponse = response
        .json()
        .await
        .map_err(|e| format!("Nodes JSON parse failed: {}", e))?;

    let gateways: HashSet<String> = nodes
        .data
        .into_iter()
        .map(|n| n.description.host_information.keys.ed25519)
        .collect();

    Ok(gateways)
}

pub async fn fetch_active_provider() -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let active_gateways = fetch_active_gateways(&client).await?;

    if active_gateways.is_empty() {
        return Err("No active gateways found".to_string());
    }

    let response = client
        .get(HARBOURMASTER_API)
        .send()
        .await
        .map_err(|e| format!("Harbourmaster request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Harbourmaster returned {}", response.status()));
    }

    let harbourmaster: HarbourmasterResponse = response
        .json()
        .await
        .map_err(|e| format!("Harbourmaster JSON parse failed: {}", e))?;

    for provider in harbourmaster.items {
        if active_gateways.contains(&provider.gateway_identity_key) {
            return Ok(provider.service_provider_client_id);
        }
    }

    Err("No service provider with active gateway found".to_string())
}

pub fn default_provider() -> &'static str {
    "HuNL1pFprNSKW6jdqppibXP5KNKCNJxDh7ivpYcoULN9.C62NahRTUf6kqpNtDVHXoVriQr6yyaU5LtxdgpbsGrtA@23A7CSaBSA2L67PWuFTPXUnYrCdyVcB7ATYsjUsfdftb"
}

pub async fn get_provider() -> String {
    match fetch_active_provider().await {
        Ok(p) => p,
        Err(_) => default_provider().to_string(),
    }
}
