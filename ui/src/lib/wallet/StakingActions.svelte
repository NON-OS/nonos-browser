<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	export let stakingLoading = false;
	export let pendingRewards = '0';

	let stakeAmount = '';
	let unstakeAmount = '';

	function handleStake() { if (!stakeAmount) return; dispatch('stake', { amount: stakeAmount }); stakeAmount = ''; }
	function handleUnstake() { if (!unstakeAmount) return; dispatch('unstake', { amount: unstakeAmount }); unstakeAmount = ''; }
	function handleClaim() { dispatch('claim'); }
</script>

<div class="staking-actions">
	<div class="staking-input-group">
		<input type="number" bind:value={stakeAmount} placeholder="Amount to stake" class="input staking-input" disabled={stakingLoading} />
		<button class="btn primary" on:click={handleStake} disabled={stakingLoading || !stakeAmount}>
			{stakingLoading ? 'Processing...' : 'Stake NOX'}
		</button>
	</div>
	<div class="staking-input-group">
		<input type="number" bind:value={unstakeAmount} placeholder="Amount to unstake" class="input staking-input" disabled={stakingLoading} />
		<button class="btn secondary" on:click={handleUnstake} disabled={stakingLoading || !unstakeAmount}>
			{stakingLoading ? 'Processing...' : 'Unstake'}
		</button>
	</div>
	{#if parseFloat(pendingRewards) > 0}
		<button class="btn claim-btn" on:click={handleClaim} disabled={stakingLoading}>
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2v20M2 12h20"/></svg>
			{stakingLoading ? 'Claiming...' : `Claim ${pendingRewards} NOX`}
		</button>
	{/if}
</div>

<style>
	.staking-actions { display: flex; flex-direction: column; gap: var(--nox-space-md); margin-bottom: var(--nox-space-lg); }
	.staking-input-group { display: flex; gap: var(--nox-space-md); }
	.staking-input { flex: 1; }
	.btn { display: inline-flex; align-items: center; justify-content: center; gap: var(--nox-space-sm); padding: var(--nox-space-sm) var(--nox-space-lg); border-radius: var(--nox-radius-md); font-weight: var(--nox-font-medium); transition: all var(--nox-transition-fast); font-size: var(--nox-text-sm); }
	.btn.primary { background: var(--nox-accent-gradient); color: var(--nox-bg-primary); }
	.btn.primary:hover:not(:disabled) { box-shadow: var(--nox-shadow-glow); }
	.btn.secondary { background: var(--nox-bg-tertiary); border: 1px solid var(--nox-border); color: var(--nox-text-primary); }
	.btn.secondary:hover:not(:disabled) { border-color: var(--nox-accent-primary); }
	.btn:disabled { opacity: 0.5; cursor: not-allowed; }
	.claim-btn { width: 100%; padding: var(--nox-space-md); background: var(--nox-success-bg); border: 1px solid var(--nox-success); color: var(--nox-success); border-radius: var(--nox-radius-md); font-weight: var(--nox-font-medium); }
	.claim-btn:hover:not(:disabled) { background: var(--nox-success); color: var(--nox-bg-primary); }
	.claim-btn svg { width: 18px; height: 18px; }
</style>
