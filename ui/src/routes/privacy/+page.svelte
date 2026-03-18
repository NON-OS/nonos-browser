<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		NodesPanel,
		StatsBar,
		ActiveSessions,
		ServiceCard,
		EconomyInfo
	} from '$lib/privacy';
	import type { PrivacyService, ActiveSession, NetworkStats, BootstrapNode } from '$lib/privacy';

	let services: PrivacyService[] = [
		{
			id: 'zk_proofs',
			name: 'ZK Credential Proofs',
			description: 'Generate zero-knowledge proofs for age verification, identity attestation, or membership without revealing personal data.',
			category: 'ZK Proof Generation',
			weight: '30%',
			pricePerUnit: '5',
			unit: 'proof',
			icon: 'shield',
			active: false
		},
		{
			id: 'mixer',
			name: 'Token Mixing',
			description: 'Mix NOX/ETH transactions using commitment-nullifier pools. Standard mixing 0.10%, priority mixing 0.25% fee.',
			category: 'Mixer Operations',
			weight: '25%',
			pricePerUnit: '0.10%',
			unit: 'amount',
			icon: 'shuffle',
			active: false
		},
		{
			id: 'tracking_blocker',
			name: 'Tracking Blocker',
			description: 'Block trackers, fingerprinting scripts, and surveillance beacons. Normalizes browser fingerprints for anonymity.',
			category: 'Tracking Protection',
			weight: '20%',
			pricePerUnit: '1',
			unit: '1k blocks',
			icon: 'eye-off',
			active: false
		},
		{
			id: 'pir',
			name: 'Private Information Retrieval',
			description: 'Fetch content from databases without revealing which item you requested. Perfect for private lookups and queries.',
			category: 'PIR Queries',
			weight: '15%',
			pricePerUnit: '2',
			unit: 'query',
			icon: 'search',
			active: false
		},
		{
			id: 'stealth_registry',
			name: 'Stealth Address Registry',
			description: 'Register one-time stealth addresses for unlinkable payments. Registration 50 NOX, lookups 1 NOX.',
			category: 'Registry Operations',
			weight: '10%',
			pricePerUnit: '50',
			unit: 'registration',
			icon: 'key',
			active: false
		}
	];

	let activeSessions: ActiveSession[] = [];
	let networkStats: NetworkStats = {
		connected_nodes: 0,
		total_bandwidth_gb: '0',
		proofs_generated: 0,
		mix_operations: 0,
		trackers_blocked: 0,
		active_users: 0
	};

	let networkConnected = false;
	let refreshInterval: ReturnType<typeof setInterval>;

	// Official NONOS bootstrap nodes from daemon config
	let bootstrapNodes: BootstrapNode[] = [
		{ id: 'nl', name: 'Netherlands', ip: '150.40.127.8', flag: '🇳🇱', status: 'checking' }
	];

	onMount(async () => {
		let retries = 0;
		while (!window.nonos && retries < 20) {
			await new Promise(r => setTimeout(r, 250));
			retries++;
		}

		if (window.nonos) {
			await loadNetworkStatus();
			refreshInterval = setInterval(loadNetworkStatus, 10000);
		}
	});

	onDestroy(() => {
		if (refreshInterval) clearInterval(refreshInterval);
	});

	let daemonOnline = false;

	async function loadNetworkStatus() {
		if (!window.nonos) return;
		try {
			const nodeStatus = await window.nonos.node.getStatus();
			daemonOnline = nodeStatus.embedded_running || false;

			const status = await window.nonos.network.getStatus();
			networkConnected = status.connected;

			let privacy = {
				zk_proofs_issued: 0,
				zk_verifications: 0,
				tracking_blocked: 0,
				stealth_payments: 0
			};

			if (daemonOnline) {
				try {
					privacy = await window.nonos.privacy.getStats();
				} catch {
					// Daemon API not responding
				}

				// Load mixer status
				try {
					const mixerStatus = await window.nonos.privacy.mixerStatus();
					privacy.stealth_payments = mixerStatus.total_deposits || 0;
				} catch {
					// Mixer not available
				}

				// Check bootstrap node status
				try {
					const connectedPeers = await window.nonos.node.getConnected();
					bootstrapNodes = bootstrapNodes.map(node => ({
						...node,
						status: connectedPeers.some((p: { address: string }) => p.address?.includes(node.ip)) ? 'online' : 'checking'
					}));
				} catch {
					// Unable to check peers
				}
			}

			networkStats = {
				connected_nodes: status.circuits || 0,
				total_bandwidth_gb: '-',
				proofs_generated: privacy.zk_proofs_issued || 0,
				mix_operations: privacy.stealth_payments || 0,
				trackers_blocked: privacy.tracking_blocked || 0,
				active_users: privacy.zk_verifications || 0
			};

			// Services are always-on when daemon is running
			services = services.map(s => ({ ...s, active: daemonOnline }));
		} catch (e) {
			console.error('Failed to load network status:', e);
		}
	}

	async function handleUseService(e: CustomEvent<{ serviceId: string }>) {
		if (!window.nonos || !daemonOnline) return;
		const { serviceId } = e.detail;

		// Handle service-specific actions
		if (serviceId === 'mixer') {
			// Open mixer modal (could dispatch event to parent or use store)
			alert('Token Mixer: Use the Wallet page to deposit/spend through the mixer.');
		} else if (serviceId === 'zk_proofs') {
			alert('ZK Proofs: Generate proofs via CLI: nonos-daemon identity --generate-proof');
		} else if (serviceId === 'tracking_blocker') {
			alert('Tracking Blocker: Active automatically when browsing through Nym.');
		} else if (serviceId === 'pir') {
			alert('PIR: Private lookups available via daemon API.');
		} else if (serviceId === 'stealth_registry') {
			alert('Stealth Registry: Generate stealth addresses in Wallet settings.');
		}
	}

	function handleStartService(e: CustomEvent<{ serviceId: string }>) {
		handleUseService(e);
	}

	function handleStopService(e: CustomEvent<{ serviceId: string }>) {
		// Services are always-on, can't be stopped individually
		alert('Privacy services are always active when daemon is running.');
	}
