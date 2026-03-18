<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		WalletSetup,
		SendModal,
		ReceiveModal,
		MnemonicDisplay,
		BalanceCards,
		StakingSection,
		NetworkSwitcher,
		WalletActions,
		AddressCard,
		SecurityInfo
	} from '$lib/wallet';

	let walletState: 'locked' | 'unlocked' | 'none' = 'none';
	let address = '';
	let balances = { eth: '0', nox: '0', sepolia_eth: '0', sepolia_nox: '0' };
	let mnemonic = '';
	let blake3Key = '';
	let showMnemonic = false;
	let isLoading = false;
	let error = '';
	let successMessage = '';
	let selectedNetwork: 'mainnet' | 'sepolia' = 'mainnet';
	let isRefreshing = false;

	let showSendModal = false;
	let showReceiveModal = false;
	let sendLoading = false;
	let sendError = '';
	let sendSuccess = '';

	let stakingStatus = {
		staked_amount: '0',
		tier: 'None',
		tier_multiplier: '0x',
		pending_rewards: '0',
		current_epoch: 0,
		next_tier_threshold: '1,000 NOX',
		estimated_apy: '0%'
	};
	let stakingLoading = false;
	let showStaking = false;

	let balanceInterval: ReturnType<typeof setInterval>;

	onMount(() => {
		const init = async () => {
			let retries = 0;
			while (!window.nonos && retries < 20) {
				await new Promise(r => setTimeout(r, 250));
				retries++;
			}

			if (!window.nonos) {
				error = 'NONOS bridge not available. Please restart the app.';
				return;
			}

			await checkWallet();

			balanceInterval = setInterval(async () => {
				if (walletState === 'unlocked') {
					await updateBalances();
				}
			}, 10000);
		};

		init();
	});

	onDestroy(() => {
		if (balanceInterval) clearInterval(balanceInterval);
	});

	async function checkWallet() {
		if (!window.nonos) {
			error = 'NONOS bridge not initialized';
			return;
		}
		try {
			const addr = await window.nonos.wallet.getAddress();
			if (addr) {
				address = addr;
				walletState = 'unlocked';
				await updateBalances();
			}
		} catch {
			walletState = 'none';
		}
	}

	async function updateBalances() {
		if (!window.nonos) return;
		try {
			const status = await window.nonos.wallet.getStatus();
			if (status) {
				balances = {
					eth: status.eth_balance || '0',
					nox: status.nox_balance || '0',
					sepolia_eth: status.sepolia_eth_balance || '0',
					sepolia_nox: status.sepolia_nox_balance || '0'
				};
			}
		} catch (e) {
			console.error('Failed to get balances:', e);
		}
	}

	async function refreshBalances() {
		isRefreshing = true;
		await updateBalances();
		setTimeout(() => isRefreshing = false, 500);
	}

	async function handleCreate(e: CustomEvent<{ name: string }>) {
		if (!window.nonos) {
			error = 'NONOS bridge not available';
			return;
		}
		isLoading = true;
		error = '';
		try {
			mnemonic = await window.nonos.wallet.create(e.detail.name);
			address = await window.nonos.wallet.getAddress() || '';
			blake3Key = `blake3:${address.slice(2, 18)}...${address.slice(-16)}`;
			walletState = 'unlocked';
			showMnemonic = true;
			await updateBalances();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to create wallet';
		} finally {
			isLoading = false;
		}
	}

	async function handleImport(e: CustomEvent<{ mnemonic: string; name: string }>) {
		if (!window.nonos || !e.detail.mnemonic.trim()) return;
		isLoading = true;
		error = '';
		try {
			await window.nonos.wallet.import(e.detail.mnemonic.trim(), e.detail.name);
			address = await window.nonos.wallet.getAddress() || '';
			walletState = 'unlocked';
			await updateBalances();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to import wallet';
		} finally {
			isLoading = false;
		}
	}

	async function switchNetwork(e: CustomEvent<{ network: 'mainnet' | 'sepolia' }>) {
		if (!window.nonos) return;
		try {
			await window.nonos.wallet.setSelectedNetwork(e.detail.network);
			selectedNetwork = e.detail.network;
		} catch (e) {
			console.error('Failed to switch network:', e);
		}
	}

	async function loadStakingStatus() {
		if (!window.nonos) return;
		try {
			stakingStatus = await window.nonos.staking.getStatus();
		} catch (e) {
			console.error('Failed to load staking status:', e);
		}
	}

	async function handleStake(e: CustomEvent<{ amount: string }>) {
		if (!window.nonos) return;
		stakingLoading = true;
		error = '';
		try {
			await window.nonos.staking.stake(e.detail.amount);
			await loadStakingStatus();
			await updateBalances();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to stake';
		} finally {
			stakingLoading = false;
		}
	}

	async function handleUnstake(e: CustomEvent<{ amount: string }>) {
		if (!window.nonos) return;
		stakingLoading = true;
		error = '';
		try {
			await window.nonos.staking.unstake(e.detail.amount);
			await loadStakingStatus();
			await updateBalances();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to unstake';
		} finally {
			stakingLoading = false;
		}
	}

	async function handleClaim() {
		if (!window.nonos) return;
		stakingLoading = true;
		error = '';
		try {
			await window.nonos.staking.claimRewards();
			await loadStakingStatus();
			await updateBalances();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to claim rewards';
		} finally {
			stakingLoading = false;
		}
	}

	async function handleSend(e: CustomEvent<{ token: 'eth' | 'nox'; to: string; amount: string }>) {
		const { token, to, amount } = e.detail;

		if (!window.nonos || !to || !amount) {
			sendError = 'Please fill in all fields';
			return;
		}

		if (!to.match(/^0x[a-fA-F0-9]{40}$/)) {
			sendError = 'Invalid Ethereum address';
			return;
		}

		const amountNum = parseFloat(amount);
		if (isNaN(amountNum) || amountNum <= 0) {
			sendError = 'Invalid amount';
			return;
		}

		const currentBalance = token === 'eth' ? parseFloat(balances.eth) : parseFloat(balances.nox);
		if (amountNum > currentBalance) {
			sendError = `Insufficient ${token.toUpperCase()} balance`;
			return;
		}

		sendLoading = true;
		sendError = '';
		sendSuccess = '';

		try {
			let result: string;
			if (token === 'eth') {
				result = await window.nonos.wallet.sendEth(to, amount);
			} else {
				result = await window.nonos.wallet.sendNox(to, amount);
			}
			sendSuccess = result;
			await updateBalances();
			setTimeout(() => {
				showSendModal = false;
				successMessage = `Successfully sent ${amount} ${token.toUpperCase()}!`;
				setTimeout(() => successMessage = '', 5000);
			}, 2000);
		} catch (e) {
			sendError = e instanceof Error ? e.message : typeof e === 'string' ? e : 'Transaction failed';
		} finally {
			sendLoading = false;
		}
	}

	function toggleStaking() {
		showStaking = !showStaking;
		if (showStaking) loadStakingStatus();
	}

	async function lockWallet() {
		if (!window.nonos) return;
		await window.nonos.wallet.lock();
		walletState = 'locked';
		address = '';
		balances = { eth: '0', nox: '0', sepolia_eth: '0', sepolia_nox: '0' };
	}

	function dismissError() {
		error = '';
	}

	function dismissSuccess() {
		successMessage = '';
	}
