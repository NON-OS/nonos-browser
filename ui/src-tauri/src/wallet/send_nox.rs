use super::state::WALLET_MANAGER;
use crate::contracts::{
    fetch_mainnet_balances, send_on_network, Network, NOX_TOKEN_ADDRESS_MAINNET,
};
use crate::helpers::{validate_amount, validate_eth_address};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn wallet_send_nox(
    _state: State<'_, AppState>,
    to: String,
    amount: String,
) -> Result<String, String> {
    let manager = WALLET_MANAGER.read().await;
    let wallet = manager.active().ok_or("Wallet not unlocked")?;
    if !wallet.is_unlocked() {
        return Err("Wallet is locked".into());
    }

    let private_key = wallet
        .get_account_private_key(0)
        .map_err(|e| format!("Key: {}", e))?;
    let from_address = wallet.address().to_hex();
    drop(manager);

    validate_eth_address(&to).map_err(|e| e.to_string())?;
    let amount_nox = validate_amount(&amount).map_err(|e| e.to_string())?;
    let amount_wei = (amount_nox * 1e18) as u128;

    let (eth_balance, nox_balance) = fetch_mainnet_balances(&from_address).await;

    if amount_wei > nox_balance {
        return Err(format!(
            "Insufficient NOX. Have {}, sending {}",
            nox_balance as f64 / 1e18,
            amount_nox
        ));
    }

    if eth_balance < 3_000_000_000_000_000u128 {
        return Err(format!(
            "Insufficient ETH for gas. Have {} ETH",
            eth_balance as f64 / 1e18
        ));
    }

    let to_addr = to.trim_start_matches("0x").to_lowercase();
    let data = hex::decode(format!(
        "a9059cbb{:0>64}{:0>64}",
        to_addr,
        format!("{:x}", amount_wei)
    ))
    .map_err(|_| "Encode error")?;

    send_on_network(
        Network::Mainnet,
        &private_key,
        NOX_TOKEN_ADDRESS_MAINNET,
        0,
        data,
        100000,
    )
    .await
}
