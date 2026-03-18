<script lang="ts">
	export let connected = false;
</script>

<div class="status-badge" class:connected>
	<span class="dot"></span>
	{connected ? 'Connected to Nym Mixnet' : 'Disconnected'}
</div>

<style>
	.status-badge { display: inline-flex; align-items: center; gap: var(--nox-space-sm); padding: var(--nox-space-sm) var(--nox-space-md); background: rgba(239, 68, 68, 0.1); border-radius: var(--nox-radius-full); font-size: 14px; color: var(--nox-error); }
	.status-badge.connected { background: rgba(34, 197, 94, 0.1); color: var(--nox-success); }
	.status-badge .dot { width: 10px; height: 10px; border-radius: 50%; background: currentColor; animation: pulse 2s infinite; }
	@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }
</style>
