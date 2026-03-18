<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();
	export let isLoading = false;
	let newWalletName = 'My Wallet';
</script>

<div class="setup-card">
	<div class="card-icon create"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><path d="M12 8v8M8 12h8"/></svg></div>
	<h2>Create New Wallet</h2>
	<p>Generate a new wallet with a secure 24-word mnemonic phrase and BLAKE3 key derivation.</p>
	<input type="text" bind:value={newWalletName} placeholder="Wallet name" class="input" />
	<button class="btn primary" on:click={() => dispatch('create', { name: newWalletName })} disabled={isLoading}>
		{#if isLoading}<svg class="spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>Creating...{:else}Create Wallet{/if}
	</button>
</div>

<style>
	.setup-card { background: var(--nox-bg-secondary); border: 1px solid var(--nox-border); border-radius: var(--nox-radius-xl); padding: var(--nox-space-xl); display: flex; flex-direction: column; }
	.card-icon { width: 56px; height: 56px; display: flex; align-items: center; justify-content: center; border-radius: var(--nox-radius-lg); margin-bottom: var(--nox-space-lg); background: var(--nox-accent-glow); }
	.card-icon svg { width: 28px; height: 28px; color: var(--nox-accent-primary); }
	h2 { font-size: var(--nox-text-lg); font-weight: var(--nox-font-semibold); margin-bottom: var(--nox-space-sm); }
	p { color: var(--nox-text-secondary); font-size: var(--nox-text-sm); margin-bottom: var(--nox-space-lg); line-height: 1.5; flex: 1; }
	.input { width: 100%; margin-bottom: var(--nox-space-md); }
	.btn { display: inline-flex; align-items: center; justify-content: center; gap: var(--nox-space-sm); padding: var(--nox-space-sm) var(--nox-space-lg); border-radius: var(--nox-radius-md); font-weight: var(--nox-font-medium); transition: all var(--nox-transition-fast); font-size: var(--nox-text-sm); }
	.btn svg { width: 18px; height: 18px; }
	.btn.primary { background: var(--nox-accent-gradient); color: var(--nox-bg-primary); }
	.btn.primary:hover:not(:disabled) { box-shadow: var(--nox-shadow-glow); }
	.btn:disabled { opacity: 0.5; cursor: not-allowed; }
	.spinner { animation: spin 1s linear infinite; }
	@keyframes spin { 100% { transform: rotate(360deg); } }
</style>
