<script lang="ts">
	const workCategories = [
		{ name: 'Traffic Relay', weight: '30%', desc: 'Bandwidth relayed through circuits' },
		{ name: 'ZK Proof Generation', weight: '25%', desc: 'Zero-knowledge proofs computed' },
		{ name: 'Mixer Operations', weight: '20%', desc: 'Participation in mixing rounds' },
		{ name: 'Uptime', weight: '15%', desc: 'Node availability percentage' },
		{ name: 'Registry Operations', weight: '10%', desc: 'Stealth address registrations' }
	];
</script>

<div class="work-section">
	<h2>Work Categories</h2>
	<p class="section-desc">
		Node operators earn rewards based on measurable privacy work performed, not tokens staked. Work is tracked across five categories:
	</p>

	<div class="work-grid">
		{#each workCategories as category}
			<div class="work-card">
				<div class="work-header">
					<span class="work-name">{category.name}</span>
					<span class="work-weight">{category.weight}</span>
				</div>
				<div class="work-desc">{category.desc}</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.work-section {
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

	.work-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		gap: var(--nox-space-md);
	}

	.work-card {
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-md);
		transition: all var(--nox-transition-fast);
	}

	.work-card:hover {
		border-color: var(--nox-accent-primary);
	}

	.work-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: var(--nox-space-sm);
	}

	.work-name {
		font-weight: 600;
		font-size: 13px;
	}

	.work-weight {
		font-family: var(--nox-font-mono);
		font-size: 12px;
		font-weight: 600;
		color: var(--nox-accent-primary);
		background: var(--nox-accent-glow);
		padding: 2px 8px;
		border-radius: var(--nox-radius-sm);
	}

	.work-desc {
		font-size: 11px;
		color: var(--nox-text-muted);
		line-height: 1.4;
	}
</style>
