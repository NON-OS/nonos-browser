<script lang="ts">
	import type { BootstrapNode } from './types';

	export let nodes: BootstrapNode[] = [];
</script>

<div class="nodes-panel">
	<div class="nodes-header">
		<h3>Infrastructure Nodes</h3>
		<span class="node-count">{nodes.filter(n => n.status === 'online').length} online</span>
	</div>
	<div class="nodes-grid">
		{#each nodes as node}
			<div class="node-card" class:online={node.status === 'online'}>
				<span class="node-flag">{node.flag}</span>
				<div class="node-info">
					<span class="node-name">{node.name}</span>
					<span class="node-ip">{node.ip}</span>
				</div>
				<span class="node-status"></span>
			</div>
		{/each}
	</div>
</div>

<style>
	.nodes-panel {
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-xl);
		padding: var(--nox-space-lg);
		margin-bottom: var(--nox-space-lg);
	}

	.nodes-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: var(--nox-space-md);
	}

	.nodes-header h3 {
		font-size: var(--nox-text-sm);
		font-weight: var(--nox-font-medium);
		color: var(--nox-text-secondary);
	}

	.node-count {
		font-size: var(--nox-text-xs);
		color: var(--nox-success);
		font-weight: var(--nox-font-medium);
	}

	.nodes-grid {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: var(--nox-space-sm);
	}

	@media (max-width: 900px) {
		.nodes-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 500px) {
		.nodes-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.node-card {
		display: flex;
		align-items: center;
		gap: var(--nox-space-sm);
		padding: var(--nox-space-sm);
		background: var(--nox-bg-tertiary);
		border-radius: var(--nox-radius-md);
		position: relative;
	}

	.node-flag {
		font-size: 16px;
	}

	.node-info {
		flex: 1;
		min-width: 0;
	}

	.node-name {
		font-size: var(--nox-text-xs);
		font-weight: var(--nox-font-medium);
		display: block;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.node-ip {
		font-size: 9px;
		color: var(--nox-text-muted);
		font-family: var(--nox-font-mono);
	}

	.node-status {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: var(--nox-text-muted);
	}

	.node-card.online .node-status {
		background: var(--nox-success);
		box-shadow: 0 0 6px var(--nox-success);
	}
</style>
