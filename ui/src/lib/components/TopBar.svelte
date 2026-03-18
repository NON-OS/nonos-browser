<script lang="ts">
	import { browserStore } from '$lib/stores/browser';
	import { NavControls, UrlBar, IdentityButton, WalletPill } from '$lib/topbar';

	export let networkStatus: { connected: boolean; bootstrap_progress: number; circuits: number };
	export let walletAddress: string;

	let url = '';

	$: if ($browserStore.currentUrl && $browserStore.currentUrl !== url) {
		url = $browserStore.currentUrl;
	}

	function handleHome() {
		url = '';
	}
</script>

<header class="topbar">
	<NavControls on:home={handleHome} />

	<UrlBar connected={networkStatus.connected} bind:url />

	<div class="toolbar-actions">
		<IdentityButton circuits={networkStatus.circuits} />
		<WalletPill {walletAddress} />
	</div>
</header>

<style>
	.topbar {
		display: flex;
		align-items: center;
		gap: var(--nox-space-md);
		padding: var(--nox-space-sm) var(--nox-space-lg);
		background: linear-gradient(180deg, rgba(18, 18, 31, 0.98) 0%, rgba(13, 13, 26, 0.98) 100%);
		border-bottom: 1px solid rgba(102, 255, 255, 0.1);
		-webkit-app-region: drag;
		height: 56px;
		position: relative;
	}

	.topbar::before {
		content: '';
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 1px;
		background: linear-gradient(90deg, transparent, rgba(102, 255, 255, 0.3), transparent);
	}

	.toolbar-actions {
		display: flex;
		align-items: center;
		gap: var(--nox-space-sm);
		-webkit-app-region: no-drag;
	}
</style>
