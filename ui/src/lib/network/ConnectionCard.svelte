<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { NetworkStatus } from './types';
	import StatusBadge from './StatusBadge.svelte';
	import ArchitectureVisual from './ArchitectureVisual.svelte';
	import ConnectionStats from './ConnectionStats.svelte';

	const dispatch = createEventDispatcher();
	export let networkStatus: NetworkStatus;
	export let isConnecting: boolean;
</script>

<div class="connection-card">
	<div class="connection-header"><StatusBadge connected={networkStatus.connected} /></div>
	<ArchitectureVisual />

	{#if !networkStatus.connected && networkStatus.bootstrap_progress > 0}
		<div class="bootstrap-section">
			<div class="bootstrap-label">Bootstrapping... {networkStatus.status}</div>
			<div class="progress-bar"><div class="progress-fill" style="width: {networkStatus.bootstrap_progress}%"></div></div>
			<div class="progress-text">{networkStatus.bootstrap_progress}%</div>
		</div>
	{/if}

	{#if networkStatus.error}<div class="error-section"><span class="error-text">{networkStatus.error}</span></div>{/if}

	<ConnectionStats circuits={networkStatus.circuits} />

	<div class="connection-actions">
		{#if networkStatus.connected}
			<button class="btn danger" on:click={() => dispatch('disconnect')}>Disconnect</button>
			<button class="btn secondary" on:click={() => dispatch('newIdentity')}>New Identity</button>
		{:else}
			<button class="btn primary" on:click={() => dispatch('connect')} disabled={isConnecting}>{isConnecting ? 'Connecting...' : 'Connect to Nym Mixnet'}</button>
		{/if}
	</div>
</div>

<style>
	.connection-card { background: var(--nox-bg-secondary); border: 1px solid var(--nox-border); border-radius: var(--nox-radius-lg); padding: var(--nox-space-lg); margin-bottom: var(--nox-space-xl); }
	.connection-header { margin-bottom: var(--nox-space-lg); }
	.bootstrap-section { margin-bottom: var(--nox-space-lg); }
	.bootstrap-label { font-size: 13px; color: var(--nox-text-secondary); margin-bottom: var(--nox-space-xs); }
	.progress-bar { height: 8px; background: var(--nox-bg-tertiary); border-radius: var(--nox-radius-full); overflow: hidden; margin-bottom: var(--nox-space-xs); }
	.progress-fill { height: 100%; background: var(--nox-accent-gradient); transition: width var(--nox-transition-normal); }
	.progress-text { font-size: 12px; color: var(--nox-text-muted); font-family: var(--nox-font-mono); }
	.error-section { background: rgba(239, 68, 68, 0.1); border: 1px solid var(--nox-error); border-radius: var(--nox-radius-md); padding: var(--nox-space-md); margin-bottom: var(--nox-space-lg); }
	.error-text { color: var(--nox-error); font-size: 13px; font-family: var(--nox-font-mono); }
	.connection-actions { display: flex; gap: var(--nox-space-md); }
	.btn { padding: var(--nox-space-sm) var(--nox-space-lg); border-radius: var(--nox-radius-md); font-weight: 500; transition: all var(--nox-transition-fast); cursor: pointer; }
	.btn.primary { background: var(--nox-accent-gradient); color: white; flex: 1; }
	.btn.primary:hover { filter: brightness(1.1); }
	.btn.secondary { background: var(--nox-bg-tertiary); border: 1px solid var(--nox-border); }
	.btn.danger { background: var(--nox-error); color: white; }
	.btn:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
