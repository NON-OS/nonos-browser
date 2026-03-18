use super::state::WALLET_MANAGER;
use crate::contracts::{
    fetch_mainnet_balances, get_gas_price, send_mainnet, send_on_network, Network,
    NOX_TOKEN_ADDRESS_MAINNET,
};
use crate::helpers::{validate_amount, validate_eth_address};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn wallet_send_eth(
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
        .map_err(|e| format!("Failed to get signing key: {}", e))?;
    let from_address = wallet.address().to_hex();
    drop(manager);

    validate_eth_address(&to)?;
    let amount_eth = validate_amount(&amount)?;
    let amount_wei = (amount_eth * 1e18) as u128;

    let (eth_balance, _) = fetch_mainnet_balances(&from_address).await;
    if amount_wei > eth_balance {
        return Err(format!(
            "Insufficient ETH balance. Have {} ETH, sending {} ETH",
            eth_balance as f64 / 1e18,
            amount_eth
        ));
    }

    let gas_price = get_gas_price(Network::Mainnet).await?;
    let gas_limit = 21000u64;
    let gas_cost = gas_price * gas_limit as u128;

    if amount_wei + gas_cost > eth_balance {
        return Err(format!(
            "Insufficient ETH for amount + gas. Have {} ETH, need {} ETH",
            eth_balance as f64 / 1e18,
            (amount_wei + gas_cost) as f64 / 1e18
        ));
    }

    send_mainnet(&private_key, &to, amount_wei, vec![]).await
}

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
        .map_err(|e| format!("Failed to get signing key: {}", e))?;
    let from_address = wallet.address().to_hex();
    drop(manager);

    validate_eth_address(&to)?;
    let amount_nox = validate_amount(&amount)?;
    let amount_wei = (amount_nox * 1e18) as u128;

    let (eth_balance, nox_balance) = fetch_mainnet_balances(&from_address).await;

    if amount_wei > nox_balance {
        return Err(format!(
            "Insufficient NOX balance. Have {} NOX, sending {} NOX",
            nox_balance as f64 / 1e18,
            amount_nox
        ));
    }

    let min_gas_eth = 3_000_000_000_000_000u128;
    if eth_balance < min_gas_eth {
        return Err(format!(
            "Insufficient ETH for gas. Have {} ETH, need at least 0.003 ETH",
            eth_balance as f64 / 1e18
        ));
    }

    let to_addr = to.trim_start_matches("0x").to_lowercase();
    let padded_to = format!("{:0>64}", to_addr);
    let padded_amount = format!("{:0>64}", format!("{:x}", amount_wei));
    let data = hex::decode(format!("a9059cbb{}{}", padded_to, padded_amount))
        .map_err(|_| "Failed to encode transfer data")?;

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
