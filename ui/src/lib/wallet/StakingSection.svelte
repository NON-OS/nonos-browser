<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import StakingStats from './StakingStats.svelte';
	import StakingActions from './StakingActions.svelte';
	import TierTable from './TierTable.svelte';

	const dispatch = createEventDispatcher();

	export let stakingStatus = { staked_amount: '0', tier: 'None', tier_multiplier: '0x', pending_rewards: '0', current_epoch: 0, next_tier_threshold: '1,000 NOX', estimated_apy: '0%' };
	export let stakingLoading = false;

	function handleStake(e: CustomEvent) { dispatch('stake', e.detail); }
	function handleUnstake(e: CustomEvent) { dispatch('unstake', e.detail); }
	function handleClaim() { dispatch('claim'); }
</script>

<div class="staking-section">
	<h2>NOX Staking</h2>
	<p class="staking-subtitle">Stake NOX to earn rewards and power the NONOS network</p>
	<StakingStats {stakingStatus} />
	<StakingActions {stakingLoading} pendingRewards={stakingStatus.pending_rewards} on:stake={handleStake} on:unstake={handleUnstake} on:claim={handleClaim} />
	<TierTable currentTier={stakingStatus.tier} nextTierThreshold={stakingStatus.next_tier_threshold} />
</div>

<style>
	.staking-section { background: var(--nox-bg-secondary); border: 1px solid var(--nox-border); border-radius: var(--nox-radius-xl); padding: var(--nox-space-xl); margin-top: var(--nox-space-lg); }
	.staking-section h2 { font-size: var(--nox-text-lg); font-weight: var(--nox-font-semibold); margin-bottom: var(--nox-space-xs); }
	.staking-subtitle { color: var(--nox-text-muted); font-size: var(--nox-text-sm); margin-bottom: var(--nox-space-lg); }
</style>
