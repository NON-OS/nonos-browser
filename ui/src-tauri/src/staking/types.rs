use crate::types::STAKING_TIERS;

pub fn get_staking_tier(staked_nox: u128) -> (usize, &'static str, &'static str) {
    for (i, (name, threshold, mult)) in STAKING_TIERS.iter().enumerate().rev() {
        if staked_nox >= *threshold {
            return (i, name, mult);
        }
    }
    (0, "None", "0x")
}

pub fn get_next_tier_threshold(current_tier: usize) -> u128 {
    if current_tier + 1 < STAKING_TIERS.len() {
        STAKING_TIERS[current_tier + 1].1
    } else {
        0
    }
}
