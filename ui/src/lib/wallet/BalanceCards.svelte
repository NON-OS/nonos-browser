<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import BalanceCard from './BalanceCard.svelte';

	const dispatch = createEventDispatcher();
	export let balances = { eth: '0', nox: '0', sepolia_eth: '0', sepolia_nox: '0' };
	export let isRefreshing = false;
</script>

<div class="balance-section">
	<div class="balance-header">
		<span class="balance-title">Mainnet Balances</span>
		<button class="refresh-btn" on:click={() => dispatch('refresh')} disabled={isRefreshing} title="Refresh balances">
			<svg class:spinning={isRefreshing} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M23 4v6h-6M1 20v-6h6"/><path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/></svg>
		</button>
	</div>
	<div class="balance-cards">
		<BalanceCard type="eth" label="Ethereum" value={balances.eth} />
		<BalanceCard type="nox" label="NOX Token" value={balances.nox} />
	</div>
</div>

<div class="balance-section sepolia-section">
	<div class="balance-header">
		<span class="balance-title"><span class="network-badge sepolia">Sepolia Testnet</span>Staking Balances</span>
	</div>
	<div class="balance-cards">
		<BalanceCard type="eth" label="Sepolia ETH (for gas)" value={balances.sepolia_eth} sepolia />
		<BalanceCard type="nox" label="Sepolia NOX (for staking)" value={balances.sepolia_nox} sepolia />
	</div>
</div>

<style>
	.balance-section { margin-bottom: var(--nox-space-md); }
	.balance-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--nox-space-md); }
	.balance-title { font-size: var(--nox-text-sm); color: var(--nox-text-muted); text-transform: uppercase; letter-spacing: 0.08em; display: flex; align-items: center; }
	.refresh-btn { width: 32px; height: 32px; display: flex; align-items: center; justify-content: center; border-radius: var(--nox-radius-md); color: var(--nox-text-muted); transition: all var(--nox-transition-fast); }
	.refresh-btn:hover:not(:disabled) { background: var(--nox-bg-hover); color: var(--nox-accent-primary); }
	.refresh-btn:disabled { opacity: 0.5; }
	.refresh-btn svg { width: 18px; height: 18px; }
	.refresh-btn svg.spinning { animation: spin 1s linear infinite; }
	@keyframes spin { 100% { transform: rotate(360deg); } }
	.network-badge { display: inline-flex; align-items: center; padding: 2px 8px; border-radius: var(--nox-radius-sm); font-size: var(--nox-text-xs); font-weight: var(--nox-font-semibold); text-transform: uppercase; letter-spacing: 0.05em; margin-right: var(--nox-space-sm); }
	.network-badge.sepolia { background: rgba(245, 158, 11, 0.15); color: #f59e0b; }
	.sepolia-section { border: 1px solid rgba(245, 158, 11, 0.3); border-radius: var(--nox-radius-xl); padding: var(--nox-space-md); background: rgba(245, 158, 11, 0.05); }
	.balance-cards { display: grid; grid-template-columns: repeat(2, 1fr); gap: var(--nox-space-md); }
</style>
