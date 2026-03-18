<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let nodeRunning = false;
	export let nodeStatus = { running: false, connected_nodes: 0, quality: 0, total_requests: 0 };
	export let stakingStatus = { staked_amount: '0', tier: 'None', tier_multiplier: '0x', pending_rewards: '0', estimated_apy: '0%' };
	export let isLoading = false;
	export let startTime = Date.now();

	function formatUptime(): string {
		if (!nodeRunning) return '0h 0m';
		const seconds = Math.floor((Date.now() - startTime) / 1000);
		const hours = Math.floor(seconds / 3600);
		const mins = Math.floor((seconds % 3600) / 60);
		return `${hours}h ${mins}m`;
	}

	function toggle() {
		dispatch('toggle');
	}
</script>

<div class="node-status">
	<div class="status-header">
		<h2>Node Status</h2>
		<div class="status-indicator" class:running={nodeRunning}>
			<span class="dot"></span>
			<span>{nodeRunning ? 'Running' : 'Stopped'}</span>
		</div>
	</div>

	<div class="metrics-grid">
		<div class="metric-card">
			<div class="metric-label">Uptime</div>
			<div class="metric-value">{formatUptime()}</div>
		</div>
		<div class="metric-card">
			<div class="metric-label">Requests Served</div>
			<div class="metric-value">{nodeStatus.total_requests.toLocaleString()}</div>
		</div>
		<div class="metric-card">
			<div class="metric-label">Staked NOX</div>
			<div class="metric-value">{stakingStatus.staked_amount} NOX</div>
		</div>
		<div class="metric-card">
			<div class="metric-label">Current Tier</div>
			<div class="metric-value tier">{stakingStatus.tier}</div>
		</div>
		<div class="metric-card">
			<div class="metric-label">Pending Rewards</div>
			<div class="metric-value reward">{stakingStatus.pending_rewards} NOX</div>
		</div>
		<div class="metric-card">
			<div class="metric-label">Est. APY</div>
			<div class="metric-value apy">{stakingStatus.estimated_apy}</div>
		</div>
	</div>

	<button class="btn toggle" class:stop={nodeRunning} on:click={toggle} disabled={isLoading}>
		{isLoading ? 'Processing...' : nodeRunning ? 'Stop Node' : 'Start Node'}
	</button>
</div>

<style>
	.node-status {
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-lg);
		margin-bottom: var(--nox-space-xl);
	}

	.status-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: var(--nox-space-lg);
	}

	.status-header h2 {
		font-size: 18px;
		font-weight: 600;
		margin-bottom: 0;
	}

	.status-indicator {
		display: flex;
		align-items: center;
		gap: var(--nox-space-sm);
		padding: var(--nox-space-xs) var(--nox-space-md);
		background: rgba(239, 68, 68, 0.1);
		border-radius: var(--nox-radius-full);
		font-size: 13px;
		color: var(--nox-error);
	}

	.status-indicator.running {
		background: rgba(34, 197, 94, 0.1);
		color: var(--nox-success);
	}

	.status-indicator .dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: currentColor;
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: var(--nox-space-md);
		margin-bottom: var(--nox-space-lg);
	}

	.metric-card {
		background: var(--nox-bg-tertiary);
		padding: var(--nox-space-md);
		border-radius: var(--nox-radius-md);
	}

	.metric-label {
		font-size: 12px;
		color: var(--nox-text-muted);
		margin-bottom: var(--nox-space-xs);
	}

	.metric-value {
		font-size: 20px;
		font-weight: 600;
		font-family: var(--nox-font-mono);
	}

	.metric-value.tier { color: var(--nox-accent-primary); }
	.metric-value.reward { color: var(--nox-success); }
	.metric-value.apy { color: var(--nox-warning); }

	.btn.toggle {
		width: 100%;
		padding: var(--nox-space-md);
		background: var(--nox-success);
		color: white;
		border-radius: var(--nox-radius-md);
		font-weight: 600;
		transition: all var(--nox-transition-fast);
	}

	.btn.toggle:hover:not(:disabled) { filter: brightness(1.1); }
	.btn.toggle.stop { background: var(--nox-error); }
	.btn.toggle:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
