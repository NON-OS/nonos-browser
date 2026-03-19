use super::types::LP_STAKING_ADDRESS;
use crate::contracts::abi;
use crate::contracts::send_sepolia;
use crate::state::AppState;
use crate::wallet::state::WALLET_MANAGER;
use tauri::State;

async fn get_wallet_key() -> Result<String, String> {
    let manager = WALLET_MANAGER.read().await;
    let wallet = manager.active().ok_or("Wallet not unlocked")?;
    if !wallet.is_unlocked() {
        return Err("Wallet is locked".into());
    }
    wallet
        .get_account_private_key(0)
        .map_err(|e| format!("Key error: {}", e))
}

#[tauri::command]
pub async fn lp_claim_rewards(_state: State<'_, AppState>, lock_id: u64) -> Result<String, String> {
    let private_key = get_wallet_key().await?;
    let claim = abi::encode_claim_lock_rewards(lock_id);
    let tx = send_sepolia(&private_key, LP_STAKING_ADDRESS, 0, claim, 150000).await?;
    Ok(format!("Claimed rewards for lock #{}. Tx: {}", lock_id, tx))
}

#[tauri::command]
pub async fn lp_claim_all_rewards(_state: State<'_, AppState>) -> Result<String, String> {
    let private_key = get_wallet_key().await?;
    let claim = abi::encode_claim_all();
    let tx = send_sepolia(&private_key, LP_STAKING_ADDRESS, 0, claim, 300000).await?;
    Ok(format!("Claimed all rewards. Tx: {}", tx))
}

#[tauri::command]
pub async fn lp_compound_rewards(
    _state: State<'_, AppState>,
    lock_id: u64,
) -> Result<String, String> {
    let private_key = get_wallet_key().await?;
    let compound = abi::encode_compound(lock_id);
    let tx = send_sepolia(&private_key, LP_STAKING_ADDRESS, 0, compound, 200000).await?;
    Ok(format!(
        "Compounded rewards for lock #{}. Tx: {}",
        lock_id, tx
    ))
}
