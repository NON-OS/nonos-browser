<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import MnemonicWords from './MnemonicWords.svelte';
	import Blake3Card from './Blake3Card.svelte';

	const dispatch = createEventDispatcher();
	export let mnemonic = '';
	export let blake3Key = '';
</script>

<div class="mnemonic-display">
	<div class="warning-banner">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
		<div>
			<strong>Critical Security Information</strong>
			<p>Write down your recovery phrase and store it securely offline. This is the only way to recover your wallet if you lose access. Never share it with anyone.</p>
		</div>
	</div>
	<MnemonicWords {mnemonic} />
	<Blake3Card {blake3Key} />
	<button class="btn primary full-width" on:click={() => dispatch('confirm')}>
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
		I've Saved My Recovery Phrase
	</button>
</div>

<style>
	.mnemonic-display { display: flex; flex-direction: column; gap: var(--nox-space-lg); }
	.warning-banner { display: flex; gap: var(--nox-space-md); background: var(--nox-warning-bg); border: 1px solid var(--nox-warning); padding: var(--nox-space-lg); border-radius: var(--nox-radius-lg); }
	.warning-banner svg { width: 24px; height: 24px; color: var(--nox-warning); flex-shrink: 0; }
	.warning-banner strong { display: block; color: var(--nox-warning); margin-bottom: var(--nox-space-xs); }
	.warning-banner p { font-size: var(--nox-text-sm); color: var(--nox-text-secondary); line-height: 1.5; }
	.btn { display: inline-flex; align-items: center; justify-content: center; gap: var(--nox-space-sm); padding: var(--nox-space-sm) var(--nox-space-lg); border-radius: var(--nox-radius-md); font-weight: var(--nox-font-medium); transition: all var(--nox-transition-fast); font-size: var(--nox-text-sm); }
	.btn svg { width: 18px; height: 18px; }
	.btn.primary { background: var(--nox-accent-gradient); color: var(--nox-bg-primary); }
	.btn.primary:hover:not(:disabled) { box-shadow: var(--nox-shadow-glow); }
	.btn.full-width { width: 100%; }
</style>
