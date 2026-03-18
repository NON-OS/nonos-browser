<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();
	export let isLoading = false;
	let importMnemonic = '';
	let walletName = 'My Wallet';
</script>

<div class="setup-card">
	<div class="card-icon import"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg></div>
	<h2>Import Wallet</h2>
	<p>Restore a wallet using your 24-word mnemonic recovery phrase.</p>
	<textarea bind:value={importMnemonic} placeholder="Enter your 24-word mnemonic phrase..." rows="3" class="textarea"></textarea>
	<button class="btn secondary" on:click={() => dispatch('import', { mnemonic: importMnemonic.trim(), name: walletName })} disabled={isLoading || !importMnemonic}>
		{isLoading ? 'Importing...' : 'Import Wallet'}
	</button>
</div>

<style>
	.setup-card { background: var(--nox-bg-secondary); border: 1px solid var(--nox-border); border-radius: var(--nox-radius-xl); padding: var(--nox-space-xl); display: flex; flex-direction: column; }
	.card-icon { width: 56px; height: 56px; display: flex; align-items: center; justify-content: center; border-radius: var(--nox-radius-lg); margin-bottom: var(--nox-space-lg); background: var(--nox-success-bg); }
	.card-icon svg { width: 28px; height: 28px; color: var(--nox-success); }
	h2 { font-size: var(--nox-text-lg); font-weight: var(--nox-font-semibold); margin-bottom: var(--nox-space-sm); }
	p { color: var(--nox-text-secondary); font-size: var(--nox-text-sm); margin-bottom: var(--nox-space-lg); line-height: 1.5; flex: 1; }
	.textarea { width: 100%; margin-bottom: var(--nox-space-md); resize: none; }
	.btn { display: inline-flex; align-items: center; justify-content: center; gap: var(--nox-space-sm); padding: var(--nox-space-sm) var(--nox-space-lg); border-radius: var(--nox-radius-md); font-weight: var(--nox-font-medium); transition: all var(--nox-transition-fast); font-size: var(--nox-text-sm); }
	.btn.secondary { background: var(--nox-bg-tertiary); border: 1px solid var(--nox-border); color: var(--nox-text-primary); }
	.btn.secondary:hover:not(:disabled) { border-color: var(--nox-accent-primary); }
	.btn:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
