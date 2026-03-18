<script lang="ts">
	export let circuits: number;

	async function handleNewCircuit() {
		try {
			if (window.nonos) {
				await window.nonos.network.newIdentity();
			}
		} catch (e) {
			console.error('Failed to create new identity:', e);
		}
	}
</script>

<button class="action-btn identity-btn" on:click={handleNewCircuit} title="New Identity (New Circuit)">
	<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
		<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
	</svg>
	{#if circuits > 0}
		<span class="circuit-badge">{circuits}</span>
	{/if}
</button>

<style>
	.action-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: var(--nox-radius-md);
		color: var(--nox-text-muted);
		background: var(--nox-bg-tertiary);
		border: 1px solid var(--nox-border);
		transition: all var(--nox-transition-fast);
	}

	.action-btn svg {
		width: 18px;
		height: 18px;
	}

	.action-btn:hover {
		border-color: var(--nox-accent-primary);
		color: var(--nox-accent-primary);
	}

	.identity-btn {
		position: relative;
	}

	.circuit-badge {
		position: absolute;
		top: -4px;
		right: -4px;
		min-width: 18px;
		height: 18px;
		padding: 0 5px;
		background: var(--nox-accent-primary);
		border-radius: var(--nox-radius-full);
		font-size: 10px;
		font-weight: var(--nox-font-semibold);
		color: var(--nox-bg-primary);
		display: flex;
		align-items: center;
		justify-content: center;
		font-family: var(--nox-font-mono);
	}
</style>
