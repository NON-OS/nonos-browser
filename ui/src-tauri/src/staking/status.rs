use super::types::{get_next_tier_threshold, get_staking_tier};
use crate::helpers::format_wei;
use crate::state::AppState;
use crate::types::StakingStatusResponse;
use tauri::State;

#[tauri::command]
pub async fn staking_get_status(
    state: State<'_, AppState>,
) -> Result<StakingStatusResponse, String> {
    let wallet = state.wallet.read().await;

    if !wallet.initialized {
        return Err("Wallet not initialized".into());
    }

    let staked_nox = wallet.staked_amount / 10u128.pow(18);
    let (tier_idx, tier_name, tier_mult) = get_staking_tier(staked_nox);
    let next_threshold = get_next_tier_threshold(tier_idx);

    let base_apy = 10.0;
    let multiplier: f64 = tier_mult.trim_end_matches('x').parse().unwrap_or(1.0);
    let estimated_apy = base_apy * multiplier;

    Ok(StakingStatusResponse {
        staked_amount: format_wei(wallet.staked_amount),
        tier: tier_name.to_string(),
        tier_multiplier: tier_mult.to_string(),
        pending_rewards: format_wei(wallet.pending_rewards),
        current_epoch: wallet.current_epoch,
        next_tier_threshold: if next_threshold > 0 {
            format!("{} NOX", next_threshold)
        } else {
            "Max tier".to_string()
        },
        estimated_apy: format!("{:.1}%", estimated_apy),
    })
}
