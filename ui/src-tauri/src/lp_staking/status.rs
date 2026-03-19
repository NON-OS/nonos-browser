use super::parse::fetch_user_locks;
use super::query::{format_nox, parse_u256, query_contract};
use super::types::{get_lock_tiers, LPStakingStatus, LockTier};
use crate::contracts::abi;
use crate::state::AppState;
use crate::wallet::state::WALLET_MANAGER;
use tauri::State;

#[tauri::command]
pub async fn lp_get_status(_state: State<'_, AppState>) -> Result<LPStakingStatus, String> {
    let manager = WALLET_MANAGER.read().await;
    let address = manager
        .active()
        .map(|w| w.address().to_hex())
        .unwrap_or_default();
    drop(manager);

    if address.is_empty() {
        return Ok(LPStakingStatus {
            total_locked: "0".into(),
            weighted_total: "0".into(),
            locks: vec![],
            total_pending_rewards: "0".into(),
            available_tiers: get_lock_tiers(),
            current_epoch: 0,
            epoch_lp_pool: "0".into(),
        });
    }

    let total_locked = query_contract(abi::encode_total_locked())
        .await
        .map(|r| parse_u256(&r))
        .unwrap_or(0);
    let weighted_total = query_contract(abi::encode_weighted_total())
        .await
        .map(|r| parse_u256(&r))
        .unwrap_or(0);
    let current_epoch = query_contract(abi::encode_current_epoch())
        .await
        .map(|r| parse_u256(&r) as u64)
        .unwrap_or(0);
    let epoch_pool = query_contract(abi::encode_epoch_pool())
        .await
        .map(|r| parse_u256(&r))
        .unwrap_or(0);

    let locks = fetch_user_locks(&address).await;
    let total_pending: u128 = locks
        .iter()
        .filter_map(|l| l.pending_rewards.parse::<f64>().ok())
        .map(|n| (n * 1e18) as u128)
        .sum();

    Ok(LPStakingStatus {
        total_locked: format_nox(total_locked),
        weighted_total: format_nox(weighted_total),
        locks,
        total_pending_rewards: format_nox(total_pending),
        available_tiers: get_lock_tiers(),
        current_epoch,
        epoch_lp_pool: format_nox(epoch_pool),
    })
}

#[tauri::command]
pub fn lp_get_tiers() -> Vec<LockTier> {
    get_lock_tiers()
}
