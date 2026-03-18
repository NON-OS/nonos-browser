<script lang="ts">
	export let variant: 'default' | 'success' | 'warning' | 'error' | 'info' = 'default';
	export let size: 'sm' | 'md' = 'md';
	export let glow = false;
</script>

<span class="badge badge-{variant} badge-{size}" class:glow>
	<slot />
</span>

<style>
	.badge {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		font-weight: 500;
		border-radius: 4px;
		white-space: nowrap;
	}
	.badge-sm { padding: 2px 6px; font-size: 11px; }
	.badge-md { padding: 4px 10px; font-size: 12px; }
	.badge-default {
		background: rgba(160, 160, 184, 0.15);
		color: #a0a0b8;
	}
	.badge-success {
		background: rgba(0, 255, 136, 0.15);
		color: #00ff88;
	}
	.badge-warning {
		background: rgba(255, 170, 0, 0.15);
		color: #ffaa00;
	}
	.badge-error {
		background: rgba(255, 68, 102, 0.15);
		color: #ff4466;
	}
	.badge-info {
		background: rgba(102, 255, 255, 0.15);
		color: #66FFFF;
	}
	.glow.badge-success { box-shadow: 0 0 10px rgba(0, 255, 136, 0.3); }
	.glow.badge-info { box-shadow: 0 0 10px rgba(102, 255, 255, 0.3); }
	.glow.badge-error { box-shadow: 0 0 10px rgba(255, 68, 102, 0.3); }
</style>
