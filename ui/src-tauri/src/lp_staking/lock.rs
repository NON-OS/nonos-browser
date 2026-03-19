use super::types::{get_lock_tiers, LP_STAKING_ADDRESS};
use crate::contracts::abi;
use crate::contracts::{fetch_sepolia_balances, send_sepolia, NOX_TOKEN_ADDRESS_SEPOLIA};
use crate::helpers::{validate_amount, validate_tier};
use crate::state::AppState;
use crate::wallet::state::WALLET_MANAGER;
use tauri::State;

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

#[tauri::command]
pub async fn lp_lock(
    _state: State<'_, AppState>,
    amount: String,
    tier: u8,
) -> Result<String, String> {
    validate_tier(tier).map_err(|e| e.to_string())?;
    let (private_key, address) = get_wallet_key().await?;
    let amount_nox = validate_amount(&amount).map_err(|e| e.to_string())?;
    let amount_wei = (amount_nox * 1e18) as u128;

    let (_, sepolia_nox) = fetch_sepolia_balances(&address).await;
    if amount_wei > sepolia_nox {
        return Err(format!(
            "Insufficient NOX balance. Have {} NOX, locking {} NOX",
            sepolia_nox as f64 / 1e18,
            amount_nox
        ));
    }

    let approve = abi::encode_approve(LP_STAKING_ADDRESS, amount_wei);
    send_sepolia(&private_key, NOX_TOKEN_ADDRESS_SEPOLIA, 0, approve, 100000).await?;

    let lock = abi::encode_lock(amount_wei, tier);
    let tx = send_sepolia(&private_key, LP_STAKING_ADDRESS, 0, lock, 200000).await?;

    let tier_info = get_lock_tiers()
        .get(tier as usize)
        .map(|t| t.duration_days)
        .unwrap_or(0);
    Ok(format!(
        "Locked {} NOX for {} days. Tx: {}",
        amount_nox, tier_info, tx
    ))
}

#[tauri::command]
pub async fn lp_unlock(_state: State<'_, AppState>, lock_id: u64) -> Result<String, String> {
    let (private_key, _) = get_wallet_key().await?;
    let unlock = abi::encode_unlock(lock_id);
    let tx = send_sepolia(&private_key, LP_STAKING_ADDRESS, 0, unlock, 150000).await?;
    Ok(format!("Unlocked position #{}. Tx: {}", lock_id, tx))
}

#[tauri::command]
pub async fn lp_early_unlock(_state: State<'_, AppState>, lock_id: u64) -> Result<String, String> {
    let (private_key, _) = get_wallet_key().await?;
    let unlock = abi::encode_early_unlock(lock_id);
    let tx = send_sepolia(&private_key, LP_STAKING_ADDRESS, 0, unlock, 150000).await?;
    Ok(format!(
        "Early unlock for position #{} (penalty applied). Tx: {}",
        lock_id, tx
    ))
}

#[tauri::command]
pub async fn lp_extend_lock(
    _state: State<'_, AppState>,
    lock_id: u64,
    new_tier: u8,
) -> Result<String, String> {
    validate_tier(new_tier).map_err(|e| e.to_string())?;
    let (private_key, _) = get_wallet_key().await?;
    let extend = abi::encode_extend_lock(lock_id, new_tier);
    let tx = send_sepolia(&private_key, LP_STAKING_ADDRESS, 0, extend, 150000).await?;

    let tier_info = get_lock_tiers()
        .get(new_tier as usize)
        .map(|t| t.duration_days)
        .unwrap_or(0);
    Ok(format!(
        "Extended lock #{} to {} day tier. Tx: {}",
        lock_id, tier_info, tx
    ))
}