</script>

<div class="wallet-page">
	<div class="page-header">
		<h1>Wallet</h1>
		<p class="subtitle">Secure NOX & ETH wallet with stealth address support</p>
	</div>

	{#if error}
		<div class="error-banner">
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<circle cx="12" cy="12" r="10"/>
				<path d="M15 9l-6 6M9 9l6 6"/>
			</svg>
			<span>{error}</span>
			<button class="dismiss" on:click={dismissError}>
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M18 6L6 18M6 6l12 12"/>
				</svg>
			</button>
		</div>
	{/if}

	{#if successMessage}
		<div class="success-banner">
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
				<polyline points="22 4 12 14.01 9 11.01"/>
			</svg>
			<span>{successMessage}</span>
			<button class="dismiss" on:click={dismissSuccess}>
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M18 6L6 18M6 6l12 12"/>
				</svg>
			</button>
		</div>
	{/if}

	{#if walletState === 'none'}
		<WalletSetup
			{isLoading}
			{error}
			on:create={handleCreate}
			on:import={handleImport}
		/>
	{:else if showMnemonic}
		<MnemonicDisplay
			{mnemonic}
			{blake3Key}
			on:confirm={() => showMnemonic = false}
		/>
	{:else}
		<div class="wallet-dashboard">
			<AddressCard {address} />

			<NetworkSwitcher
				{selectedNetwork}
				on:switch={switchNetwork}
			/>

			<BalanceCards
				{balances}
				{isRefreshing}
				on:refresh={refreshBalances}
			/>

			<WalletActions
				on:send={() => showSendModal = true}
				on:receive={() => showReceiveModal = true}
				on:stake={toggleStaking}
				on:lock={lockWallet}
			/>

			{#if showStaking}
				<StakingSection
					{stakingStatus}
					{stakingLoading}
					on:stake={handleStake}
					on:unstake={handleUnstake}
					on:claim={handleClaim}
				/>
			{/if}

			<SecurityInfo />
		</div>
	{/if}
</div>

<SendModal
	show={showSendModal}
	{balances}
	{sendLoading}
	{sendError}
	{sendSuccess}
	on:close={() => showSendModal = false}
	on:send={handleSend}
/>

<ReceiveModal
	show={showReceiveModal}
	{address}
	on:close={() => showReceiveModal = false}
/>

<style>
	.wallet-page {
		max-width: 800px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: var(--nox-space-xl);
	}

	.page-header h1 {
		font-size: var(--nox-text-2xl);
		font-weight: var(--nox-font-semibold);
		margin-bottom: var(--nox-space-xs);
	}

	.subtitle {
		color: var(--nox-text-muted);
		font-size: var(--nox-text-sm);
	}

	.error-banner {
		display: flex;
		align-items: center;
		gap: var(--nox-space-md);
		background: var(--nox-error-bg);
		border: 1px solid var(--nox-error);
		color: var(--nox-error);
		padding: var(--nox-space-md);
		border-radius: var(--nox-radius-lg);
		margin-bottom: var(--nox-space-lg);
	}

	.error-banner svg {
		width: 20px;
		height: 20px;
		flex-shrink: 0;
	}

	.error-banner span {
		flex: 1;
	}

	.success-banner {
		display: flex;
		align-items: center;
		gap: var(--nox-space-md);
		background: var(--nox-success-bg);
		border: 1px solid var(--nox-success);
		color: var(--nox-success);
		padding: var(--nox-space-md);
		border-radius: var(--nox-radius-lg);
		margin-bottom: var(--nox-space-lg);
	}

	.success-banner svg {
		width: 20px;
		height: 20px;
		flex-shrink: 0;
	}

	.success-banner span {
		flex: 1;
	}

	.dismiss {
		padding: var(--nox-space-xs);
		border-radius: var(--nox-radius-sm);
	}

	.error-banner .dismiss {
		color: var(--nox-error);
	}

	.error-banner .dismiss:hover {
		background: rgba(255, 68, 102, 0.2);
	}

	.success-banner .dismiss {
		color: var(--nox-success);
	}

	.success-banner .dismiss:hover {
		background: rgba(0, 255, 136, 0.2);
	}

	.dismiss svg {
		width: 16px;
		height: 16px;
	}

	.wallet-dashboard {
		display: flex;
		flex-direction: column;
		gap: var(--nox-space-lg);
	}
</style>