</script>

<div class="privacy-page">
	<div class="page-header">
		<div class="header-row">
			<h1>Privacy Services</h1>
			<span class="network-badge" class:online={daemonOnline}>
				<span class="dot"></span>
				{daemonOnline ? 'Daemon Online' : 'Daemon Offline'}
			</span>
		</div>
		<p class="subtitle">
			Privacy services powered by NONOS daemon node operators.
			Traffic privacy is handled via Nym Mixnet (see Network tab).
		</p>
	</div>

	<div class="economy-banner">
		<div class="economy-title">Privacy Work Economy</div>
		<div class="economy-split">
			<div class="split-item">
				<span class="split-pct">50%</span>
				<span class="split-label">Node Operators</span>
			</div>
			<div class="split-divider"></div>
			<div class="split-item">
				<span class="split-pct">35%</span>
				<span class="split-label">ZeroState Treasury</span>
			</div>
			<div class="split-divider"></div>
			<div class="split-item">
				<span class="split-pct">15%</span>
				<span class="split-label">Staking Rewards</span>
			</div>
		</div>
	</div>

	<NodesPanel nodes={bootstrapNodes} />

	<StatsBar stats={networkStats} />

	<ActiveSessions sessions={activeSessions} on:stop={handleStopService} />

	<div class="services-section">
		<h2>Available Services</h2>
		<p class="section-desc">
			Node operators earn NOX rewards for providing privacy services.
			All fees distributed according to the Privacy Work Economy model.
		</p>

		<div class="services-grid">
			{#each services as service}
				<ServiceCard
					{service}
					networkConnected={daemonOnline}
					on:start={handleStartService}
					on:stop={handleStopService}
				/>
			{/each}
		</div>
	</div>

	<EconomyInfo />
</div>

<style>
	.privacy-page {
		max-width: 1000px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: var(--nox-space-lg);
	}

	.header-row {
		display: flex;
		align-items: center;
		gap: var(--nox-space-md);
		margin-bottom: var(--nox-space-sm);
	}

	.page-header h1 {
		font-size: var(--nox-text-2xl);
		font-weight: var(--nox-font-semibold);
	}

	.network-badge {
		display: inline-flex;
		align-items: center;
		gap: var(--nox-space-xs);
		padding: 4px 12px;
		border-radius: var(--nox-radius-full);
		font-size: var(--nox-text-xs);
		font-weight: var(--nox-font-medium);
		background: var(--nox-error-bg);
		color: var(--nox-error);
	}

	.network-badge.online {
		background: var(--nox-success-bg);
		color: var(--nox-success);
	}

	.network-badge .dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: currentColor;
	}

	.subtitle {
		color: var(--nox-text-muted);
		font-size: var(--nox-text-sm);
		line-height: 1.6;
	}

	.economy-banner {
		background: linear-gradient(135deg, rgba(0, 212, 170, 0.1), rgba(102, 255, 255, 0.05));
		border: 1px solid rgba(0, 212, 170, 0.3);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-lg);
		margin-bottom: var(--nox-space-xl);
	}

	.economy-title {
		font-size: var(--nox-text-sm);
		font-weight: var(--nox-font-semibold);
		color: var(--nox-accent);
		text-transform: uppercase;
		letter-spacing: 1px;
		margin-bottom: var(--nox-space-md);
		text-align: center;
	}

	.economy-split {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: var(--nox-space-lg);
	}

	.split-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
	}

	.split-pct {
		font-size: var(--nox-text-xl);
		font-weight: var(--nox-font-bold);
		font-family: var(--nox-font-mono);
		color: var(--nox-accent);
	}

	.split-label {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
	}

	.split-divider {
		width: 1px;
		height: 40px;
		background: rgba(102, 255, 255, 0.2);
	}

	.services-section {
		margin-bottom: var(--nox-space-xl);
	}

	.services-section h2 {
		font-size: var(--nox-text-lg);
		font-weight: var(--nox-font-semibold);
		margin-bottom: var(--nox-space-xs);
	}

	.section-desc {
		color: var(--nox-text-muted);
		font-size: var(--nox-text-sm);
		margin-bottom: var(--nox-space-lg);
	}

	.services-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: var(--nox-space-md);
	}
</style>
