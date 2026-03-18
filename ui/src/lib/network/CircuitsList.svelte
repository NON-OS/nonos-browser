<script lang="ts">
	import type { Circuit } from './types';

	export let circuits: Circuit[] = [];
</script>

<div class="circuits-section">
	<h2>Active Circuits</h2>
	<p class="section-desc">
		Circuits route your traffic through multiple relays for privacy. Each circuit provides a unique identity.
	</p>

	{#if circuits.length === 0}
		<div class="empty-state">
			<p>No active circuits. Connect to the network to start browsing privately.</p>
		</div>
	{:else}
		<div class="circuits-list">
			{#each circuits as circuit}
				<div class="circuit-card">
					<div class="circuit-id">{circuit.id}</div>
					<div class="circuit-path">
						{#each circuit.path as relay, i}
							<span class="relay">{relay}</span>
							{#if i < circuit.path.length - 1}
								<span class="arrow">-></span>
							{/if}
						{/each}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	h2 {
		font-size: 18px;
		font-weight: 600;
		margin-bottom: var(--nox-space-sm);
	}

	.section-desc {
		color: var(--nox-text-secondary);
		font-size: 14px;
		margin-bottom: var(--nox-space-lg);
	}

	.circuits-section {
		margin-bottom: var(--nox-space-xl);
	}

	.empty-state {
		background: var(--nox-bg-secondary);
		border: 1px dashed var(--nox-border);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-xl);
		text-align: center;
		color: var(--nox-text-muted);
	}

	.circuits-list {
		display: flex;
		flex-direction: column;
		gap: var(--nox-space-sm);
	}

	.circuit-card {
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-md);
		padding: var(--nox-space-md);
	}

	.circuit-id {
		font-size: 12px;
		font-family: var(--nox-font-mono);
		color: var(--nox-text-muted);
		margin-bottom: var(--nox-space-xs);
	}

	.circuit-path {
		display: flex;
		align-items: center;
		gap: var(--nox-space-sm);
		flex-wrap: wrap;
	}

	.relay {
		background: var(--nox-bg-tertiary);
		padding: var(--nox-space-xs) var(--nox-space-sm);
		border-radius: var(--nox-radius-sm);
		font-size: 12px;
		font-family: var(--nox-font-mono);
	}

	.arrow {
		color: var(--nox-text-muted);
	}
</style>
