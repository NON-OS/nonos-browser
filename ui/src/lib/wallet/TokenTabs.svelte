<script lang="ts">
	export let sendToken: 'eth' | 'nox' = 'eth';
	export let balances = { eth: '0', nox: '0' };
</script>

<div class="token-tabs">
	<button class="token-tab" class:active={sendToken === 'eth'} on:click={() => sendToken = 'eth'}>
		<span class="token-icon eth">ETH</span>
		<span class="token-balance">{balances.eth}</span>
	</button>
	<button class="token-tab" class:active={sendToken === 'nox'} on:click={() => sendToken = 'nox'}>
		<span class="token-icon nox">NOX</span>
		<span class="token-balance">{balances.nox}</span>
	</button>
</div>

<style>
	.token-tabs { display: grid; grid-template-columns: 1fr 1fr; gap: var(--nox-space-sm); margin-bottom: var(--nox-space-lg); }
	.token-tab { display: flex; flex-direction: column; align-items: center; gap: var(--nox-space-xs); padding: var(--nox-space-md); background: var(--nox-bg-tertiary); border: 2px solid transparent; border-radius: var(--nox-radius-lg); transition: all var(--nox-transition-fast); }
	.token-tab:hover { border-color: var(--nox-border-light); }
	.token-tab.active { border-color: var(--nox-accent-primary); background: var(--nox-accent-glow); }
	.token-icon { font-size: var(--nox-text-lg); font-weight: var(--nox-font-semibold); }
	.token-icon.eth { color: #627eea; }
	.token-icon.nox { color: var(--nox-accent-primary); }
	.token-balance { font-size: var(--nox-text-xs); color: var(--nox-text-muted); font-family: var(--nox-font-mono); }
</style>
