<script lang="ts">
	import { onMount } from 'svelte';

	let error = '';
	let success = '';
	let loading = false;
	let lockAmount = '';
	let selectedTier = 0;

	interface LockTier { id: number; duration_days: number; multiplier: number; multiplier_display: string; }
	interface LockInfo { lock_id: number; amount: string; tier: number; lock_end: number; pending_rewards: string; }
	interface LPStatus { total_locked: string; weighted_total: string; total_pending_rewards: string; current_epoch: number; }

	let lockTiers: LockTier[] = [];
	let myLocks: LockInfo[] = [];
	let lpStatus: LPStatus = { total_locked: '0', weighted_total: '0', total_pending_rewards: '0', current_epoch: 0 };

	onMount(async () => {
		let retries = 0;
		while (!window.nonos && retries < 20) { await new Promise(r => setTimeout(r, 250)); retries++; }
		if (!window.nonos) { error = 'NONOS bridge not available'; return; }
		await loadData();
	});

	async function loadData() {
		if (!window.nonos) return;
		try {
			lockTiers = await window.nonos.lpStaking.getTiers();
			const status = await window.nonos.lpStaking.getStatus();
			lpStatus = { total_locked: status.total_locked, weighted_total: status.weighted_total, total_pending_rewards: status.total_pending_rewards, current_epoch: status.current_epoch };
			myLocks = status.locks || [];
		} catch (e) { console.error('Failed to load LP data:', e); }
	}

	async function handleLock() {
		if (!window.nonos || !lockAmount) return;
		loading = true; error = ''; success = '';
		try { success = await window.nonos.lpStaking.lock(lockAmount, selectedTier); lockAmount = ''; await loadData(); }
		catch (e) { error = e instanceof Error ? e.message : String(e); }
		finally { loading = false; }
	}

	async function handleUnlock(lockId: number) {
		if (!window.nonos) return;
		loading = true; error = ''; success = '';
		try { success = await window.nonos.lpStaking.unlock(lockId); await loadData(); }
		catch (e) { error = e instanceof Error ? e.message : String(e); }
		finally { loading = false; }
	}

	async function handleClaim(lockId: number) {
		if (!window.nonos) return;
		loading = true; error = ''; success = '';
		try { success = await window.nonos.lpStaking.claimRewards(lockId); await loadData(); }
		catch (e) { error = e instanceof Error ? e.message : String(e); }
		finally { loading = false; }
	}

	async function handleClaimAll() {
		if (!window.nonos) return;
		loading = true; error = ''; success = '';
		try { success = await window.nonos.lpStaking.claimAllRewards(); await loadData(); }
		catch (e) { error = e instanceof Error ? e.message : String(e); }
		finally { loading = false; }
	}

	function formatDate(ts: number): string { return new Date(ts * 1000).toLocaleDateString(); }
</script>

