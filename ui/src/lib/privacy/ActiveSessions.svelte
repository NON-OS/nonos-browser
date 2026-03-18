<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { ActiveSession } from './types';

	const dispatch = createEventDispatcher();

	export let sessions: ActiveSession[] = [];

	function formatDuration(startTime: number): string {
		const seconds = Math.floor((Date.now() - startTime) / 1000);
		const hours = Math.floor(seconds / 3600);
		const mins = Math.floor((seconds % 3600) / 60);
		return `${hours}h ${mins}m`;
	}

	function stopSession(serviceId: string) {
		dispatch('stop', { serviceId });
	}
</script>

{#if sessions.length > 0}
	<div class="active-sessions">
		<h2>Active Sessions</h2>
		<div class="sessions-list">
			{#each sessions as session}
				<div class="session-card">
					<div class="session-icon active">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
							<circle cx="12" cy="12" r="10"/>
							<path d="M2 12h20M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/>
						</svg>
					</div>
					<div class="session-info">
						<span class="session-name">{session.service}</span>
						<span class="session-duration">Running for {formatDuration(session.startTime)}</span>
					</div>
					<div class="session-stats">
						<div class="session-stat">
							<span class="value">{session.usage}</span>
							<span class="label">Used</span>
						</div>
						<div class="session-stat">
							<span class="value">{session.cost}</span>
							<span class="label">Cost</span>
						</div>
					</div>
					<button class="btn stop" on:click={() => stopSession('traffic_relay')}>
						Stop
					</button>
				</div>
			{/each}
		</div>
	</div>
{/if}

<style>
	.active-sessions {
		margin-bottom: var(--nox-space-xl);
	}

	.active-sessions h2 {
		font-size: var(--nox-text-lg);
		font-weight: var(--nox-font-semibold);
		margin-bottom: var(--nox-space-md);
	}

	.session-card {
		display: flex;
		align-items: center;
		gap: var(--nox-space-lg);
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-success);
		border-radius: var(--nox-radius-xl);
		padding: var(--nox-space-lg);
	}

	.session-icon {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--nox-bg-tertiary);
		border-radius: var(--nox-radius-lg);
		color: var(--nox-text-muted);
	}

	.session-icon.active {
		background: var(--nox-success-bg);
		color: var(--nox-success);
	}

	.session-icon svg {
		width: 24px;
		height: 24px;
	}

	.session-info {
		flex: 1;
	}

	.session-name {
		display: block;
		font-weight: var(--nox-font-semibold);
		margin-bottom: var(--nox-space-2xs);
	}

	.session-duration {
		font-size: var(--nox-text-sm);
		color: var(--nox-text-muted);
	}

	.session-stats {
		display: flex;
		gap: var(--nox-space-xl);
	}

	.session-stat {
		text-align: center;
	}

	.session-stat .value {
		display: block;
		font-family: var(--nox-font-mono);
		font-weight: var(--nox-font-semibold);
	}

	.session-stat .label {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
	}

	.btn.stop {
		padding: var(--nox-space-sm) var(--nox-space-lg);
		background: var(--nox-error);
		color: white;
		border: none;
		border-radius: var(--nox-radius-md);
		font-weight: var(--nox-font-medium);
		cursor: pointer;
	}

	@media (max-width: 600px) {
		.session-card {
			flex-wrap: wrap;
		}

		.session-stats {
			width: 100%;
			justify-content: space-around;
			margin: var(--nox-space-md) 0;
		}
	}
</style>
