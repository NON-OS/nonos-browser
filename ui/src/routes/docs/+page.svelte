<script lang="ts">
	import { DocNav, EconomyDoc, ArchitectureDoc, StakingDoc, NymDoc } from '$lib/docs';
	let activeDoc = 'economy';
	function handleSelect(e: CustomEvent<{ id: string }>) { activeDoc = e.detail.id; }
</script>

<div class="docs-page">
	<header class="docs-header">
		<h1>Documentation</h1>
	</header>

	<div class="docs-layout">
		<aside class="docs-sidebar">
			<DocNav {activeDoc} on:select={handleSelect} />
		</aside>

		<main class="docs-main">
			{#if activeDoc === 'economy'}
				<EconomyDoc />
			{:else if activeDoc === 'architecture'}
				<ArchitectureDoc />
			{:else if activeDoc === 'staking'}
				<StakingDoc />
			{:else if activeDoc === 'nym'}
				<NymDoc />
			{/if}
		</main>
	</div>
</div>

<style>
	.docs-page { max-width: 1100px; margin: 0 auto; }
	.docs-header { margin-bottom: 2rem; padding-bottom: 1.5rem; border-bottom: 1px solid #1a1a1a; }
	.docs-header h1 { font-size: 1.5rem; font-weight: 600; }
	.docs-layout { display: grid; grid-template-columns: 200px 1fr; gap: 3rem; }
	.docs-sidebar { position: sticky; top: 2rem; align-self: start; }
	.docs-main { min-height: 60vh; }

	@media (max-width: 800px) {
		.docs-layout { grid-template-columns: 1fr; gap: 1.5rem; }
		.docs-sidebar { position: static; }
	}
</style>
