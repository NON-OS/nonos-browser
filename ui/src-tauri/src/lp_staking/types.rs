use crate::contracts::PRIVACY_LIQUIDITY_POOL_SEPOLIA;
use serde::{Deserialize, Serialize};

pub const LP_STAKING_ADDRESS: &str = PRIVACY_LIQUIDITY_POOL_SEPOLIA;

#[derive(Clone, Serialize, Deserialize)]
pub struct LockTier {
    pub id: u8,
    pub duration_days: u16,
    pub multiplier: u16,
    pub multiplier_display: String,
}

pub fn get_lock_tiers() -> Vec<LockTier> {
    vec![
        LockTier {
            id: 0,
            duration_days: 14,
            multiplier: 10000,
            multiplier_display: "1.00x".into(),
        },
        LockTier {
            id: 1,
            duration_days: 30,
            multiplier: 12500,
            multiplier_display: "1.25x".into(),
        },
        LockTier {
            id: 2,
            duration_days: 90,
            multiplier: 16000,
            multiplier_display: "1.60x".into(),
        },
        LockTier {
            id: 3,
            duration_days: 180,
            multiplier: 20000,
            multiplier_display: "2.00x".into(),
        },
        LockTier {
            id: 4,
            duration_days: 365,
            multiplier: 25000,
            multiplier_display: "2.50x".into(),
        },
    ]
}

#[derive(Serialize)]
pub struct LPLockInfo {
    pub lock_id: u64,
    pub amount: String,
    pub tier: u8,
    pub tier_name: String,
    pub multiplier: String,
    pub lock_start: u64,
    pub lock_end: u64,
    pub is_locked: bool,
    pub pending_rewards: String,
}

#[derive(Serialize)]
pub struct LPStakingStatus {
    pub total_locked: String,
    pub weighted_total: String,
    pub locks: Vec<LPLockInfo>,
    pub total_pending_rewards: String,
    pub available_tiers: Vec<LockTier>,
    pub current_epoch: u64,
    pub epoch_lp_pool: String,
}
