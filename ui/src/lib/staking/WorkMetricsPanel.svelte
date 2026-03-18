<script lang="ts">
	import type { WorkMetrics, WorkCategory } from './types';
	import EpochProgress from './EpochProgress.svelte';
	import WorkScoreCard from './WorkScoreCard.svelte';
	import WorkCategoriesGrid from './WorkCategoriesGrid.svelte';
	import WorkDetailsGrid from './WorkDetailsGrid.svelte';

	export let workMetrics: WorkMetrics | null = null;
	export let workCategories: WorkCategory[] = [];
	export let estimatedReward = '0';
	export let epochProgress = 0;
	export let epochTimeRemaining = '';
</script>

<div class="section">
	<div class="section-header">
		<h2>Work Metrics</h2>
		<p>Your node's privacy work score determines 70% of your epoch rewards</p>
	</div>

	{#if workMetrics}
		<EpochProgress
			currentEpoch={workMetrics.epoch.current_epoch}
			{epochProgress}
			{epochTimeRemaining}
		/>

		<WorkScoreCard
			workScore={workMetrics.total_work_score}
			{estimatedReward}
		/>

		<WorkCategoriesGrid {workCategories} />

		<WorkDetailsGrid {workMetrics} />
	{:else}
		<div class="loading-state">
			<p>Loading work metrics...</p>
		</div>
	{/if}
</div>

<style>
	.section {
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-xl);
		padding: var(--nox-space-xl);
	}

	.section-header {
		margin-bottom: var(--nox-space-lg);
	}

	.section-header h2 {
		font-size: var(--nox-text-lg);
		font-weight: var(--nox-font-semibold);
		margin-bottom: var(--nox-space-xs);
	}

	.section-header p {
		color: var(--nox-text-muted);
		font-size: var(--nox-text-sm);
	}

	.loading-state {
		text-align: center;
		padding: var(--nox-space-xl);
		color: var(--nox-text-muted);
	}
</style>
