<script lang="ts">
	export let mnemonic = '';
	let copied = false;

	async function copyMnemonic() {
		await navigator.clipboard.writeText(mnemonic);
		copied = true;
		setTimeout(() => copied = false, 2000);
	}
</script>

<div class="mnemonic-card">
	<div class="card-header">
		<h3>Recovery Phrase (24 words)</h3>
		<button class="copy-action" on:click={copyMnemonic}>
			{#if copied}<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>Copied
			{:else}<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>Copy{/if}
		</button>
	</div>
	<div class="mnemonic-words">
		{#each mnemonic.split(' ') as word, i}<span class="word"><span class="num">{i + 1}</span>{word}</span>{/each}
	</div>
</div>

<style>
	.mnemonic-card { background: var(--nox-bg-secondary); border: 1px solid var(--nox-border); border-radius: var(--nox-radius-xl); padding: var(--nox-space-lg); }
	.card-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--nox-space-md); }
	.card-header h3 { font-size: var(--nox-text-base); font-weight: var(--nox-font-medium); }
	.copy-action { display: inline-flex; align-items: center; gap: var(--nox-space-xs); padding: var(--nox-space-xs) var(--nox-space-sm); border-radius: var(--nox-radius-sm); font-size: var(--nox-text-xs); color: var(--nox-text-muted); transition: all var(--nox-transition-fast); }
	.copy-action:hover { background: var(--nox-bg-hover); color: var(--nox-accent-primary); }
	.copy-action svg { width: 14px; height: 14px; }
	.mnemonic-words { display: grid; grid-template-columns: repeat(4, 1fr); gap: var(--nox-space-sm); }
	@media (max-width: 600px) { .mnemonic-words { grid-template-columns: repeat(3, 1fr); } }
	.word { display: flex; align-items: center; gap: var(--nox-space-sm); background: var(--nox-bg-tertiary); padding: var(--nox-space-sm) var(--nox-space-md); border-radius: var(--nox-radius-md); font-family: var(--nox-font-mono); font-size: var(--nox-text-sm); }
	.word .num { color: var(--nox-text-disabled); font-size: var(--nox-text-xs); min-width: 20px; }
</style>
