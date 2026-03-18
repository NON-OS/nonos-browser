<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	export let sendTo = '';
	export let sendAmount = '';
	export let sendToken: 'eth' | 'nox' = 'eth';
	export let sendLoading = false;
	export let sendError = '';
	export let sendSuccess = '';
	export let balances = { eth: '0', nox: '0' };

	function setMaxAmount() {
		if (sendToken === 'eth') {
			const maxEth = Math.max(0, parseFloat(balances.eth) - 0.005);
			sendAmount = maxEth.toFixed(6);
		} else {
			sendAmount = balances.nox;
		}
	}

	function handleSend() {
		dispatch('send', { token: sendToken, to: sendTo, amount: sendAmount });
	}
</script>

<div class="form-group">
	<label for="send-to">Recipient Address</label>
	<input id="send-to" type="text" bind:value={sendTo} placeholder="0x..." class="input" disabled={sendLoading} />
</div>

<div class="form-group">
	<label for="send-amount">Amount ({sendToken.toUpperCase()})</label>
	<div class="amount-input-group">
		<input id="send-amount" type="number" step="0.000001" bind:value={sendAmount} placeholder="0.0" class="input" disabled={sendLoading} />
		<button class="max-btn" on:click={setMaxAmount} disabled={sendLoading}>MAX</button>
	</div>
</div>

{#if sendError}<div class="modal-error">{sendError}</div>{/if}
{#if sendSuccess}<div class="modal-success">{sendSuccess}</div>{/if}

<button class="btn primary full-width send-btn" on:click={handleSend} disabled={sendLoading || !sendTo || !sendAmount}>
	{#if sendLoading}<svg class="spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>Sending...
	{:else}Send {sendAmount || '0'} {sendToken.toUpperCase()}{/if}
</button>

<style>
	.form-group { margin-bottom: var(--nox-space-md); }
	.form-group label { display: block; font-size: var(--nox-text-sm); color: var(--nox-text-secondary); margin-bottom: var(--nox-space-xs); }
	.amount-input-group { display: flex; gap: var(--nox-space-sm); }
	.amount-input-group :global(.input) { flex: 1; }
	.max-btn { padding: var(--nox-space-sm) var(--nox-space-md); background: var(--nox-bg-tertiary); border: 1px solid var(--nox-border); border-radius: var(--nox-radius-md); font-size: var(--nox-text-xs); font-weight: var(--nox-font-semibold); color: var(--nox-accent-primary); transition: all var(--nox-transition-fast); }
	.max-btn:hover:not(:disabled) { background: var(--nox-accent-glow); }
	.modal-error { background: var(--nox-error-bg); border: 1px solid var(--nox-error); color: var(--nox-error); padding: var(--nox-space-sm) var(--nox-space-md); border-radius: var(--nox-radius-md); font-size: var(--nox-text-sm); margin-bottom: var(--nox-space-md); }
	.modal-success { background: var(--nox-success-bg); border: 1px solid var(--nox-success); color: var(--nox-success); padding: var(--nox-space-sm) var(--nox-space-md); border-radius: var(--nox-radius-md); font-size: var(--nox-text-sm); margin-bottom: var(--nox-space-md); }
	.btn { display: inline-flex; align-items: center; justify-content: center; gap: var(--nox-space-sm); padding: var(--nox-space-sm) var(--nox-space-lg); border-radius: var(--nox-radius-md); font-weight: var(--nox-font-medium); transition: all var(--nox-transition-fast); font-size: var(--nox-text-sm); }
	.btn.primary { background: var(--nox-accent-gradient); color: var(--nox-bg-primary); }
	.btn.primary:hover:not(:disabled) { box-shadow: var(--nox-shadow-glow); }
	.btn:disabled { opacity: 0.5; cursor: not-allowed; }
	.btn.full-width { width: 100%; }
	.send-btn { margin-top: var(--nox-space-md); }
	.spinner { animation: spin 1s linear infinite; width: 18px; height: 18px; }
	@keyframes spin { 100% { transform: rotate(360deg); } }
</style>
