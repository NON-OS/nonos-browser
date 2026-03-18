<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import QRCodeDisplay from './QRCodeDisplay.svelte';
	import AddressDisplay from './AddressDisplay.svelte';
	import QRCode from 'qrcode';

	const dispatch = createEventDispatcher();
	export let show = false;
	export let address = '';
	let qrCodeDataUrl = '';

	async function generateQRCode(text: string): Promise<void> {
		try {
			qrCodeDataUrl = await QRCode.toDataURL(text, { width: 200, margin: 2, color: { dark: '#00ff88', light: '#0a0f0a' } });
		} catch (e) { console.error('QR error:', e); qrCodeDataUrl = ''; }
	}

	$: if (show && address) { generateQRCode(address); }
	function close() { qrCodeDataUrl = ''; dispatch('close'); }
</script>

{#if show}
	<div class="modal-overlay" on:click={close} on:keydown={(e) => e.key === 'Escape' && close()} role="button" tabindex="0">
		<div class="modal" on:click|stopPropagation role="dialog" aria-modal="true">
			<div class="modal-header">
				<h2>Receive Tokens</h2>
				<button class="close-btn" on:click={close}><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6L6 18M6 6l12 12"/></svg></button>
			</div>
			<div class="modal-body">
				<p class="receive-info">Send ETH or NOX tokens to your wallet address below:</p>
				<QRCodeDisplay {qrCodeDataUrl} />
				<AddressDisplay {address} />
				<div class="receive-tokens">
					<div class="receive-token eth"><svg viewBox="0 0 24 24" fill="none"><path d="M12 2L4 12l8 5 8-5-8-10z" fill="currentColor" opacity="0.6"/><path d="M12 22l8-10-8 5-8-5 8 10z" fill="currentColor"/></svg><span>ETH</span></div>
					<div class="receive-token nox"><svg viewBox="0 0 24 24" fill="none"><path d="M12 2L22 12L12 22L2 12L12 2Z" fill="currentColor"/></svg><span>NOX</span></div>
				</div>
				<p class="network-note">Network: Ethereum Mainnet</p>
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
	.modal-body { padding: var(--nox-space-lg); text-align: center; }
	.receive-info { color: var(--nox-text-secondary); font-size: var(--nox-text-sm); margin-bottom: var(--nox-space-lg); }
	.receive-tokens { display: flex; justify-content: center; gap: var(--nox-space-lg); margin-bottom: var(--nox-space-md); }
	.receive-token { display: flex; align-items: center; gap: var(--nox-space-xs); font-size: var(--nox-text-sm); color: var(--nox-text-secondary); }
	.receive-token svg { width: 20px; height: 20px; }
	.receive-token.eth svg { color: #627eea; }
	.receive-token.nox svg { color: var(--nox-accent-primary); }
	.network-note { font-size: var(--nox-text-xs); color: var(--nox-text-muted); }
</style>
