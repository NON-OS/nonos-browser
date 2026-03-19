use super::query::query_contract;
use super::types::{get_lock_tiers, LPLockInfo, LockTier};
use crate::contracts::abi;

fn parse_u256(hex: &str) -> u128 {
    u128::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0)
}

fn format_nox(wei: u128) -> String {
    let nox = wei as f64 / 1e18;
    if nox == 0.0 {
        "0".into()
    } else {
        format!("{:.4}", nox)
    }
}

pub async fn fetch_user_locks(address: &str) -> Vec<LPLockInfo> {
    let locks_data = abi::encode_get_locks(address);
    let locks_result = query_contract(locks_data).await.unwrap_or_default();
    parse_locks_response(&locks_result).await
}

async fn parse_locks_response(result: &str) -> Vec<LPLockInfo> {
    let mut locks = Vec::new();
    if result.len() <= 130 {
        return locks;
    }

    let data = result.trim_start_matches("0x");
    let count = u64::from_str_radix(&data[64..128], 16).unwrap_or(0);
    let tiers = get_lock_tiers();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    for i in 0..count.min(20) {
        if let Some(lock) = parse_single_lock(data, i as usize, &tiers, now).await {
            locks.push(lock);
        }
    }
    locks
}

async fn parse_single_lock(
    data: &str,
    idx: usize,
    tiers: &[LockTier],
    now: u64,
) -> Option<LPLockInfo> {
    let offset = 128 + (idx * 192);
    if offset + 192 > data.len() {
        return None;
    }

    let lock_id = u64::from_str_radix(&data[offset..offset + 64], 16).ok()?;
    let amount = parse_u256(&data[offset + 64..offset + 128]);
    let tier = u8::from_str_radix(data[offset + 128..offset + 192].trim_start_matches('0'), 16)
        .unwrap_or(0);
    let lock_end =
        u64::from_str_radix(data.get(offset + 256..offset + 320)?.get(..64)?, 16).unwrap_or(0);

    let pending = query_contract(abi::encode_pending_rewards(lock_id))
        .await
        .map(|r| parse_u256(&r))
        .unwrap_or(0);
    let tier_info = tiers.get(tier as usize).unwrap_or(&tiers[0]);

    Some(LPLockInfo {
        lock_id,
        amount: format_nox(amount),
        tier,
        tier_name: format!("{} Days", tier_info.duration_days),
        multiplier: tier_info.multiplier_display.clone(),
        lock_start: 0,
        lock_end,
        is_locked: now < lock_end,
        pending_rewards: format_nox(pending),
    })
}
