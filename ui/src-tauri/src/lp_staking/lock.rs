use super::types::{get_lock_tiers, LP_STAKING_ADDRESS};
use crate::contracts::{fetch_sepolia_balances, send_sepolia, NOX_TOKEN_ADDRESS_SEPOLIA};
use crate::helpers::{validate_amount, validate_tier};
use crate::state::AppState;
use crate::wallet::state::WALLET_MANAGER;
use tauri::State;

#[tauri::command]
pub async fn lp_lock(
    _state: State<'_, AppState>,
    amount: String,
    tier: u8,
) -> Result<String, String> {
    validate_tier(tier).map_err(|e| e.to_string())?;

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

    let amount_nox = validate_amount(&amount).map_err(|e| e.to_string())?;
    let amount_wei = (amount_nox * 1e18) as u128;

    let (_, sepolia_nox) = fetch_sepolia_balances(&address).await;
    if amount_wei > sepolia_nox {
        return Err(format!(
            "Insufficient Sepolia NOX balance. Have {} NOX, locking {} NOX",
            sepolia_nox as f64 / 1e18,
            amount_nox
        ));
    }

    let approve_data = format!(
        "0x095ea7b3{:0>64}{:0>64}",
        LP_STAKING_ADDRESS.trim_start_matches("0x"),
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

    let lock_data = format!(
        "0x4a4de4a8{:0>64}{:0>64}",
        format!("{:x}", amount_wei),
        format!("{:x}", tier)
    );
    let lock_data_bytes = hex::decode(lock_data.trim_start_matches("0x"))
        .map_err(|e| format!("Encode error: {}", e))?;
    let lock_tx =
        send_sepolia(&private_key, LP_STAKING_ADDRESS, 0, lock_data_bytes, 200000).await?;

    let tier_info = get_lock_tiers()
        .get(tier as usize)
        .map(|t| t.duration_days)
        .unwrap_or(0);
    Ok(format!(
        "Locked {} NOX for {} days on Sepolia! Tx: {}",
        amount_nox, tier_info, lock_tx
    ))
}

#[tauri::command]
pub async fn lp_unlock(_state: State<'_, AppState>, lock_id: u64) -> Result<String, String> {
    let manager = WALLET_MANAGER.read().await;
    let wallet = manager.active().ok_or("Wallet not unlocked")?;
    if !wallet.is_unlocked() {
        return Err("Wallet is locked".into());
    }

    let private_key = wallet
        .get_account_private_key(0)
        .map_err(|e| format!("Failed to get signing key: {}", e))?;
    drop(manager);

    let unlock_data = format!("0xa69df4b5{:0>64}", format!("{:x}", lock_id));
    let unlock_data_bytes = hex::decode(unlock_data.trim_start_matches("0x"))
        .map_err(|e| format!("Encode error: {}", e))?;
    let unlock_tx = send_sepolia(
        &private_key,
        LP_STAKING_ADDRESS,
        0,
        unlock_data_bytes,
        150000,
    )
    .await?;

    Ok(format!(
        "Unlocked position #{} on Sepolia! Tx: {}",
        lock_id, unlock_tx
    ))
}

#[tauri::command]
pub async fn lp_early_unlock(_state: State<'_, AppState>, lock_id: u64) -> Result<String, String> {
    let manager = WALLET_MANAGER.read().await;
    let wallet = manager.active().ok_or("Wallet not unlocked")?;
    if !wallet.is_unlocked() {
        return Err("Wallet is locked".into());
    }

    let private_key = wallet
        .get_account_private_key(0)
        .map_err(|e| format!("Failed to get signing key: {}", e))?;
    drop(manager);

    let unlock_data = format!("0x7b4d0e4c{:0>64}", format!("{:x}", lock_id));
    let unlock_data_bytes = hex::decode(unlock_data.trim_start_matches("0x"))
        .map_err(|e| format!("Encode error: {}", e))?;
    let unlock_tx = send_sepolia(
        &private_key,
        LP_STAKING_ADDRESS,
        0,
        unlock_data_bytes,
        150000,
    )
    .await?;

    Ok(format!(
        "Early unlock for position #{} on Sepolia! (Penalty applied) Tx: {}",
        lock_id, unlock_tx
    ))
}

#[tauri::command]
pub async fn lp_extend_lock(
    _state: State<'_, AppState>,
    lock_id: u64,
    new_tier: u8,
) -> Result<String, String> {
    validate_tier(new_tier).map_err(|e| e.to_string())?;

    let manager = WALLET_MANAGER.read().await;
    let wallet = manager.active().ok_or("Wallet not unlocked")?;
    if !wallet.is_unlocked() {
        return Err("Wallet is locked".into());
    }

    let private_key = wallet
        .get_account_private_key(0)
        .map_err(|e| format!("Failed to get signing key: {}", e))?;
    drop(manager);

    let extend_data = format!(
        "0x9e8c708e{:0>64}{:0>64}",
        format!("{:x}", lock_id),
        format!("{:x}", new_tier)
    );
    let extend_data_bytes = hex::decode(extend_data.trim_start_matches("0x"))
        .map_err(|e| format!("Encode error: {}", e))?;
    let extend_tx = send_sepolia(
        &private_key,
        LP_STAKING_ADDRESS,
        0,
        extend_data_bytes,
        150000,
    )
    .await?;

    let tier_info = get_lock_tiers()
        .get(new_tier as usize)
        .map(|t| t.duration_days)
        .unwrap_or(0);
    Ok(format!(
        "Extended lock #{} to {} day tier on Sepolia! Tx: {}",
        lock_id, tier_info, extend_tx
    ))
}
