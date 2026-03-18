use super::types::LP_STAKING_ADDRESS;
use crate::contracts::send_sepolia;
use crate::state::AppState;
use crate::wallet::state::WALLET_MANAGER;
use tauri::State;

#[tauri::command]
pub async fn lp_claim_rewards(_state: State<'_, AppState>, lock_id: u64) -> Result<String, String> {
    let manager = WALLET_MANAGER.read().await;
    let wallet = manager.active().ok_or("Wallet not unlocked")?;
    if !wallet.is_unlocked() {
        return Err("Wallet is locked".into());
    }

    let private_key = wallet
        .get_account_private_key(0)
        .map_err(|e| format!("Failed to get signing key: {}", e))?;
    drop(manager);

    let claim_data = format!("0x0fae75d9{:0>64}", format!("{:x}", lock_id));
    let claim_data_bytes = hex::decode(claim_data.trim_start_matches("0x"))
        .map_err(|e| format!("Encode error: {}", e))?;
    let claim_tx = send_sepolia(
        &private_key,
        LP_STAKING_ADDRESS,
        0,
        claim_data_bytes,
        150000,
    )
    .await?;

    Ok(format!(
        "Claimed rewards for lock #{} on Sepolia! Tx: {}",
        lock_id, claim_tx
    ))
}

#[tauri::command]
pub async fn lp_claim_all_rewards(_state: State<'_, AppState>) -> Result<String, String> {
    let manager = WALLET_MANAGER.read().await;
    let wallet = manager.active().ok_or("Wallet not unlocked")?;
    if !wallet.is_unlocked() {
        return Err("Wallet is locked".into());
    }

    let private_key = wallet
        .get_account_private_key(0)
        .map_err(|e| format!("Failed to get signing key: {}", e))?;
    drop(manager);

    let claim_data = "0x4e71d92d";
    let claim_data_bytes = hex::decode(claim_data.trim_start_matches("0x"))
        .map_err(|e| format!("Encode error: {}", e))?;
    let claim_tx = send_sepolia(
        &private_key,
        LP_STAKING_ADDRESS,
        0,
        claim_data_bytes,
        300000,
    )
    .await?;

    Ok(format!("Claimed all rewards on Sepolia! Tx: {}", claim_tx))
}

#[tauri::command]
pub async fn lp_compound_rewards(
    _state: State<'_, AppState>,
    lock_id: u64,
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

    let compound_data = format!("0xf69e2046{:0>64}", format!("{:x}", lock_id));
    let compound_data_bytes = hex::decode(compound_data.trim_start_matches("0x"))
        .map_err(|e| format!("Encode error: {}", e))?;
    let compound_tx = send_sepolia(
        &private_key,
        LP_STAKING_ADDRESS,
        0,
        compound_data_bytes,
        200000,
    )
    .await?;

    Ok(format!(
        "Compounded rewards for lock #{} on Sepolia! Tx: {}",
        lock_id, compound_tx
    ))
}
