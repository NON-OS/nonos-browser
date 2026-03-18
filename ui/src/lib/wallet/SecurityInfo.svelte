<script lang="ts">
</script>

<div class="security-info">
	<div class="info-item">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
			<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
		</svg>
		<span>BLAKE3 key derivation</span>
	</div>
	<div class="info-item">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
			<circle cx="12" cy="12" r="10"/>
			<path d="M12 6v6l4 2"/>
		</svg>
		<span>Stealth addresses</span>
	</div>
	<div class="info-item">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
			<path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"/>
		</svg>
		<span>secp256k1 signatures</span>
	</div>
</div>

<style>
	.security-info {
		display: flex;
		justify-content: center;
		gap: var(--nox-space-xl);
		padding: var(--nox-space-lg);
		background: var(--nox-bg-tertiary);
		border-radius: var(--nox-radius-lg);
	}

	.info-item {
		display: flex;
		align-items: center;
		gap: var(--nox-space-sm);
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
	}

	.info-item svg {
		width: 16px;
		height: 16px;
		color: var(--nox-accent-primary);
	}
</style>
