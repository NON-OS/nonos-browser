<script lang="ts">
	export let qrCodeDataUrl = '';
</script>

<div class="qr-code-container">
	{#if qrCodeDataUrl && qrCodeDataUrl.startsWith('data:')}
		<img src={qrCodeDataUrl} alt="Scan to receive tokens" class="qr-code-img" />
	{:else}
		<div class="qr-loading">
			<svg class="spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/>
			</svg>
			<span>Generating QR...</span>
		</div>
	{/if}
</div>

<style>
	.qr-code-container { width: 220px; height: 220px; margin: 0 auto var(--nox-space-lg); background: var(--nox-bg-tertiary); border: 1px solid var(--nox-border); border-radius: var(--nox-radius-lg); display: flex; align-items: center; justify-content: center; padding: var(--nox-space-sm); }
	.qr-code-img { width: 200px; height: 200px; border-radius: var(--nox-radius-md); }
	.qr-loading { display: flex; flex-direction: column; align-items: center; gap: var(--nox-space-sm); color: var(--nox-text-muted); }
	.qr-loading svg { width: 32px; height: 32px; }
	.qr-loading span { font-size: var(--nox-text-xs); }
	.spinner { animation: spin 1s linear infinite; }
	@keyframes spin { 100% { transform: rotate(360deg); } }
</style>
