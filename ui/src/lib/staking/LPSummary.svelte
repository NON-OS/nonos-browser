<script lang="ts">
	import type { LPStatus } from './types';

	export let lpStatus: LPStatus;
</script>

<div class="lp-summary">
	<div class="summary-card">
		<div class="summary-label">Total Locked</div>
		<div class="summary-value">{lpStatus.total_locked} NOX</div>
	</div>
	<div class="summary-card">
		<div class="summary-label">Weighted Total</div>
		<div class="summary-value">{lpStatus.weighted_total}</div>
	</div>
	<div class="summary-card">
		<div class="summary-label">Pending Rewards</div>
		<div class="summary-value accent">{lpStatus.total_pending_rewards} NOX</div>
	</div>
	<div class="summary-card">
		<div class="summary-label">Current Epoch</div>
		<div class="summary-value">{lpStatus.current_epoch}</div>
	</div>
</div>

<style>
	.lp-summary {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: var(--nox-space-md);
	}

	.summary-card {
		background: var(--nox-bg-tertiary);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-md);
		text-align: center;
	}

	.summary-label {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
		margin-bottom: var(--nox-space-xs);
	}

	.summary-value {
		font-size: var(--nox-text-lg);
		font-weight: var(--nox-font-semibold);
		font-family: var(--nox-font-mono);
	}

	.summary-value.accent {
		color: var(--nox-accent-primary);
	}

	@media (max-width: 800px) {
		.lp-summary {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
