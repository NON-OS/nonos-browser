<script lang="ts">
	import type { LPStatus } from './types';
	import { createEventDispatcher } from 'svelte';
	import RewardCard from './RewardCard.svelte';
	import RewardsBreakdown from './RewardsBreakdown.svelte';
	import EmissionInfo from './EmissionInfo.svelte';

	export let lpStatus: LPStatus;
	export let estimatedReward = '0';
	export let lockLoading = false;

	const dispatch = createEventDispatcher<{ claimAll: void }>();
</script>

<div class="section">
	<div class="section-header">
		<h2>Rewards</h2>
		<p>Claim your earned NOX rewards from node operations and LP staking</p>
	</div>
	<RewardCard totalPendingRewards={lpStatus.total_pending_rewards} {lockLoading} on:claimAll={() => dispatch('claimAll')} />
	<RewardsBreakdown totalPendingRewards={lpStatus.total_pending_rewards} {estimatedReward} />
	<EmissionInfo />
</div>

<style>
	.section { background: var(--nox-bg-secondary); border: 1px solid var(--nox-border); border-radius: var(--nox-radius-xl); padding: var(--nox-space-xl); }
	.section-header { margin-bottom: var(--nox-space-lg); }
	.section-header h2 { font-size: var(--nox-text-lg); font-weight: var(--nox-font-semibold); margin-bottom: var(--nox-space-xs); }
	.section-header p { color: var(--nox-text-muted); font-size: var(--nox-text-sm); }
</style>
