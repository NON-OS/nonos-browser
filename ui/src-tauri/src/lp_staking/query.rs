use crate::contracts::{eth_call, Network, PRIVACY_LIQUIDITY_POOL_SEPOLIA};

pub async fn query_contract(data: Vec<u8>) -> Result<String, String> {
    eth_call(
        Network::Sepolia,
        PRIVACY_LIQUIDITY_POOL_SEPOLIA,
        &format!("0x{}", hex::encode(data)),
    )
    .await
}

pub fn parse_u256(hex: &str) -> u128 {
    u128::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0)
}

pub fn format_nox(wei: u128) -> String {
    let nox = wei as f64 / 1e18;
    if nox == 0.0 {
        "0".into()
    } else {
        format!("{:.4}", nox)
    }
}
