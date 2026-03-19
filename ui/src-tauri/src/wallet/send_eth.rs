use super::state::WALLET_MANAGER;
use crate::contracts::{fetch_mainnet_balances, get_gas_price, send_mainnet, Network};
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
        .map_err(|e| format!("Key: {}", e))?;
    let from_address = wallet.address().to_hex();
    drop(manager);

    validate_eth_address(&to).map_err(|e| e.to_string())?;
    let amount_eth = validate_amount(&amount).map_err(|e| e.to_string())?;
    let amount_wei = (amount_eth * 1e18) as u128;

    let (eth_balance, _) = fetch_mainnet_balances(&from_address).await;
    if amount_wei > eth_balance {
        return Err(format!(
            "Insufficient ETH. Have {}, sending {}",
            eth_balance as f64 / 1e18,
            amount_eth
        ));
    }

    let gas_price = get_gas_price(Network::Mainnet).await?;
    let gas_cost = gas_price * 21000u128;

    if amount_wei + gas_cost > eth_balance {
        return Err(format!(
            "Insufficient ETH for gas. Have {}",
            eth_balance as f64 / 1e18
        ));
    }

    send_mainnet(&private_key, &to, amount_wei, vec![]).await
}
