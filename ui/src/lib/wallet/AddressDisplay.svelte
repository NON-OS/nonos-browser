<script lang="ts">
	export let address = '';
	let copied = false;

	async function copyAddress() {
		await navigator.clipboard.writeText(address);
		copied = true;
		setTimeout(() => copied = false, 2000);
	}
</script>

<div class="address-display">
	<div class="address-full">{address}</div>
	<button class="copy-address-btn" on:click={copyAddress}>
		{#if copied}
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>Copied!
		{:else}
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>Copy Address
		{/if}
	</button>
</div>

<style>
	.address-display { background: var(--nox-bg-tertiary); border: 1px solid var(--nox-border); border-radius: var(--nox-radius-lg); padding: var(--nox-space-md); margin-bottom: var(--nox-space-lg); }
	.address-full { font-family: var(--nox-font-mono); font-size: var(--nox-text-xs); word-break: break-all; color: var(--nox-text-primary); margin-bottom: var(--nox-space-md); line-height: 1.6; }
	.copy-address-btn { display: inline-flex; align-items: center; gap: var(--nox-space-sm); padding: var(--nox-space-sm) var(--nox-space-lg); background: var(--nox-accent-gradient); color: var(--nox-bg-primary); border-radius: var(--nox-radius-md); font-size: var(--nox-text-sm); font-weight: var(--nox-font-medium); transition: all var(--nox-transition-fast); }
	.copy-address-btn:hover { box-shadow: var(--nox-shadow-glow); }
	.copy-address-btn svg { width: 16px; height: 16px; }
</style>
