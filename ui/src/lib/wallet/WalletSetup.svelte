<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import CreateWalletCard from './CreateWalletCard.svelte';
	import ImportWalletCard from './ImportWalletCard.svelte';

	const dispatch = createEventDispatcher();
	export let isLoading = false;
	export let error = '';

	function handleCreate(e: CustomEvent) { dispatch('create', e.detail); }
	function handleImport(e: CustomEvent) { dispatch('import', e.detail); }
</script>

<div class="wallet-setup">
	<CreateWalletCard {isLoading} on:create={handleCreate} />
	<ImportWalletCard {isLoading} on:import={handleImport} />
</div>

<style>
	.wallet-setup { display: grid; grid-template-columns: repeat(2, 1fr); gap: var(--nox-space-lg); }
	@media (max-width: 700px) { .wallet-setup { grid-template-columns: 1fr; } }
</style>
