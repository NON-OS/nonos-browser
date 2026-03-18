<script lang="ts">
	export let value = 0;
	export let max = 100;
	export let variant: 'default' | 'success' | 'warning' | 'error' = 'default';
	export let size: 'sm' | 'md' | 'lg' = 'md';
	export let showLabel = false;
	export let animated = false;

	$: percent = Math.min(100, Math.max(0, (value / max) * 100));
</script>

<div class="progress-container">
	<div class="track track-{size}">
		<div class="bar bar-{variant}" class:animated style="width: {percent}%"></div>
	</div>
	{#if showLabel}
		<span class="label">{Math.round(percent)}%</span>
	{/if}
</div>

<style>
	.progress-container {
		display: flex;
		align-items: center;
		gap: 8px;
	}
	.track {
		flex: 1;
		background: #1a1a26;
		border-radius: 4px;
		overflow: hidden;
	}
	.track-sm { height: 4px; }
	.track-md { height: 8px; }
	.track-lg { height: 12px; }
	.bar {
		height: 100%;
		transition: width 0.3s ease;
		border-radius: 4px;
	}
	.bar-default { background: linear-gradient(90deg, #66FFFF, #2E5C5C); }
	.bar-success { background: linear-gradient(90deg, #00ff88, #00aa55); }
	.bar-warning { background: linear-gradient(90deg, #ffaa00, #ff7700); }
	.bar-error { background: linear-gradient(90deg, #ff4466, #aa2244); }
	.animated {
		background-size: 20px 20px;
		animation: progress-flow 1s linear infinite;
	}
	.bar-default.animated {
		background-image: linear-gradient(
			45deg,
			rgba(255, 255, 255, 0.1) 25%,
			transparent 25%,
			transparent 50%,
			rgba(255, 255, 255, 0.1) 50%,
			rgba(255, 255, 255, 0.1) 75%,
			transparent 75%
		);
	}
	.label { font-size: 12px; color: #a0a0b8; min-width: 40px; }
	@keyframes progress-flow {
		from { background-position: 0 0; }
		to { background-position: 20px 0; }
	}
</style>
