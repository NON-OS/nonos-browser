<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let isLoading = false;

	let stakeAmount = '';

	function handleStake() {
		if (!stakeAmount) return;
		dispatch('stake', { amount: stakeAmount });
		stakeAmount = '';
	}

	function handleClaim() {
		dispatch('claim');
	}
</script>

<div class="stake-form">
	<h2>Stake NOX</h2>
	<div class="form-row">
		<input
			type="text"
			bind:value={stakeAmount}
			placeholder="Amount to stake..."
			class="input"
		/>
		<button class="btn primary" on:click={handleStake} disabled={isLoading || !stakeAmount}>
			Stake
		</button>
	</div>
	<button class="btn secondary" on:click={handleClaim} disabled={isLoading}>
		Claim Pending Rewards
	</button>
</div>

<style>
	.stake-form {
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-lg);
		margin-bottom: var(--nox-space-xl);
	}

	h2 {
		font-size: 18px;
		font-weight: 600;
		margin-bottom: var(--nox-space-md);
	}

	.form-row {
		display: flex;
		gap: var(--nox-space-md);
		margin-bottom: var(--nox-space-md);
	}

	.form-row .input {
		flex: 1;
	}

	.btn {
		padding: var(--nox-space-sm) var(--nox-space-lg);
		border-radius: var(--nox-radius-md);
		font-weight: 500;
		transition: all var(--nox-transition-fast);
	}

	.btn.primary {
		background: var(--nox-accent-gradient);
		color: white;
	}

	.btn.secondary {
		background: var(--nox-bg-tertiary);
		border: 1px solid var(--nox-border);
		width: 100%;
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
