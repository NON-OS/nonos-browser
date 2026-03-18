<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let selectedNetwork: 'mainnet' | 'sepolia' = 'mainnet';

	function switchNetwork(network: 'mainnet' | 'sepolia') {
		dispatch('switch', { network });
	}
</script>

<div class="network-switcher">
	<button
		class="network-btn"
		class:active={selectedNetwork === 'mainnet'}
		on:click={() => switchNetwork('mainnet')}
	>
		<span class="network-dot mainnet"></span>
		Mainnet
	</button>
	<button
		class="network-btn"
		class:active={selectedNetwork === 'sepolia'}
		on:click={() => switchNetwork('sepolia')}
	>
		<span class="network-dot sepolia"></span>
		Sepolia (Staking)
	</button>
</div>

<style>
	.network-switcher {
		display: flex;
		gap: var(--nox-space-sm);
		margin-bottom: var(--nox-space-lg);
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-xs);
	}

	.network-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--nox-space-sm);
		padding: var(--nox-space-sm) var(--nox-space-md);
		border-radius: var(--nox-radius-md);
		font-size: var(--nox-text-sm);
		font-weight: var(--nox-font-medium);
		color: var(--nox-text-muted);
		background: transparent;
		transition: all var(--nox-transition-fast);
	}

	.network-btn:hover {
		color: var(--nox-text-primary);
		background: var(--nox-bg-hover);
	}

	.network-btn.active {
		color: var(--nox-text-primary);
		background: var(--nox-bg-tertiary);
	}

	.network-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}

	.network-dot.mainnet {
		background: var(--nox-success);
	}

	.network-dot.sepolia {
		background: #f59e0b;
	}
</style>
