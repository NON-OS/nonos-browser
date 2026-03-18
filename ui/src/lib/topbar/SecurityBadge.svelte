<script lang="ts">
	export let connected: boolean;
</script>

<div class="security-badge" class:secure={connected}>
	{#if connected}
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
			<path d="M9 12l2 2 4-4" />
		</svg>
		<span class="security-text">Secure</span>
	{:else}
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<circle cx="12" cy="12" r="10" />
			<path d="M12 8v4M12 16h.01" />
		</svg>
		<span class="security-text">Connecting...</span>
	{/if}
</div>

<style>
	.security-badge {
		display: flex;
		align-items: center;
		gap: var(--nox-space-xs);
		padding: var(--nox-space-2xs) var(--nox-space-sm);
		background: var(--nox-warning-bg);
		border-radius: var(--nox-radius-md);
		color: var(--nox-warning);
		font-size: var(--nox-text-xs);
		font-weight: var(--nox-font-medium);
		white-space: nowrap;
	}

	.security-badge svg {
		width: 14px;
		height: 14px;
	}

	.security-badge.secure {
		background: var(--nox-success-bg);
		color: var(--nox-success);
	}

	.security-text {
		display: none;
	}

	@media (min-width: 800px) {
		.security-text {
			display: inline;
		}
	}
</style>
