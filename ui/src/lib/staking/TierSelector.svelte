<script lang="ts">
	import type { LockTier } from './types';

	export let lockTiers: LockTier[];
	export let selectedTier: number;
</script>

<div class="tier-grid">
	{#each lockTiers as tier}
		<button class="tier-card" class:selected={selectedTier === tier.id} on:click={() => selectedTier = tier.id}>
			<div class="tier-duration">{tier.duration_days} Days</div>
			<div class="tier-multiplier">{tier.multiplier_display}</div>
			<div class="tier-label">Reward Multiplier</div>
		</button>
	{/each}
</div>

<style>
	.tier-grid {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: var(--nox-space-md);
		margin-bottom: var(--nox-space-xl);
	}

	.tier-card {
		background: var(--nox-bg-tertiary);
		border: 2px solid var(--nox-border);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-md);
		text-align: center;
		cursor: pointer;
		transition: all var(--nox-transition-fast);
	}

	.tier-card:hover {
		border-color: var(--nox-border-light);
	}

	.tier-card.selected {
		border-color: var(--nox-accent-primary);
		background: var(--nox-accent-glow);
	}

	.tier-duration {
		font-size: var(--nox-text-lg);
		font-weight: var(--nox-font-semibold);
		margin-bottom: var(--nox-space-xs);
	}

	.tier-multiplier {
		font-size: var(--nox-text-xl);
		font-weight: var(--nox-font-bold);
		color: var(--nox-accent-primary);
	}

	.tier-label {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
		margin-top: var(--nox-space-xs);
	}

	@media (max-width: 800px) {
		.tier-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 500px) {
		.tier-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
