<script lang="ts">
	export let type: 'eth' | 'nox' = 'eth';
	export let label = '';
	export let value = '0';
	export let sepolia = false;
</script>

<div class="balance-card {type}" class:sepolia>
	<div class="balance-icon">
		{#if type === 'eth'}
			<svg viewBox="0 0 24 24" fill="none"><path d="M12 2L4 12l8 5 8-5-8-10z" fill="currentColor" opacity="0.6"/><path d="M12 22l8-10-8 5-8-5 8 10z" fill="currentColor"/></svg>
		{:else}
			<svg viewBox="0 0 24 24" fill="none"><path d="M12 2L22 12L12 22L2 12L12 2Z" fill="currentColor"/></svg>
		{/if}
	</div>
	<div class="balance-info">
		<div class="balance-label">{label}</div>
		<div class="balance-value">{value} <span class="unit">{type === 'eth' ? 'ETH' : 'NOX'}</span></div>
	</div>
</div>

<style>
	.balance-card { display: flex; align-items: center; gap: var(--nox-space-lg); background: var(--nox-bg-secondary); border: 1px solid var(--nox-border); border-radius: var(--nox-radius-xl); padding: var(--nox-space-lg); transition: all var(--nox-transition-fast); }
	.balance-card:hover { border-color: var(--nox-border-light); }
	.balance-card.sepolia .balance-icon { opacity: 0.8; }
	.balance-icon { width: 56px; height: 56px; display: flex; align-items: center; justify-content: center; border-radius: var(--nox-radius-full); }
	.balance-icon svg { width: 28px; height: 28px; }
	.balance-card.eth .balance-icon { background: linear-gradient(135deg, #627eea 0%, #3c5fc9 100%); color: white; }
	.balance-card.nox .balance-icon { background: var(--nox-accent-gradient); color: var(--nox-bg-primary); }
	.balance-label { font-size: var(--nox-text-xs); color: var(--nox-text-muted); margin-bottom: var(--nox-space-2xs); }
	.balance-value { font-size: var(--nox-text-xl); font-weight: var(--nox-font-semibold); font-family: var(--nox-font-mono); }
	.balance-value .unit { font-size: var(--nox-text-sm); font-weight: var(--nox-font-regular); color: var(--nox-text-muted); }
</style>
