<script lang="ts">
	import { browserStore } from '$lib/stores/browser';
	import SecurityBadge from './SecurityBadge.svelte';

	export let connected: boolean;
	export let url: string;

	function navigate() {
		if (!url) return;

		let targetUrl: string;
		if (url.startsWith('http://') || url.startsWith('https://')) {
			targetUrl = url;
		} else if (url.includes('.') && !url.includes(' ')) {
			targetUrl = 'https://' + url;
		} else {
			targetUrl = `https://html.duckduckgo.com/html/?q=${encodeURIComponent(url)}`;
		}

		browserStore.setCurrentUrl(targetUrl);
	}
</script>

<form class="url-bar" on:submit|preventDefault={navigate}>
	<SecurityBadge {connected} />
	<input
		type="text"
		bind:value={url}
		placeholder="Search or enter URL..."
		class="url-input"
		autocomplete="off"
		spellcheck="false"
	/>
	<button type="submit" class="go-btn" disabled={!url} aria-label="Navigate">
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<path d="M5 12h14M12 5l7 7-7 7" />
		</svg>
	</button>
</form>

<style>
	.url-bar {
		flex: 1;
		display: flex;
		align-items: center;
		gap: var(--nox-space-sm);
		background: var(--nox-bg-tertiary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-xs) var(--nox-space-xs) var(--nox-space-xs) var(--nox-space-sm);
		-webkit-app-region: no-drag;
		transition: all var(--nox-transition-fast);
	}

	.url-bar:focus-within {
		border-color: var(--nox-accent-primary);
		box-shadow: 0 0 0 3px var(--nox-accent-glow);
	}

	.url-input {
		flex: 1;
		background: transparent;
		border: none;
		padding: var(--nox-space-xs) var(--nox-space-sm);
		font-size: var(--nox-text-sm);
		color: var(--nox-text-primary);
		min-width: 0;
	}

	.url-input:focus {
		outline: none;
	}

	.url-input::placeholder {
		color: var(--nox-text-muted);
	}

	.go-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--nox-accent-gradient);
		border-radius: var(--nox-radius-md);
		color: var(--nox-bg-primary);
		transition: all var(--nox-transition-fast);
	}

	.go-btn svg {
		width: 16px;
		height: 16px;
	}

	.go-btn:hover:not(:disabled) {
		box-shadow: var(--nox-shadow-glow);
		transform: translateX(2px);
	}

	.go-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}
</style>
