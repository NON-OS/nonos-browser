<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let variant: 'info' | 'success' | 'warning' | 'error' = 'info';
	export let dismissible = false;

	const dispatch = createEventDispatcher();
</script>

<div class="banner banner-{variant}">
	<div class="content">
		<slot />
	</div>
	{#if dismissible}
		<button class="dismiss" on:click={() => dispatch('dismiss')}>×</button>
	{/if}
</div>

<style>
	.banner {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		border-radius: 8px;
		font-size: 14px;
	}
	.content { flex: 1; }
	.banner-info {
		background: rgba(102, 255, 255, 0.1);
		border: 1px solid rgba(102, 255, 255, 0.3);
		color: #66FFFF;
	}
	.banner-success {
		background: rgba(0, 255, 136, 0.1);
		border: 1px solid rgba(0, 255, 136, 0.3);
		color: #00ff88;
	}
	.banner-warning {
		background: rgba(255, 170, 0, 0.1);
		border: 1px solid rgba(255, 170, 0, 0.3);
		color: #ffaa00;
	}
	.banner-error {
		background: rgba(255, 68, 102, 0.1);
		border: 1px solid rgba(255, 68, 102, 0.3);
		color: #ff4466;
	}
	.dismiss {
		width: 24px;
		height: 24px;
		font-size: 18px;
		background: none;
		border: none;
		color: inherit;
		opacity: 0.7;
		cursor: pointer;
	}
	.dismiss:hover { opacity: 1; }
</style>
