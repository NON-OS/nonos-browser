use crate::contracts::{
    fetch_sepolia_balances, send_sepolia, NOX_STAKING_POOL_SEPOLIA, NOX_TOKEN_ADDRESS_SEPOLIA,
};
use crate::helpers::validate_amount;
use crate::state::AppState;
use crate::wallet::state::WALLET_MANAGER;
use tauri::State;

#[tauri::command]
pub async fn staking_stake(_state: State<'_, AppState>, amount: String) -> Result<String, String> {
    let manager = WALLET_MANAGER.read().await;
    let wallet = manager.active().ok_or("Wallet not unlocked")?;
    if !wallet.is_unlocked() {
        return Err("Wallet is locked".into());
    }

    let private_key = wallet
        .get_account_private_key(0)
        .map_err(|e| format!("Failed to get signing key: {}", e))?;
    let address = wallet.address().to_hex();
    drop(manager);

    let amount_nox = validate_amount(&amount)?;
    let amount_wei = (amount_nox * 1e18) as u128;

    let (_, sepolia_nox) = fetch_sepolia_balances(&address).await;
    if amount_wei > sepolia_nox {
        return Err(format!(
            "Insufficient Sepolia NOX balance. Have {} NOX, staking {} NOX",
            sepolia_nox as f64 / 1e18,
            amount_nox
        ));
    }

    let approve_data = format!(
        "0x095ea7b3{:0>64}{:0>64}",
        NOX_STAKING_POOL_SEPOLIA.trim_start_matches("0x"),
        format!("{:x}", amount_wei)
    );
    let approve_data_bytes = hex::decode(approve_data.trim_start_matches("0x"))
        .map_err(|e| format!("Encode error: {}", e))?;
    let _ = send_sepolia(
        &private_key,
        NOX_TOKEN_ADDRESS_SEPOLIA,
        0,
        approve_data_bytes,
        100000,
    )
    .await?;

    let stake_data = format!("0xa694fc3a{:0>64}", format!("{:x}", amount_wei));
    let stake_data_bytes = hex::decode(stake_data.trim_start_matches("0x"))
        .map_err(|e| format!("Encode error: {}", e))?;
    let stake_tx = send_sepolia(
        &private_key,
        NOX_STAKING_POOL_SEPOLIA,
        0,
        stake_data_bytes,
        150000,
    )
    .await?;

    Ok(format!(
        "Staked {} NOX on Sepolia! Tx: {}",
        amount_nox, stake_tx
    ))
}

#[tauri::command]
pub async fn staking_unstake(
    _state: State<'_, AppState>,
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
    drop(manager);

    let amount_nox = validate_amount(&amount)?;
    let amount_wei = (amount_nox * 1e18) as u128;

    let unstake_data = format!("0x34dfb268{:0>64}", format!("{:x}", amount_wei));
    let unstake_data_bytes = hex::decode(unstake_data.trim_start_matches("0x"))
        .map_err(|e| format!("Encode error: {}", e))?;
    let unstake_tx = send_sepolia(
        &private_key,
        NOX_STAKING_POOL_SEPOLIA,
        0,
        unstake_data_bytes,
        150000,
    )
    .await?;

    Ok(format!(
        "Unstake initiated for {} NOX (14-day unbonding period). Tx: {}",
        amount_nox, unstake_tx
    ))
}

#[tauri::command]
pub async fn staking_claim_rewards(_state: State<'_, AppState>) -> Result<String, String> {
    let manager = WALLET_MANAGER.read().await;
    let wallet = manager.active().ok_or("Wallet not unlocked")?;
    if !wallet.is_unlocked() {
        return Err("Wallet is locked".into());
    }

    let private_key = wallet
        .get_account_private_key(0)
        .map_err(|e| format!("Failed to get signing key: {}", e))?;
    drop(manager);

    let claim_data_bytes = hex::decode("4e71d92d").map_err(|e| format!("Encode error: {}", e))?;
    let claim_tx = send_sepolia(
        &private_key,
        NOX_STAKING_POOL_SEPOLIA,
        0,
        claim_data_bytes,
        150000,
    )
    .await?;

    Ok(format!("Rewards claimed on Sepolia! Tx: {}", claim_tx))
}

#[tauri::command]
pub async fn staking_withdraw(_state: State<'_, AppState>) -> Result<String, String> {
    let manager = WALLET_MANAGER.read().await;
    let wallet = manager.active().ok_or("Wallet not unlocked")?;
    if !wallet.is_unlocked() {
        return Err("Wallet is locked".into());
    }

    let private_key = wallet
        .get_account_private_key(0)
        .map_err(|e| format!("Failed to get signing key: {}", e))?;
    drop(manager);

    let withdraw_data_bytes =
        hex::decode("3ccfd60b").map_err(|e| format!("Encode error: {}", e))?;
    let withdraw_tx = send_sepolia(
        &private_key,
        NOX_STAKING_POOL_SEPOLIA,
        0,
        withdraw_data_bytes,
        150000,
    )
    .await?;

    Ok(format!(
        "Withdrawal complete on Sepolia! Tx: {}",
        withdraw_tx
    ))
}
