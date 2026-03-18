<script lang="ts">
	export let label: string;
	export let value: string | number;
	export let subtext = '';
	export let variant: 'default' | 'success' | 'warning' | 'error' | 'info' = 'default';
	export let icon = '';
</script>

<div class="stat stat-{variant}">
	{#if icon}
		<span class="icon">{icon}</span>
	{/if}
	<div class="content">
		<span class="label">{label}</span>
		<span class="value">{value}</span>
		{#if subtext}
			<span class="subtext">{subtext}</span>
		{/if}
	</div>
</div>

<style>
	.stat {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		padding: 16px;
		background: #1a1a26;
		border: 1px solid #2a2a40;
		border-radius: 8px;
	}
	.icon { font-size: 24px; }
	.content {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}
	.label {
		font-size: 12px;
		color: #6b6b80;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}
	.value {
		font-size: 24px;
		font-weight: 600;
		color: white;
	}
	.subtext { font-size: 12px; color: #a0a0b8; }
	.stat-success .value { color: #00ff88; }
	.stat-warning .value { color: #ffaa00; }
	.stat-error .value { color: #ff4466; }
	.stat-info .value { color: #66FFFF; }
	.stat-success { border-color: rgba(0, 255, 136, 0.2); }
	.stat-info { border-color: rgba(102, 255, 255, 0.2); }
</style>
