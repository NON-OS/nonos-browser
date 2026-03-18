<script lang="ts">
	import { onMount } from 'svelte';
	import { NodeStatus, FeeDistribution, WorkCategories, StakeForm, EconomyInfo } from '$lib/node';

	let nodeRunning = false;
	let nodeStatus = { running: false, connected_nodes: 0, quality: 0, total_requests: 0 };
	let stakingStatus = {
		staked_amount: '0',
		tier: 'None',
		tier_multiplier: '0x',
		pending_rewards: '0',
		current_epoch: 0,
		next_tier_threshold: '1,000 NOX',
		estimated_apy: '0%'
	};
	let isLoading = false;
	let startTime = Date.now();

	onMount(() => {
		updateStatus();
		const interval = setInterval(updateStatus, 5000);
		return () => clearInterval(interval);
	});

	async function updateStatus() {
		if (!window.nonos) return;
		try {
			nodeStatus = await window.nonos.node.getStatus();
			nodeRunning = nodeStatus.running;

			try {
				stakingStatus = await window.nonos.staking.getStatus();
			} catch (e) {
				// Wallet may not be initialized yet
			}
		} catch (e) {
			console.error('Failed to get status:', e);
		}
	}

	async function handleToggleNode() {
		if (!window.nonos) return;
		isLoading = true;
		try {
			if (nodeRunning) {
				await window.nonos.node.stopEmbedded();
			} else {
				await window.nonos.node.startEmbedded();
				startTime = Date.now();
			}
			await updateStatus();
		} catch (e) {
			console.error('Failed to toggle node:', e);
		} finally {
			isLoading = false;
		}
	}

	async function handleStake(e: CustomEvent<{ amount: string }>) {
		if (!window.nonos) return;
		isLoading = true;
		try {
			await window.nonos.staking.stake(e.detail.amount);
			await updateStatus();
		} catch (e) {
			console.error('Failed to stake:', e);
		} finally {
			isLoading = false;
		}
	}

	async function handleClaim() {
		if (!window.nonos) return;
		isLoading = true;
		try {
			await window.nonos.staking.claimRewards();
			await updateStatus();
		} catch (e) {
			console.error('Failed to claim rewards:', e);
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="node-page">
	<h1>Node & Staking</h1>

	<NodeStatus
		{nodeRunning}
		{nodeStatus}
		{stakingStatus}
		{isLoading}
		{startTime}
		on:toggle={handleToggleNode}
	/>

	<FeeDistribution />

	<WorkCategories />

	<StakeForm {isLoading} on:stake={handleStake} on:claim={handleClaim} />

	<EconomyInfo />
</div>

<style>
	.node-page {
		max-width: 900px;
		margin: 0 auto;
	}

	h1 {
		font-size: 28px;
		font-weight: 700;
		margin-bottom: var(--nox-space-lg);
	}
</style>
