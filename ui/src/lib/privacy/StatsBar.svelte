<script lang="ts">
	import type { NetworkStats } from './types';

	export let stats: NetworkStats;
</script>

<div class="stats-bar">
	<div class="stat">
		<span class="stat-value">{stats.proofs_generated}</span>
		<span class="stat-label">ZK Proofs</span>
	</div>
	<div class="stat">
		<span class="stat-value">{stats.mix_operations}</span>
		<span class="stat-label">Mix Ops</span>
	</div>
	<div class="stat">
		<span class="stat-value">{stats.trackers_blocked}</span>
		<span class="stat-label">Trackers Blocked</span>
	</div>
	<div class="stat">
		<span class="stat-value">{stats.active_users}</span>
		<span class="stat-label">Active Identities</span>
	</div>
</div>

<style>
	.stats-bar {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: var(--nox-space-md);
		margin-bottom: var(--nox-space-xl);
	}

	@media (max-width: 600px) {
		.stats-bar {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.stat {
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-md);
		text-align: center;
	}

	.stat-value {
		display: block;
		font-size: var(--nox-text-xl);
		font-weight: var(--nox-font-semibold);
		font-family: var(--nox-font-mono);
		color: var(--nox-accent-primary);
	}

	.stat-label {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
	}
</style>