<div class="page cyber-scanline">
	<header class="header">
		<div class="title-row">
			<span class="sys-id">SYS://STAKING</span>
			<h1>NOX STAKING PROTOCOL</h1>
		</div>
		<span class="badge cyber-badge cyber-badge-amber">SEPOLIA TESTNET</span>
	</header>

	{#if error}<div class="alert alert-error"><span class="alert-icon">!</span>{error}<button on:click={() => error = ''}>×</button></div>{/if}
	{#if success}<div class="alert alert-success"><span class="alert-icon">✓</span>{success}<button on:click={() => success = ''}>×</button></div>{/if}

	<section class="stats-grid">
		<div class="cyber-stat cyber-hud-corners">
			<div class="cyber-stat-label">Total Locked</div>
			<div class="cyber-stat-value">{lpStatus.total_locked}</div>
			<div class="stat-unit">NOX</div>
		</div>
		<div class="cyber-stat cyber-hud-corners">
			<div class="cyber-stat-label">Weighted Total</div>
			<div class="cyber-stat-value">{lpStatus.weighted_total}</div>
			<div class="stat-unit">WNOX</div>
		</div>
		<div class="cyber-stat cyber-hud-corners cyber-glow-pulse">
			<div class="cyber-stat-label">Pending Rewards</div>
			<div class="cyber-stat-value highlight">{lpStatus.total_pending_rewards}</div>
			<div class="stat-unit">NOX</div>
		</div>
		<div class="cyber-stat cyber-hud-corners">
			<div class="cyber-stat-label">Current Epoch</div>
			<div class="cyber-stat-value">{lpStatus.current_epoch}</div>
			<div class="stat-unit">BLOCK</div>
		</div>
	</section>

	<section class="panel cyber-panel">
		<h2 class="cyber-section-title">Lock NOX Tokens</h2>

		<div class="lock-controls">
			<div class="input-group">
				<label class="cyber-label">Amount to Lock</label>
				<input type="text" class="cyber-input" bind:value={lockAmount} placeholder="0.00" disabled={loading} />
			</div>
			<div class="input-group">
				<label class="cyber-label">Lock Duration</label>
				<select class="cyber-input" bind:value={selectedTier} disabled={loading}>
					{#each lockTiers as tier, i}
						<option value={i}>{tier.duration_days}D @ {tier.multiplier_display}</option>
					{/each}
				</select>
			</div>
			<button class="cyber-btn-solid" on:click={handleLock} disabled={loading || !lockAmount}>
				{loading ? 'PROCESSING...' : 'EXECUTE LOCK'}
			</button>
		</div>

		<div class="tier-grid">
			{#each lockTiers as tier, i}
				<button class="tier-card cyber-card" class:selected={selectedTier === i} on:click={() => selectedTier = i}>
					<span class="tier-duration">{tier.duration_days}D</span>
					<span class="tier-mult">{tier.multiplier_display}</span>
					<span class="tier-label">MULTIPLIER</span>
				</button>
			{/each}
		</div>
	</section>

	{#if myLocks.length > 0}
		<section class="panel cyber-panel">
			<div class="panel-header">
				<h2 class="cyber-section-title">Active Locks</h2>
				{#if parseFloat(lpStatus.total_pending_rewards) > 0}
					<button class="cyber-btn" on:click={handleClaimAll} disabled={loading}>CLAIM ALL REWARDS</button>
				{/if}
			</div>

			<table class="cyber-table">
				<thead>
					<tr>
						<th>Amount</th>
						<th>Tier</th>
						<th>Unlock Date</th>
						<th>Rewards</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each myLocks as lock}
						<tr>
							<td>{lock.amount} NOX</td>
							<td>{lockTiers[lock.tier]?.multiplier_display || '1.00x'}</td>
							<td>{formatDate(lock.lock_end)}</td>
							<td class="rewards">{lock.pending_rewards} NOX</td>
							<td class="actions">
								{#if parseFloat(lock.pending_rewards) > 0}
									<button class="action-btn" on:click={() => handleClaim(lock.lock_id)} disabled={loading}>CLAIM</button>
								{/if}
								{#if Date.now() / 1000 >= lock.lock_end}
									<button class="action-btn primary" on:click={() => handleUnlock(lock.lock_id)} disabled={loading}>UNLOCK</button>
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</section>
	{/if}
</div>

<style>
	.page { max-width: 1000px; margin: 0 auto; padding: 2rem 1rem; }

	.header { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 2rem; }
	.title-row { display: flex; flex-direction: column; gap: 0.25rem; }
	.sys-id { font-family: 'JetBrains Mono', monospace; font-size: 0.65rem; color: var(--cyber-cyan); opacity: 0.6; letter-spacing: 0.2em; }
	.header h1 { font-size: 1.5rem; font-weight: 700; letter-spacing: 0.1em; text-transform: uppercase; }

	.alert { display: flex; align-items: center; gap: 0.75rem; padding: 0.75rem 1rem; margin-bottom: 1.5rem; border: 1px solid; font-size: 0.8rem; }
	.alert-error { background: rgba(255, 0, 51, 0.1); border-color: var(--cyber-red); color: var(--cyber-red); }
	.alert-success { background: rgba(0, 255, 102, 0.1); border-color: var(--cyber-green); color: var(--cyber-green); }
	.alert-icon { font-weight: 700; }
	.alert button { margin-left: auto; background: none; border: none; font-size: 1.25rem; cursor: pointer; color: inherit; }

	.stats-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 1rem; margin-bottom: 2rem; }
	.stat-unit { font-size: 0.6rem; color: var(--cyber-text-dim); letter-spacing: 0.15em; margin-top: 0.25rem; }
	.highlight { color: var(--cyber-green) !important; }

	.panel { margin-bottom: 1.5rem; }
	.panel-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem; }

	.lock-controls { display: grid; grid-template-columns: 1fr 1fr auto; gap: 1rem; align-items: end; margin-bottom: 1.5rem; }
	.input-group { display: flex; flex-direction: column; }

	.tier-grid { display: grid; grid-template-columns: repeat(5, 1fr); gap: 0.5rem; }
	.tier-card { padding: 1rem; text-align: center; cursor: pointer; transition: all 0.2s; }
	.tier-card.selected { border-color: var(--cyber-cyan); box-shadow: var(--cyber-glow-cyan); }
	.tier-duration { display: block; font-family: 'JetBrains Mono', monospace; font-size: 1rem; font-weight: 700; margin-bottom: 0.25rem; }
	.tier-mult { display: block; font-size: 1.5rem; font-weight: 700; color: var(--cyber-cyan); }
	.tier-label { display: block; font-size: 0.5rem; color: var(--cyber-text-dim); letter-spacing: 0.15em; margin-top: 0.25rem; }

	.rewards { color: var(--cyber-green); }
	.actions { display: flex; gap: 0.5rem; }
	.action-btn { padding: 0.35rem 0.75rem; font-size: 0.65rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; background: var(--cyber-surface); border: 1px solid var(--cyber-border); color: var(--cyber-text); cursor: pointer; transition: all 0.15s; }
	.action-btn:hover { border-color: var(--cyber-cyan); color: var(--cyber-cyan); }
	.action-btn.primary { background: var(--cyber-cyan); border-color: var(--cyber-cyan); color: var(--cyber-void); }
	.action-btn:disabled { opacity: 0.5; cursor: not-allowed; }

	@media (max-width: 900px) {
		.stats-grid { grid-template-columns: repeat(2, 1fr); }
		.tier-grid { grid-template-columns: repeat(3, 1fr); }
		.lock-controls { grid-template-columns: 1fr; }
	}
	@media (max-width: 600px) {
		.tier-grid { grid-template-columns: repeat(2, 1fr); }
		.header { flex-direction: column; gap: 1rem; }
	}
</style>
