<script lang="ts">
	export let totalPendingRewards = '0';
	export let estimatedReward = '0';
</script>

<div class="rewards-breakdown">
	<div class="breakdown-card">
		<h4>Staking Pool Rewards</h4>
		<p class="breakdown-desc">15% of service fees + bootstrap emissions to stakers</p>
		<div class="breakdown-stat"><span>Your Share</span><span>{totalPendingRewards} NOX</span></div>
	</div>
	<div class="breakdown-card">
		<h4>Node Work Rewards</h4>
		<p class="breakdown-desc">50% of service fees distributed by work score</p>
		<div class="breakdown-stat"><span>Estimated This Epoch</span><span>{estimatedReward} NOX</span></div>
	</div>
</div>

<style>
	.rewards-breakdown { display: grid; grid-template-columns: repeat(2, 1fr); gap: var(--nox-space-md); margin-bottom: var(--nox-space-xl); }
	.breakdown-card { background: var(--nox-bg-tertiary); border-radius: var(--nox-radius-lg); padding: var(--nox-space-lg); }
	.breakdown-card h4 { font-size: var(--nox-text-base); margin-bottom: var(--nox-space-xs); }
	.breakdown-desc { font-size: var(--nox-text-xs); color: var(--nox-text-muted); margin-bottom: var(--nox-space-md); }
	.breakdown-stat { display: flex; justify-content: space-between; font-size: var(--nox-text-sm); }
	.breakdown-stat span:last-child { font-weight: var(--nox-font-semibold); color: var(--nox-accent-primary); }
	@media (max-width: 500px) { .rewards-breakdown { grid-template-columns: 1fr; } }
</style>
