<script lang="ts">
	export let walletAddress: string;

	function formatAddress(addr: string): string {
		if (!addr) return '';
		return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
	}
</script>

{#if walletAddress}
	<a href="/wallet" class="wallet-pill">
		<span class="wallet-dot"></span>
		<span class="wallet-address">{formatAddress(walletAddress)}</span>
	</a>
{:else}
	<a href="/wallet" class="wallet-pill empty">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
			<rect x="2" y="6" width="20" height="14" rx="2"/>
			<path d="M2 10h20"/>
		</svg>
		<span>Connect</span>
	</a>
{/if}

<style>
	.wallet-pill {
		display: flex;
		align-items: center;
		gap: var(--nox-space-sm);
		padding: var(--nox-space-xs) var(--nox-space-md);
		background: var(--nox-bg-tertiary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-full);
		font-size: var(--nox-text-sm);
		color: var(--nox-text-secondary);
		text-decoration: none;
		transition: all var(--nox-transition-fast);
	}

	.wallet-pill:hover {
		border-color: var(--nox-accent-primary);
		color: var(--nox-text-primary);
	}

	.wallet-pill svg {
		width: 16px;
		height: 16px;
	}

	.wallet-pill.empty {
		background: var(--nox-accent-gradient);
		border: none;
		color: var(--nox-bg-primary);
		font-weight: var(--nox-font-medium);
	}

	.wallet-pill.empty:hover {
		box-shadow: var(--nox-shadow-glow);
	}

	.wallet-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--nox-success);
		box-shadow: 0 0 6px var(--nox-success);
	}

	.wallet-address {
		font-family: var(--nox-font-mono);
		font-size: var(--nox-text-xs);
	}
</style>
