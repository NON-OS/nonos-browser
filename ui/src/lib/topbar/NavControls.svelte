<script lang="ts">
	import { browserStore } from '$lib/stores/browser';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	function handleBack() {
		const browser = (window as any).__nonosBrowser;
		if (browser?.goBack) browser.goBack();
	}

	function handleForward() {
		const browser = (window as any).__nonosBrowser;
		if (browser?.goForward) browser.goForward();
	}

	function handleReload() {
		const browser = (window as any).__nonosBrowser;
		if (browser?.reload) browser.reload();
	}

	function handleHome() {
		dispatch('home');
		const browser = (window as any).__nonosBrowser;
		if (browser?.goHome) {
			browser.goHome();
		} else {
			browserStore.clearUrl();
		}
	}
</script>

<div class="nav-controls">
	<button class="nav-btn" on:click={handleBack} title="Go Back" aria-label="Go Back">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<path d="M19 12H5M12 19l-7-7 7-7" />
		</svg>
	</button>
	<button class="nav-btn" on:click={handleForward} title="Go Forward" aria-label="Go Forward">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<path d="M5 12h14M12 5l7 7-7 7" />
		</svg>
	</button>
	<button class="nav-btn" on:click={handleReload} title="Reload Page" aria-label="Reload" class:loading={$browserStore.isLoading}>
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<path d="M23 4v6h-6M1 20v-6h6" />
			<path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
		</svg>
	</button>
	<button class="nav-btn" on:click={handleHome} title="Home" aria-label="Home">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
			<path d="M9 22V12h6v10"/>
		</svg>
	</button>
</div>

<style>
	.nav-controls {
		display: flex;
		gap: var(--nox-space-xs);
		-webkit-app-region: no-drag;
	}

	.nav-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: var(--nox-radius-md);
		color: var(--nox-text-muted);
		background: transparent;
		transition: all var(--nox-transition-fast);
	}

	.nav-btn svg {
		width: 16px;
		height: 16px;
	}

	.nav-btn:hover {
		background: var(--nox-bg-hover);
		color: var(--nox-text-primary);
	}

	.nav-btn:active {
		background: var(--nox-bg-active);
	}

	.nav-btn.loading svg {
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		100% { transform: rotate(360deg); }
	}
</style>
