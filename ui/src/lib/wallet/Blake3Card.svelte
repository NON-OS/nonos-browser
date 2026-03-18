<script lang="ts">
	export let blake3Key = '';
	let copiedBlake3 = false;

	async function copyBlake3() {
		await navigator.clipboard.writeText(blake3Key);
		copiedBlake3 = true;
		setTimeout(() => copiedBlake3 = false, 2000);
	}
</script>

<div class="blake3-card">
	<div class="card-header">
		<h3>BLAKE3 Key (Alternative Recovery)</h3>
		<button class="copy-action" on:click={copyBlake3}>
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
			{copiedBlake3 ? 'Copied' : 'Copy'}
		</button>
	</div>
	<p class="key-value">{blake3Key}</p>
</div>

<style>
	.blake3-card { background: var(--nox-bg-secondary); border: 1px solid var(--nox-border); border-radius: var(--nox-radius-xl); padding: var(--nox-space-lg); }
	.card-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--nox-space-md); }
	.card-header h3 { font-size: var(--nox-text-base); font-weight: var(--nox-font-medium); }
	.copy-action { display: inline-flex; align-items: center; gap: var(--nox-space-xs); padding: var(--nox-space-xs) var(--nox-space-sm); border-radius: var(--nox-radius-sm); font-size: var(--nox-text-xs); color: var(--nox-text-muted); transition: all var(--nox-transition-fast); }
	.copy-action:hover { background: var(--nox-bg-hover); color: var(--nox-accent-primary); }
	.copy-action svg { width: 14px; height: 14px; }
	.key-value { font-family: var(--nox-font-mono); font-size: var(--nox-text-xs); word-break: break-all; background: var(--nox-bg-tertiary); padding: var(--nox-space-md); border-radius: var(--nox-radius-md); color: var(--nox-text-secondary); line-height: 1.6; }
</style>
