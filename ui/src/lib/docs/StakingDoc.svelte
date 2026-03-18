<script lang="ts">
</script>

<article class="doc-article">
	<header class="doc-header">
		<h1>Staking Guide</h1>
		<p class="tagline">Lock NOX tokens to secure the network and earn rewards.</p>
	</header>

	<section class="doc-section">
		<h2>Lock Tiers</h2>
		<p>Longer lock periods earn higher reward multipliers. All staking operations run on Sepolia testnet.</p>
		<div class="tiers-table">
			<div class="tier-row header"><span>Duration</span><span>Multiplier</span><span>Description</span></div>
			<div class="tier-row"><span>14 days</span><span class="mult">1.00x</span><span>Base rewards</span></div>
			<div class="tier-row"><span>30 days</span><span class="mult">1.25x</span><span>25% bonus</span></div>
			<div class="tier-row"><span>90 days</span><span class="mult">1.60x</span><span>60% bonus</span></div>
			<div class="tier-row"><span>180 days</span><span class="mult">2.00x</span><span>Double rewards</span></div>
			<div class="tier-row"><span>365 days</span><span class="mult">2.50x</span><span>Maximum rewards</span></div>
		</div>
	</section>

	<section class="doc-section">
		<h2>How Rewards Work</h2>
		<p>Rewards are calculated based on your weighted stake (amount × multiplier) relative to the total weighted pool. Rewards accrue per epoch and can be claimed at any time.</p>
	</section>

	<section class="doc-section">
		<h2>Early Unlock</h2>
		<p>You may unlock tokens before the lock period ends, but accumulated rewards will be forfeited. The principal amount is always returned.</p>
	</section>
</article>

<style>
	.doc-article { max-width: 800px; line-height: 1.7; }
	.doc-header { margin-bottom: 3rem; padding-bottom: 2rem; border-bottom: 1px solid #1a1a1a; }
	.doc-header h1 { font-size: 2rem; font-weight: 600; margin-bottom: 0.5rem; }
	.tagline { color: #66FFFF; font-size: 1.1rem; }
	.doc-section { margin-bottom: 2.5rem; }
	.doc-section h2 { font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #e6edf3; }
	.doc-section p { color: #9ca3af; margin-bottom: 1rem; }

	.tiers-table { display: flex; flex-direction: column; gap: 0.25rem; }
	.tier-row { display: grid; grid-template-columns: 100px 100px 1fr; gap: 1rem; padding: 0.75rem 1rem; background: #0a0a0a; border-radius: 6px; font-size: 0.875rem; }
	.tier-row.header { color: #9ca3af; font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.05em; background: none; }
	.mult { color: #66FFFF; font-weight: 600; font-family: 'SF Mono', monospace; }

	@media (max-width: 500px) {
		.tier-row { grid-template-columns: 80px 80px 1fr; font-size: 0.75rem; }
	}
</style>
