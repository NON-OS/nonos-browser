<script lang="ts">
	import type { LPLockInfo } from './types';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let myLocks: LPLockInfo[];

	function handleUnlock(lockId: number) {
		dispatch('unlock', { lockId });
	}

	function handleClaim(lockId: number) {
		dispatch('claim', { lockId });
	}
</script>

{#if myLocks.length > 0}
	<div class="my-locks">
		<h3>Your Active Locks</h3>
		<div class="locks-list">
			{#each myLocks as lock}
				<div class="lock-card">
					<div class="lock-info">
						<div class="lock-amount">{lock.amount} NOX</div>
						<div class="lock-tier">{lock.tier_name} - {lock.multiplier}</div>
					</div>
					<div class="lock-rewards">
						<div class="rewards-label">Pending</div>
						<div class="rewards-value">{lock.pending_rewards} NOX</div>
					</div>
					<div class="lock-actions">
						{#if lock.is_locked}
							<span class="locked-badge">Locked</span>
						{:else}
							<button class="btn small" on:click={() => handleUnlock(lock.lock_id)}>Unlock</button>
						{/if}
						<button class="btn small secondary" on:click={() => handleClaim(lock.lock_id)}>Claim</button>
					</div>
				</div>
			{/each}
		</div>
	</div>
{/if}

<style>
	.my-locks {
		margin-bottom: var(--nox-space-xl);
	}

	.my-locks h3 {
		font-size: var(--nox-text-base);
		margin-bottom: var(--nox-space-md);
	}

	.locks-list {
		display: flex;
		flex-direction: column;
		gap: var(--nox-space-sm);
	}

	.lock-card {
		display: flex;
		align-items: center;
		gap: var(--nox-space-lg);
		background: var(--nox-bg-tertiary);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-md);
	}

	.lock-info {
		flex: 1;
	}

	.lock-amount {
		font-size: var(--nox-text-base);
		font-weight: var(--nox-font-semibold);
	}

	.lock-tier {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
	}

	.lock-rewards {
		text-align: right;
	}

	.rewards-label {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
	}

	.rewards-value {
		font-size: var(--nox-text-sm);
		font-weight: var(--nox-font-semibold);
		color: var(--nox-accent-primary);
	}

	.lock-actions {
		display: flex;
		gap: var(--nox-space-sm);
	}

	.btn {
		padding: var(--nox-space-sm) var(--nox-space-lg);
		border-radius: var(--nox-radius-md);
		font-weight: var(--nox-font-medium);
		font-size: var(--nox-text-sm);
		cursor: pointer;
		transition: all var(--nox-transition-fast);
		border: none;
		background: var(--nox-accent-gradient);
		color: var(--nox-bg-primary);
	}

	.btn.secondary {
		background: var(--nox-bg-tertiary);
		border: 1px solid var(--nox-border);
		color: var(--nox-text-primary);
	}

	.btn.small {
		padding: var(--nox-space-xs) var(--nox-space-sm);
		font-size: var(--nox-text-xs);
	}

	.locked-badge {
		padding: var(--nox-space-xs) var(--nox-space-sm);
		background: var(--nox-warning-bg);
		color: var(--nox-warning);
		border-radius: var(--nox-radius-sm);
		font-size: var(--nox-text-xs);
	}
</style>
