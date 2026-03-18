<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { fade, scale } from 'svelte/transition';

	export let open = false;
	export let title = '';
	export let size: 'sm' | 'md' | 'lg' = 'md';

	const dispatch = createEventDispatcher();

	function close() {
		dispatch('close');
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') close();
	}
</script>

<svelte:window on:keydown={handleKeydown} />

{#if open}
	<div class="overlay" transition:fade={{ duration: 150 }} on:click={close} on:keydown role="button" tabindex="-1">
		<div class="modal modal-{size}" transition:scale={{ duration: 150, start: 0.95 }} on:click|stopPropagation on:keydown role="dialog">
			{#if title}
				<header class="header">
					<h2>{title}</h2>
					<button class="close" on:click={close}>×</button>
				</header>
			{/if}
			<div class="content">
				<slot />
			</div>
			{#if $$slots.footer}
				<footer class="footer">
					<slot name="footer" />
				</footer>
			{/if}
		</div>
	</div>
{/if}

<style>
	.overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 300;
	}
	.modal {
		background: #12121c;
		border: 1px solid rgba(102, 255, 255, 0.2);
		border-radius: 12px;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}
	.modal-sm { width: 360px; }
	.modal-md { width: 480px; }
	.modal-lg { width: 640px; }
	.header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 20px;
		border-bottom: 1px solid #2a2a40;
	}
	.header h2 { font-size: 18px; font-weight: 600; }
	.close {
		width: 32px;
		height: 32px;
		font-size: 24px;
		color: #6b6b80;
		background: none;
		border: none;
		cursor: pointer;
	}
	.close:hover { color: #66FFFF; }
	.content { padding: 20px; overflow-y: auto; }
	.footer {
		padding: 16px 20px;
		border-top: 1px solid #2a2a40;
		display: flex;
		gap: 12px;
		justify-content: flex-end;
	}
</style>
