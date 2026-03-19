use crate::contracts::abi;
use crate::contracts::{
    fetch_sepolia_balances, send_sepolia, NOX_STAKING_POOL_SEPOLIA, NOX_TOKEN_ADDRESS_SEPOLIA,
};
use crate::helpers::validate_amount;
use crate::state::AppState;
use crate::wallet::state::WALLET_MANAGER;
use tauri::State;

#[tauri::command]
pub async fn staking_stake(_state: State<'_, AppState>, amount: String) -> Result<String, String> {
    let (private_key, address) = get_wallet_key().await?;
    let amount_nox = validate_amount(&amount).map_err(|e| e.to_string())?;
    let amount_wei = (amount_nox * 1e18) as u128;

    let (_, balance) = fetch_sepolia_balances(&address).await?;
    if amount_wei > balance {
        return Err(format!(
            "Insufficient balance: have {} NOX",
            balance as f64 / 1e18
        ));
    }

    let approve = abi::encode_approve(NOX_STAKING_POOL_SEPOLIA, amount_wei);
    send_sepolia(&private_key, NOX_TOKEN_ADDRESS_SEPOLIA, 0, approve, 100000).await?;

    let stake = abi::encode_stake(amount_wei);
    let tx = send_sepolia(&private_key, NOX_STAKING_POOL_SEPOLIA, 0, stake, 150000).await?;

    Ok(format!("Staked {} NOX. Tx: {}", amount_nox, tx))
}

#[tauri::command]
pub async fn staking_unstake(
    _state: State<'_, AppState>,
    amount: String,
) -> Result<String, String> {
    let (private_key, _) = get_wallet_key().await?;
    let amount_nox = validate_amount(&amount).map_err(|e| e.to_string())?;
    let amount_wei = (amount_nox * 1e18) as u128;

    let unstake = abi::encode_unstake(amount_wei);
    let tx = send_sepolia(&private_key, NOX_STAKING_POOL_SEPOLIA, 0, unstake, 150000).await?;

    Ok(format!(
        "Unstake initiated for {} NOX (14-day unbonding). Tx: {}",
        amount_nox, tx
    ))
}

#[tauri::command]
pub async fn staking_claim_rewards(_state: State<'_, AppState>) -> Result<String, String> {
    let (private_key, _) = get_wallet_key().await?;

    let claim = abi::encode_claim();
    let tx = send_sepolia(&private_key, NOX_STAKING_POOL_SEPOLIA, 0, claim, 150000).await?;

    Ok(format!("Rewards claimed. Tx: {}", tx))
}

#[tauri::command]
pub async fn staking_withdraw(_state: State<'_, AppState>) -> Result<String, String> {
    let (private_key, _) = get_wallet_key().await?;

    let withdraw = abi::function_selector("withdraw()").to_vec();
    let tx = send_sepolia(&private_key, NOX_STAKING_POOL_SEPOLIA, 0, withdraw, 150000).await?;

    Ok(format!("Withdrawal complete. Tx: {}", tx))
}

async fn get_wallet_key() -> Result<(String, String), String> {
    let manager = WALLET_MANAGER.read().await;
    let wallet = manager.active().ok_or("Wallet not unlocked")?;
    if !wallet.is_unlocked() {
        return Err("Wallet is locked".into());
    }
    let key = wallet
        .get_account_private_key(0)
        .map_err(|e| format!("Key error: {}", e))?;
    let addr = wallet.address().to_hex();
    Ok((key, addr))
}
