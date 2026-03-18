<script lang="ts">
	export let stakingStatus = { staked_amount: '0', tier: 'None', tier_multiplier: '0x', pending_rewards: '0', estimated_apy: '0%' };
</script>

<div class="staking-stats">
	<div class="stat-card">
		<div class="stat-label">Staked Amount</div>
		<div class="stat-value">{stakingStatus.staked_amount} <span class="unit">NOX</span></div>
	</div>
	<div class="stat-card tier-card">
		<div class="stat-label">Current Tier</div>
		<div class="stat-value tier-{stakingStatus.tier.toLowerCase()}">{stakingStatus.tier}</div>
		<div class="stat-sub">{stakingStatus.tier_multiplier} rewards</div>
	</div>
	<div class="stat-card">
		<div class="stat-label">Pending Rewards</div>
		<div class="stat-value reward">{stakingStatus.pending_rewards} <span class="unit">NOX</span></div>
	</div>
	<div class="stat-card">
		<div class="stat-label">Est. APY</div>
		<div class="stat-value apy">{stakingStatus.estimated_apy}</div>
	</div>
</div>

<style>
	.staking-stats { display: grid; grid-template-columns: repeat(4, 1fr); gap: var(--nox-space-md); margin-bottom: var(--nox-space-lg); }
	@media (max-width: 700px) { .staking-stats { grid-template-columns: repeat(2, 1fr); } }
	.stat-card { background: var(--nox-bg-tertiary); border-radius: var(--nox-radius-lg); padding: var(--nox-space-md); text-align: center; }
	.stat-label { font-size: var(--nox-text-xs); color: var(--nox-text-muted); margin-bottom: var(--nox-space-xs); }
	.stat-value { font-size: var(--nox-text-lg); font-weight: var(--nox-font-semibold); font-family: var(--nox-font-mono); }
	.stat-value .unit { font-size: var(--nox-text-xs); font-weight: var(--nox-font-regular); color: var(--nox-text-muted); }
	.stat-value.reward { color: var(--nox-success); }
	.stat-value.apy { color: var(--nox-accent-primary); }
	.stat-sub { font-size: var(--nox-text-xs); color: var(--nox-text-muted); margin-top: var(--nox-space-2xs); }
	.tier-bronze { color: #cd7f32; } .tier-silver { color: #c0c0c0; } .tier-gold { color: #ffd700; } .tier-platinum { color: #e5e4e2; } .tier-diamond { color: #b9f2ff; }
</style>
