<script lang="ts">
	export let currentTier = 'None';
	export let nextTierThreshold = '1,000 NOX';
</script>

<div class="tier-info">
	<h3>Staking Tiers</h3>
	<div class="tier-table">
		<div class="tier-row header"><span>Tier</span><span>Min Stake</span><span>Multiplier</span></div>
		<div class="tier-row" class:active={currentTier === 'Bronze'}><span>Bronze</span><span>1,000 NOX</span><span>1.0x</span></div>
		<div class="tier-row" class:active={currentTier === 'Silver'}><span>Silver</span><span>10,000 NOX</span><span>1.2x</span></div>
		<div class="tier-row" class:active={currentTier === 'Gold'}><span>Gold</span><span>50,000 NOX</span><span>1.5x</span></div>
		<div class="tier-row" class:active={currentTier === 'Platinum'}><span>Platinum</span><span>200,000 NOX</span><span>2.0x</span></div>
		<div class="tier-row" class:active={currentTier === 'Diamond'}><span>Diamond</span><span>1,000,000 NOX</span><span>2.5x</span></div>
	</div>
	<p class="tier-note">Next tier: {nextTierThreshold}</p>
</div>

<style>
	.tier-info { border-top: 1px solid var(--nox-border); padding-top: var(--nox-space-lg); }
	.tier-info h3 { font-size: var(--nox-text-base); font-weight: var(--nox-font-medium); margin-bottom: var(--nox-space-md); }
	.tier-table { background: var(--nox-bg-tertiary); border-radius: var(--nox-radius-md); overflow: hidden; }
	.tier-row { display: grid; grid-template-columns: 1fr 1fr 1fr; padding: var(--nox-space-sm) var(--nox-space-md); border-bottom: 1px solid var(--nox-border); font-size: var(--nox-text-sm); }
	.tier-row:last-child { border-bottom: none; }
	.tier-row.header { background: var(--nox-bg-secondary); font-weight: var(--nox-font-medium); color: var(--nox-text-muted); }
	.tier-row.active { background: var(--nox-accent-glow); color: var(--nox-accent-primary); }
	.tier-note { margin-top: var(--nox-space-md); font-size: var(--nox-text-xs); color: var(--nox-text-muted); }
</style>
