<script lang="ts">
	export let circuits = 0;
</script>

<div class="connection-stats">
	<div class="stat">
		<div class="stat-label">Active Circuits</div>
		<div class="stat-value">{circuits}</div>
	</div>
	<div class="stat">
		<div class="stat-label">Mixnet Hops</div>
		<div class="stat-value">5</div>
	</div>
	<div class="stat">
		<div class="stat-label">SOCKS Port</div>
		<div class="stat-value">1080</div>
	</div>
</div>

<style>
	.connection-stats { display: flex; gap: var(--nox-space-md); margin-bottom: var(--nox-space-lg); }
	.stat { background: var(--nox-bg-tertiary); padding: var(--nox-space-md); border-radius: var(--nox-radius-md); flex: 1; text-align: center; }
	.stat-label { font-size: 11px; color: var(--nox-text-muted); margin-bottom: var(--nox-space-xs); text-transform: uppercase; letter-spacing: 0.5px; }
	.stat-value { font-size: 20px; font-weight: 600; font-family: var(--nox-font-mono); color: var(--nox-accent); }
</style>
