<script lang="ts">
	export let workScore: number;
	export let estimatedReward: string;
</script>

<div class="work-score-card">
	<div class="score-circle">
		<svg viewBox="0 0 100 100">
			<circle cx="50" cy="50" r="45" fill="none" stroke="var(--nox-border)" stroke-width="8" />
			<circle
				cx="50" cy="50" r="45" fill="none"
				stroke="url(#scoreGradient)" stroke-width="8"
				stroke-dasharray="{workScore * 2.83} 283"
				stroke-linecap="round"
				transform="rotate(-90 50 50)"
			/>
			<defs>
				<linearGradient id="scoreGradient">
					<stop offset="0%" stop-color="var(--nox-accent-primary)" />
					<stop offset="100%" stop-color="var(--nox-success)" />
				</linearGradient>
			</defs>
		</svg>
		<div class="score-text">
			<span class="score-value">{workScore.toFixed(1)}</span>
			<span class="score-label">Work Score</span>
		</div>
	</div>
	<div class="estimated-reward">
		<span class="reward-label">Estimated Epoch Reward</span>
		<span class="reward-value">{estimatedReward} NOX</span>
	</div>
</div>

<style>
	.work-score-card {
		display: flex;
		align-items: center;
		gap: var(--nox-space-xl);
		background: var(--nox-bg-tertiary);
		border-radius: var(--nox-radius-xl);
		padding: var(--nox-space-xl);
		margin-bottom: var(--nox-space-xl);
	}

	.score-circle {
		position: relative;
		width: 160px;
		height: 160px;
	}

	.score-circle svg {
		width: 100%;
		height: 100%;
	}

	.score-text {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.score-value {
		font-size: var(--nox-text-3xl);
		font-weight: var(--nox-font-bold);
		color: var(--nox-accent-primary);
	}

	.score-label {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
	}

	.estimated-reward {
		flex: 1;
		text-align: center;
	}

	.reward-label {
		display: block;
		font-size: var(--nox-text-sm);
		color: var(--nox-text-muted);
		margin-bottom: var(--nox-space-xs);
	}

	.reward-value {
		font-size: var(--nox-text-2xl);
		font-weight: var(--nox-font-bold);
		color: var(--nox-success);
	}
</style>
