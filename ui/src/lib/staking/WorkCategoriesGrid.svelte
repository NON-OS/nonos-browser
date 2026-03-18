<script lang="ts">
	import type { WorkCategory } from './types';
	import { formatNumber } from './types';

	export let workCategories: WorkCategory[];
</script>

<div class="work-categories">
	{#each workCategories as cat}
		<div class="category-card">
			<div class="category-header">
				<span class="category-name">{cat.name}</span>
				<span class="category-weight">{cat.weight}%</span>
			</div>
			<div class="category-bar">
				<div class="category-fill" style="width: {cat.score}%"></div>
			</div>
			<div class="category-stats">
				<span>Score: {cat.score.toFixed(1)}</span>
				<span>Raw: {formatNumber(cat.raw_value)}</span>
			</div>
		</div>
	{/each}
</div>

<style>
	.work-categories {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: var(--nox-space-md);
		margin-bottom: var(--nox-space-xl);
	}

	.category-card {
		background: var(--nox-bg-tertiary);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-md);
	}

	.category-header {
		display: flex;
		justify-content: space-between;
		margin-bottom: var(--nox-space-sm);
	}

	.category-name {
		font-size: var(--nox-text-xs);
		font-weight: var(--nox-font-medium);
	}

	.category-weight {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
	}

	.category-bar {
		height: 6px;
		background: var(--nox-bg-primary);
		border-radius: var(--nox-radius-full);
		overflow: hidden;
		margin-bottom: var(--nox-space-sm);
	}

	.category-fill {
		height: 100%;
		background: var(--nox-accent-primary);
		border-radius: var(--nox-radius-full);
	}

	.category-stats {
		display: flex;
		justify-content: space-between;
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
	}

	@media (max-width: 800px) {
		.work-categories {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 500px) {
		.work-categories {
			grid-template-columns: 1fr;
		}
	}
</style>
