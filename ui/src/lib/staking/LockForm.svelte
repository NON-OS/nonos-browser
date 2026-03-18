<script lang="ts">
	import type { LockTier } from './types';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let lockTiers: LockTier[];
	export let lockLoading: boolean;
	export let selectedTier: number;

	let lockAmount = '';

	function handleLock() {
		if (!lockAmount) return;
		dispatch('lock', { amount: String(lockAmount), tier: selectedTier });
		lockAmount = '';
	}
</script>

<div class="lock-form">
	<div class="form-row">
		<div class="form-group">
			<label for="lock-amount">Amount to Lock</label>
			<input id="lock-amount" type="number" bind:value={lockAmount} placeholder="0.00" class="input" />
		</div>
		<div class="form-group">
			<label for="lock-duration">Lock Duration</label>
			<select id="lock-duration" bind:value={selectedTier} class="input">
				{#each lockTiers as tier}
					<option value={tier.id}>{tier.duration_days} days ({tier.multiplier_display})</option>
				{/each}
			</select>
		</div>
	</div>
	<button class="btn primary full-width" on:click={handleLock} disabled={lockLoading || !lockAmount}>
		{lockLoading ? 'Processing...' : 'Lock NOX'}
	</button>
</div>

<style>
	.lock-form {
		background: var(--nox-bg-tertiary);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-lg);
		margin-bottom: var(--nox-space-lg);
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: var(--nox-space-md);
		margin-bottom: var(--nox-space-md);
	}

	.form-group label {
		display: block;
		font-size: var(--nox-text-sm);
		color: var(--nox-text-secondary);
		margin-bottom: var(--nox-space-xs);
	}

	.input {
		width: 100%;
		padding: var(--nox-space-sm) var(--nox-space-md);
		background: var(--nox-bg-primary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-md);
		color: var(--nox-text-primary);
		font-size: var(--nox-text-sm);
	}

	.input:focus {
		outline: none;
		border-color: var(--nox-accent-primary);
	}

	select.input {
		cursor: pointer;
	}

	.btn {
		padding: var(--nox-space-sm) var(--nox-space-lg);
		border-radius: var(--nox-radius-md);
		font-weight: var(--nox-font-medium);
		font-size: var(--nox-text-sm);
		cursor: pointer;
		transition: all var(--nox-transition-fast);
		border: none;
	}

	.btn.primary {
		background: var(--nox-accent-gradient);
		color: var(--nox-bg-primary);
	}

	.btn.primary:hover:not(:disabled) {
		box-shadow: var(--nox-shadow-glow);
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn.full-width {
		width: 100%;
	}

	@media (max-width: 800px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>
