<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import TokenTabs from './TokenTabs.svelte';
	import SendForm from './SendForm.svelte';

	const dispatch = createEventDispatcher();

	export let show = false;
	export let balances = { eth: '0', nox: '0' };
	export let sendLoading = false;
	export let sendError = '';
	export let sendSuccess = '';

	let sendToken: 'eth' | 'nox' = 'eth';
	let sendTo = '';
	let sendAmount = '';

	function close() { sendTo = ''; sendAmount = ''; dispatch('close'); }
	function handleSend(e: CustomEvent) { dispatch('send', e.detail); }
</script>

{#if show}
	<div class="modal-overlay" on:click={close} on:keydown={(e) => e.key === 'Escape' && close()} role="button" tabindex="0">
		<div class="modal" on:click|stopPropagation role="dialog" aria-modal="true">
			<div class="modal-header">
				<h2>Send {sendToken.toUpperCase()}</h2>
				<button class="close-btn" on:click={close}>
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6L6 18M6 6l12 12"/></svg>
				</button>
			</div>
			<div class="modal-body">
				<TokenTabs bind:sendToken {balances} />
				<SendForm bind:sendTo bind:sendAmount {sendToken} {sendLoading} {sendError} {sendSuccess} {balances} on:send={handleSend} />
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-overlay { position: fixed; inset: 0; background: rgba(0, 0, 0, 0.8); display: flex; align-items: center; justify-content: center; z-index: 1000; backdrop-filter: blur(4px); }
	.modal { background: var(--nox-bg-secondary); border: 1px solid var(--nox-border); border-radius: var(--nox-radius-xl); width: 100%; max-width: 440px; max-height: 90vh; overflow: auto; }
	.modal-header { display: flex; justify-content: space-between; align-items: center; padding: var(--nox-space-lg); border-bottom: 1px solid var(--nox-border); }
	.modal-header h2 { font-size: var(--nox-text-lg); font-weight: var(--nox-font-semibold); }
	.close-btn { width: 32px; height: 32px; display: flex; align-items: center; justify-content: center; border-radius: var(--nox-radius-md); color: var(--nox-text-muted); transition: all var(--nox-transition-fast); }
	.close-btn:hover { background: var(--nox-bg-hover); color: var(--nox-text-primary); }
	.close-btn svg { width: 18px; height: 18px; }
	.modal-body { padding: var(--nox-space-lg); }
</style>
