<script lang="ts">
	import type { LockTier, LPLockInfo, LPStatus } from './types';
	import { createEventDispatcher } from 'svelte';
	import LockForm from './LockForm.svelte';
	import TierSelector from './TierSelector.svelte';
	import LocksList from './LocksList.svelte';
	import LPSummary from './LPSummary.svelte';

	export let lockTiers: LockTier[] = [];
	export let myLocks: LPLockInfo[] = [];
	export let lpStatus: LPStatus;
	export let lockLoading = false;

	const dispatch = createEventDispatcher<{
		lock: { amount: string; tier: number };
		unlock: { lockId: number };
		claim: { lockId: number };
	}>();

	let selectedTier = 0;

	function handleLock(e: CustomEvent<{ amount: string; tier: number }>) {
		dispatch('lock', e.detail);
	}

	function handleUnlock(e: CustomEvent<{ lockId: number }>) {
		dispatch('unlock', e.detail);
	}

	function handleClaim(e: CustomEvent<{ lockId: number }>) {
		dispatch('claim', e.detail);
	}
</script>

<div class="section">
	<div class="section-header">
		<h2>Lock NOX Tokens</h2>
		<p>Lock your NOX tokens to earn a share of the LP pool rewards (30% of emissions)</p>
	</div>

	<LockForm {lockTiers} {lockLoading} {selectedTier} on:lock={handleLock} />

	<TierSelector {lockTiers} bind:selectedTier />

	<LocksList {myLocks} on:unlock={handleUnlock} on:claim={handleClaim} />

	<LPSummary {lpStatus} />
</div>

<style>
	.section {
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-xl);
		padding: var(--nox-space-xl);
	}

	.section-header {
		margin-bottom: var(--nox-space-lg);
	}

	.section-header h2 {
		font-size: var(--nox-text-lg);
		font-weight: var(--nox-font-semibold);
		margin-bottom: var(--nox-space-xs);
	}

	.section-header p {
		color: var(--nox-text-muted);
		font-size: var(--nox-text-sm);
	}
</style>
