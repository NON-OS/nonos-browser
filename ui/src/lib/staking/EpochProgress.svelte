<script lang="ts">
	export let currentEpoch: number;
	export let epochProgress: number;
	export let epochTimeRemaining: string;
</script>

<div class="epoch-progress">
	<div class="epoch-header">
		<span>Epoch {currentEpoch}</span>
		<span class="epoch-time">{epochTimeRemaining} remaining</span>
	</div>
	<div class="progress-bar">
		<div class="progress-fill" style="width: {epochProgress}%"></div>
	</div>
</div>

<style>
	.epoch-progress {
		margin-bottom: var(--nox-space-xl);
	}

	.epoch-header {
		display: flex;
		justify-content: space-between;
		margin-bottom: var(--nox-space-sm);
		font-size: var(--nox-text-sm);
	}

	.epoch-time {
		color: var(--nox-text-muted);
	}

	.progress-bar {
		height: 8px;
		background: var(--nox-bg-tertiary);
		border-radius: var(--nox-radius-full);
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--nox-accent-gradient);
		border-radius: var(--nox-radius-full);
		transition: width 0.3s ease;
	}
</style>
