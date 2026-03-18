<script lang="ts">
	export let address = '';

	let copied = false;

	function formatAddress(addr: string): string {
		return `${addr.slice(0, 10)}...${addr.slice(-8)}`;
	}

	async function copyAddress() {
		await navigator.clipboard.writeText(address);
		copied = true;
		setTimeout(() => copied = false, 2000);
	}
</script>

<div class="address-card">
	<div class="address-label">Wallet Address</div>
	<div class="address-value">
		<span class="address-text">{formatAddress(address)}</span>
		<button class="copy-btn" on:click={copyAddress} title="Copy full address">
			{#if copied}
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<polyline points="20 6 9 17 4 12"/>
				</svg>
			{:else}
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
					<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
				</svg>
			{/if}
		</button>
	</div>
</div>

<style>
	.address-card {
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-xl);
		padding: var(--nox-space-lg);
	}

	.address-label {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.08em;
		margin-bottom: var(--nox-space-sm);
	}

	.address-value {
		display: flex;
		align-items: center;
		gap: var(--nox-space-md);
	}

	.address-text {
		font-family: var(--nox-font-mono);
		font-size: var(--nox-text-lg);
		color: var(--nox-text-primary);
	}

	.copy-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: var(--nox-radius-md);
		color: var(--nox-text-muted);
		transition: all var(--nox-transition-fast);
	}

	.copy-btn:hover {
		background: var(--nox-bg-hover);
		color: var(--nox-accent-primary);
	}

	.copy-btn svg {
		width: 18px;
		height: 18px;
	}
</style>
