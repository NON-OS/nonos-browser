<script lang="ts">
	import { onMount } from 'svelte';
	import { ConnectionCard, CircuitsList, NymInfo, type NetworkStatus, type Circuit } from '$lib/network';

	let networkStatus: NetworkStatus = { connected: false, bootstrap_progress: 0, circuits: 0, status: 'Disconnected', error: null };
	let circuits: Circuit[] = [];
	let isConnecting = false;

	onMount(() => {
		updateStatus();
		const interval = setInterval(updateStatus, 3000);

		if (window.nonos?.onNetworkStatus) {
			window.nonos.onNetworkStatus((status: NetworkStatus) => {
				networkStatus = status;
				isConnecting = status.status === 'Connecting' || status.status === 'Bootstrapping';
			});
		}

		return () => clearInterval(interval);
	});

	async function updateStatus() {
		if (!window.nonos) return;
		try {
			const status = await window.nonos.network.getStatus();
			networkStatus = status;
			isConnecting = status.status === 'Connecting' || status.status === 'Bootstrapping';
		} catch (e) {
			console.error('Failed to get status:', e);
		}
	}

	async function handleConnect() {
		if (!window.nonos) return;
		isConnecting = true;
		try {
			await window.nonos.network.connect();
			await updateStatus();
		} catch (e) {
			console.error('Failed to connect:', e);
		} finally {
			isConnecting = false;
		}
	}

	async function handleDisconnect() {
		if (!window.nonos) return;
		try {
			await window.nonos.network.disconnect();
			await updateStatus();
		} catch (e) {
			console.error('Failed to disconnect:', e);
		}
	}

	async function handleNewIdentity() {
		if (!window.nonos) return;
		try {
			await window.nonos.network.newIdentity();
			await updateStatus();
			circuits = [];
		} catch (e) {
			console.error('Failed to create new identity:', e);
		}
	}
</script>

<div class="network-page">
	<h1>Network</h1>

	<ConnectionCard
		{networkStatus}
		{isConnecting}
		on:connect={handleConnect}
		on:disconnect={handleDisconnect}
		on:newIdentity={handleNewIdentity}
	/>

	<CircuitsList {circuits} />

	<NymInfo />
</div>

<style>
	.network-page {
		max-width: 800px;
		margin: 0 auto;
	}

	h1 {
		font-size: 28px;
		font-weight: 700;
		margin-bottom: var(--nox-space-lg);
	}
</style>
