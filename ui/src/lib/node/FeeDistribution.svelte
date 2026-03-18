<script lang="ts">
	const feeDistribution = [
		{ recipient: 'Node Operators', share: '50%', desc: 'Work-based rewards (not stake-weighted)' },
		{ recipient: 'ZeroState Pass', share: '35%', desc: 'NFT holders who stake NOX' },
		{ recipient: 'Staking Pool', share: '15%', desc: 'Perpetual pool refill' }
	];
</script>

<div class="staking-section">
	<h2>Privacy Work Economy</h2>
	<p class="section-desc">
		All service fees are distributed across three stakeholder classes. Node operators earn based on work performed, not tokens staked. The 50/35/15 split is immutable and hardcoded.
	</p>

	<div class="distribution-grid">
		{#each feeDistribution as item}
			<div class="distribution-card">
				<div class="distribution-share">{item.share}</div>
				<div class="distribution-recipient">{item.recipient}</div>
				<div class="distribution-desc">{item.desc}</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.staking-section {
		margin-bottom: var(--nox-space-xl);
	}

	h2 {
		font-size: 18px;
		font-weight: 600;
		margin-bottom: var(--nox-space-md);
	}

	.section-desc {
		color: var(--nox-text-secondary);
		font-size: 14px;
		margin-bottom: var(--nox-space-lg);
	}

	.distribution-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: var(--nox-space-md);
	}

	.distribution-card {
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-lg);
		text-align: center;
		transition: all var(--nox-transition-fast);
	}

	.distribution-card:hover {
		border-color: var(--nox-accent-primary);
	}

	.distribution-share {
		font-size: 32px;
		font-weight: 700;
		color: var(--nox-accent-primary);
		font-family: var(--nox-font-mono);
		margin-bottom: var(--nox-space-sm);
	}

	.distribution-recipient {
		font-size: 14px;
		font-weight: 600;
		margin-bottom: var(--nox-space-xs);
	}

	.distribution-desc {
		font-size: 11px;
		color: var(--nox-text-muted);
	}
</style>
