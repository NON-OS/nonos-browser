<script lang="ts">
</script>

<div class="economy-info">
	<h3>Privacy Infrastructure Economy</h3>
	<div class="info-grid">
		<div class="info-card">
			<div class="info-icon">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/>
				</svg>
			</div>
			<div class="info-content">
				<span class="info-title">Pay-Per-Use</span>
				<span class="info-desc">No subscriptions. Pay only for the privacy services you actually use.</span>
			</div>
		</div>
		<div class="info-card">
			<div class="info-icon">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
					<circle cx="9" cy="7" r="4"/>
					<path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
					<path d="M16 3.13a4 4 0 0 1 0 7.75"/>
				</svg>
			</div>
			<div class="info-content">
				<span class="info-title">Decentralized</span>
				<span class="info-desc">Services run on independent nodes. No single point of failure or control.</span>
			</div>
		</div>
		<div class="info-card">
			<div class="info-icon">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
				</svg>
			</div>
			<div class="info-content">
				<span class="info-title">Trustless</span>
				<span class="info-desc">Cryptographic proofs ensure privacy without trusting any single operator.</span>
			</div>
		</div>
		<div class="info-card">
			<div class="info-icon">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<rect x="3" y="11" width="18" height="11" rx="2"/>
					<path d="M7 11V7a5 5 0 0 1 10 0v4"/>
				</svg>
			</div>
			<div class="info-content">
				<span class="info-title">No Logs</span>
				<span class="info-desc">Nodes cannot log your activity. Architecture makes surveillance impossible.</span>
			</div>
		</div>
	</div>
</div>

<style>
	.economy-info {
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-xl);
		padding: var(--nox-space-xl);
	}

	.economy-info h3 {
		font-size: var(--nox-text-base);
		font-weight: var(--nox-font-semibold);
		margin-bottom: var(--nox-space-lg);
		text-align: center;
	}

	.info-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--nox-space-md);
	}

	@media (max-width: 600px) {
		.info-grid {
			grid-template-columns: 1fr;
		}
	}

	.info-card {
		display: flex;
		align-items: flex-start;
		gap: var(--nox-space-md);
		padding: var(--nox-space-md);
		background: var(--nox-bg-tertiary);
		border-radius: var(--nox-radius-lg);
	}

	.info-icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--nox-accent-glow);
		border-radius: var(--nox-radius-md);
		color: var(--nox-accent-primary);
		flex-shrink: 0;
	}

	.info-icon svg {
		width: 20px;
		height: 20px;
	}

	.info-content {
		display: flex;
		flex-direction: column;
		gap: var(--nox-space-2xs);
	}

	.info-title {
		font-weight: var(--nox-font-semibold);
		font-size: var(--nox-text-sm);
	}

	.info-desc {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
		line-height: 1.4;
	}
</style>
