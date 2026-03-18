<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();
	export let activeDoc = '';

	const docs = [
		{ id: 'economy', title: 'Privacy Economy' },
		{ id: 'architecture', title: 'Architecture' },
		{ id: 'staking', title: 'Staking' },
		{ id: 'nym', title: 'Nym Mixnet' }
	];
</script>

<nav class="doc-nav">
	{#each docs as doc}
		<button class="nav-item" class:active={activeDoc === doc.id} on:click={() => dispatch('select', { id: doc.id })}>
			{doc.title}
		</button>
	{/each}
</nav>

<style>
	.doc-nav { display: flex; flex-direction: column; gap: 0.25rem; }
	.nav-item { padding: 0.75rem 1rem; background: none; border: none; border-radius: 6px; text-align: left; color: #9ca3af; font-size: 0.875rem; cursor: pointer; transition: all 0.15s; }
	.nav-item:hover { color: #e6edf3; background: #0a0a0a; }
	.nav-item.active { color: #66FFFF; background: rgba(102, 255, 255, 0.1); }

	@media (max-width: 800px) {
		.doc-nav { flex-direction: row; overflow-x: auto; gap: 0.5rem; padding-bottom: 0.5rem; }
		.nav-item { white-space: nowrap; }
	}
</style>
