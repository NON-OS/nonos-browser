<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();
	export let totalPendingRewards = '0';
	export let lockLoading = false;
</script>

<div class="rewards-overview">
	<div class="reward-card total">
		<div class="reward-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2L22 12L12 22L2 12L12 2Z"/></svg></div>
		<div class="reward-info">
			<div class="reward-title">Total Pending Rewards</div>
			<div class="reward-amount">{totalPendingRewards} NOX</div>
		</div>
		<button class="btn primary" on:click={() => dispatch('claimAll')} disabled={lockLoading}>{lockLoading ? 'Claiming...' : 'Claim All'}</button>
	</div>
</div>

<style>
	.rewards-overview { margin-bottom: var(--nox-space-xl); }
	.reward-card { display: flex; align-items: center; gap: var(--nox-space-lg); background: var(--nox-bg-tertiary); border-radius: var(--nox-radius-xl); padding: var(--nox-space-xl); }
	.reward-card.total { background: linear-gradient(135deg, var(--nox-accent-glow) 0%, var(--nox-bg-tertiary) 100%); border: 1px solid var(--nox-accent-primary); }
	.reward-icon { width: 56px; height: 56px; display: flex; align-items: center; justify-content: center; background: var(--nox-accent-gradient); border-radius: var(--nox-radius-full); color: var(--nox-bg-primary); }
	.reward-icon svg { width: 28px; height: 28px; }
	.reward-info { flex: 1; }
	.reward-title { font-size: var(--nox-text-sm); color: var(--nox-text-muted); margin-bottom: var(--nox-space-xs); }
	.reward-amount { font-size: var(--nox-text-2xl); font-weight: var(--nox-font-bold); }
	.btn { padding: var(--nox-space-sm) var(--nox-space-lg); border-radius: var(--nox-radius-md); font-weight: var(--nox-font-medium); font-size: var(--nox-text-sm); cursor: pointer; transition: all var(--nox-transition-fast); border: none; }
	.btn.primary { background: var(--nox-accent-gradient); color: var(--nox-bg-primary); }
	.btn.primary:hover:not(:disabled) { box-shadow: var(--nox-shadow-glow); }
	.btn:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
