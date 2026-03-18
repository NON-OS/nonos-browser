<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	function send() {
		dispatch('send');
	}

	function receive() {
		dispatch('receive');
	}

	function stake() {
		dispatch('stake');
	}

	function lock() {
		dispatch('lock');
	}
</script>

<div class="actions">
	<button class="btn action-btn send" on:click={send}>
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<line x1="22" y1="2" x2="11" y2="13"/>
			<polygon points="22 2 15 22 11 13 2 9 22 2"/>
		</svg>
		Send
	</button>
	<button class="btn action-btn receive" on:click={receive}>
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/>
			<path d="M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/>
		</svg>
		Receive
	</button>
	<button class="btn action-btn stake" on:click={stake}>
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<path d="M12 2v20M2 12h20"/>
			<circle cx="12" cy="12" r="10"/>
		</svg>
		Stake
	</button>
	<button class="btn action-btn lock" on:click={lock}>
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
			<path d="M7 11V7a5 5 0 0 1 10 0v4"/>
		</svg>
		Lock
	</button>
</div>

<style>
	.actions {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: var(--nox-space-md);
	}

	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: var(--nox-space-sm);
		padding: var(--nox-space-sm) var(--nox-space-lg);
		border-radius: var(--nox-radius-md);
		font-weight: var(--nox-font-medium);
		transition: all var(--nox-transition-fast);
		font-size: var(--nox-text-sm);
	}

	.action-btn {
		flex-direction: column;
		gap: var(--nox-space-xs);
		padding: var(--nox-space-lg);
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-xl);
		font-size: var(--nox-text-sm);
		color: var(--nox-text-secondary);
		transition: all var(--nox-transition-fast);
	}

	.action-btn svg {
		width: 24px;
		height: 24px;
	}

	.action-btn:hover {
		border-color: var(--nox-accent-primary);
		color: var(--nox-text-primary);
	}

	.action-btn:hover svg {
		color: var(--nox-accent-primary);
	}

	.action-btn.send:hover {
		border-color: var(--nox-accent-primary);
	}

	.action-btn.receive:hover {
		border-color: var(--nox-success);
	}

	.action-btn.receive:hover svg {
		color: var(--nox-success);
	}

	.action-btn.stake:hover {
		border-color: var(--nox-accent-primary);
	}

	.action-btn.stake:hover svg {
		color: var(--nox-accent-primary);
	}

	.action-btn.lock:hover {
		border-color: var(--nox-warning);
	}

	.action-btn.lock:hover svg {
		color: var(--nox-warning);
	}
</style>
