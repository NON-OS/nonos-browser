use super::types::{get_lock_tiers, LPLockInfo, LPStakingStatus, LockTier};
use crate::contracts::{eth_call, Network, PRIVACY_LIQUIDITY_POOL_SEPOLIA};
use crate::state::AppState;
use crate::wallet::state::WALLET_MANAGER;
use tauri::State;

fn parse_u256(hex: &str) -> u128 {
    let clean = hex.trim_start_matches("0x");
    u128::from_str_radix(clean, 16).unwrap_or(0)
}

fn format_nox(wei: u128) -> String {
    let nox = wei as f64 / 1e18;
    if nox == 0.0 {
        "0".into()
    } else {
        format!("{:.4}", nox)
    }
}

async fn query_contract(data: &str) -> Result<String, String> {
    eth_call(Network::Sepolia, PRIVACY_LIQUIDITY_POOL_SEPOLIA, data).await
}

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

    let addr_padded = format!("{:0>64}", address.trim_start_matches("0x"));

    let total_locked = query_contract("0x56891412")
        .await
        .map(|r| parse_u256(&r))
        .unwrap_or(0);

    let weighted_total = query_contract("0x3a77236b")
        .await
        .map(|r| parse_u256(&r))
        .unwrap_or(0);

    let current_epoch = query_contract("0x76671808")
        .await
        .map(|r| parse_u256(&r) as u64)
        .unwrap_or(0);

    let epoch_pool = query_contract("0x7e1c0c09")
        .await
        .map(|r| parse_u256(&r))
        .unwrap_or(0);

    let locks_data = format!("0x9c94fcda{}", addr_padded);
    let locks_result = query_contract(&locks_data).await.unwrap_or_default();

    let mut locks = Vec::new();
    let mut total_pending: u128 = 0;
    let tiers = get_lock_tiers();

    if locks_result.len() > 130 {
        let data = locks_result.trim_start_matches("0x");
        if let Ok(count) = u64::from_str_radix(&data[64..128], 16) {
            for i in 0..count.min(20) {
                let offset = 128 + (i as usize * 192);
                if offset + 192 <= data.len() {
                    let lock_id = u64::from_str_radix(&data[offset..offset + 64], 16).unwrap_or(0);
                    let amount = parse_u256(&data[offset + 64..offset + 128]);
                    let tier = u8::from_str_radix(
                        data[offset + 128..offset + 192].trim_start_matches('0'),
                        16,
                    )
                    .unwrap_or(0);
                    let lock_start = u64::from_str_radix(
                        data[offset + 192..offset + 256].get(..64).unwrap_or("0"),
                        16,
                    )
                    .unwrap_or(0);
                    let lock_end = u64::from_str_radix(
                        data[offset + 256..offset + 320].get(..64).unwrap_or("0"),
                        16,
                    )
                    .unwrap_or(0);

                    let pending_data = format!("0x4665a42b{:0>64}", format!("{:x}", lock_id));
                    let pending = query_contract(&pending_data)
                        .await
                        .map(|r| parse_u256(&r))
                        .unwrap_or(0);
                    total_pending += pending;

                    let tier_info = tiers.get(tier as usize).unwrap_or(&tiers[0]);
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    locks.push(LPLockInfo {
                        lock_id,
                        amount: format_nox(amount),
                        tier,
                        tier_name: format!("{} Days", tier_info.duration_days),
                        multiplier: tier_info.multiplier_display.clone(),
                        lock_start,
                        lock_end,
                        is_locked: now < lock_end,
                        pending_rewards: format_nox(pending),
                    });
                }
            }
        }
    }

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
