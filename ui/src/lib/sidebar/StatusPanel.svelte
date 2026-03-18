<script lang="ts">
	export let networkStatus: { connected: boolean; bootstrap_progress: number; circuits: number };
</script>

<div class="status-panel" class:connected={networkStatus.connected}>
	<div class="status-header">
		<svg class="nym-logo" viewBox="0 0 24 24" fill="none">
			<path d="M12 2L4 6v6c0 5.55 3.84 10.74 8 12 4.16-1.26 8-6.45 8-12V6l-8-4z" stroke="currentColor" stroke-width="1.5"/>
			<path d="M8 12l3 3 5-6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
		</svg>
		<span class="status-title">NYM 2-HOP</span>
	</div>

	<div class="status-main">
		<div class="status-indicator" class:active={networkStatus.connected}>
			<div class="pulse-ring"></div>
			<div class="pulse-ring delay"></div>
			<div class="status-core"></div>
		</div>
		<div class="status-text">
			<span class="status-label">{networkStatus.connected ? 'PROTECTED' : 'OFFLINE'}</span>
			<span class="status-sub">{networkStatus.connected ? '2-hop routing active' : 'Connect to browse'}</span>
		</div>
	</div>

	{#if !networkStatus.connected && networkStatus.bootstrap_progress > 0}
		<div class="bootstrap-progress">
			<div class="progress-header">
				<span>Bootstrapping...</span>
				<span class="progress-pct">{networkStatus.bootstrap_progress}%</span>
			</div>
			<div class="progress-track">
				<div class="progress-fill" style="width: {networkStatus.bootstrap_progress}%"></div>
				<div class="progress-glow" style="left: {networkStatus.bootstrap_progress}%"></div>
			</div>
		</div>
	{/if}

	{#if networkStatus.connected}
		<div class="mixnode-grid">
			<div class="mixnode active"><span>ENTRY</span></div>
			<div class="mixnode active delay1"><span>EXIT</span></div>
		</div>
	{/if}

	<div class="status-footer">
		<div class="stat">
			<span class="stat-label">Status</span>
			<span class="stat-value">{networkStatus.connected ? 'OK' : '-'}</span>
		</div>
		<div class="stat">
			<span class="stat-label">Hops</span>
			<span class="stat-value">2</span>
		</div>
		<div class="stat">
			<span class="stat-label">Port</span>
			<span class="stat-value">1080</span>
		</div>
	</div>
</div>

<style>
	.status-panel {
		margin: var(--nox-space-md);
		padding: var(--nox-space-md);
		background: linear-gradient(145deg, rgba(20, 30, 45, 0.9), rgba(15, 20, 35, 0.95));
		border: 1px solid rgba(255, 68, 102, 0.3);
		border-radius: var(--nox-radius-lg);
		position: relative;
		overflow: hidden;
	}
	.status-panel.connected {
		border-color: rgba(0, 255, 136, 0.4);
		box-shadow: 0 0 30px rgba(0, 255, 136, 0.1), inset 0 0 30px rgba(0, 255, 136, 0.03);
	}
	.status-panel::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, transparent 40%, rgba(102, 255, 255, 0.03));
		pointer-events: none;
	}
	.status-header {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 12px;
	}
	.nym-logo {
		width: 18px;
		height: 18px;
		color: var(--nox-accent-primary);
	}
	.status-title {
		font-size: 10px;
		font-weight: 600;
		color: var(--nox-accent-primary);
		letter-spacing: 2px;
	}
	.status-main {
		display: flex;
		align-items: center;
		gap: 14px;
		padding: 12px 0;
	}
	.status-indicator {
		width: 40px;
		height: 40px;
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.status-core {
		width: 16px;
		height: 16px;
		background: var(--nox-error);
		border-radius: 50%;
		position: relative;
		z-index: 2;
		box-shadow: 0 0 10px var(--nox-error);
	}
	.status-indicator.active .status-core {
		background: #00ff88;
		box-shadow: 0 0 15px #00ff88;
	}
	.pulse-ring {
		position: absolute;
		width: 100%;
		height: 100%;
		border: 2px solid var(--nox-error);
		border-radius: 50%;
		opacity: 0;
	}
	.status-indicator.active .pulse-ring {
		border-color: #00ff88;
		animation: pulse-expand 2s ease-out infinite;
	}
	.pulse-ring.delay {
		animation-delay: 1s;
	}
	@keyframes pulse-expand {
		0% { transform: scale(0.5); opacity: 0.8; }
		100% { transform: scale(1.3); opacity: 0; }
	}
	.status-text {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}
	.status-label {
		font-size: 14px;
		font-weight: 600;
		color: #ff4466;
		letter-spacing: 1px;
	}
	.status-panel.connected .status-label {
		color: #00ff88;
	}
	.status-sub {
		font-size: 11px;
		color: var(--nox-text-muted);
	}
	.bootstrap-progress {
		padding: 10px 0;
	}
	.progress-header {
		display: flex;
		justify-content: space-between;
		font-size: 11px;
		color: var(--nox-text-muted);
		margin-bottom: 6px;
	}
	.progress-pct {
		color: var(--nox-accent-primary);
		font-family: var(--nox-font-mono);
	}
	.progress-track {
		height: 4px;
		background: rgba(102, 255, 255, 0.1);
		border-radius: 2px;
		position: relative;
		overflow: visible;
	}
	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #66FFFF, #00ff88);
		border-radius: 2px;
		transition: width 0.3s ease;
	}
	.progress-glow {
		position: absolute;
		top: -4px;
		width: 8px;
		height: 12px;
		background: #66FFFF;
		border-radius: 2px;
		box-shadow: 0 0 12px #66FFFF;
		transform: translateX(-50%);
	}
	.mixnode-grid {
		display: flex;
		gap: 6px;
		padding: 10px 0;
		justify-content: center;
	}
	.mixnode {
		width: 36px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(40, 50, 70, 0.6);
		border: 1px solid rgba(102, 255, 255, 0.2);
		border-radius: 4px;
		font-size: 8px;
		font-weight: 600;
		color: var(--nox-text-muted);
		letter-spacing: 0.5px;
	}
	.mixnode.active {
		background: rgba(0, 255, 136, 0.1);
		border-color: rgba(0, 255, 136, 0.4);
		color: #00ff88;
		animation: mixnode-pulse 2s ease-in-out infinite;
	}
	.mixnode.delay1 { animation-delay: 0.2s; }
	.mixnode.delay2 { animation-delay: 0.4s; }
	.mixnode.delay3 { animation-delay: 0.6s; }
	.mixnode.delay4 { animation-delay: 0.8s; }
	@keyframes mixnode-pulse {
		0%, 100% { box-shadow: 0 0 8px rgba(0, 255, 136, 0.2); }
		50% { box-shadow: 0 0 16px rgba(0, 255, 136, 0.4); }
	}
	.status-footer {
		display: flex;
		justify-content: space-between;
		padding-top: 10px;
		border-top: 1px solid rgba(102, 255, 255, 0.1);
		margin-top: 8px;
	}
	.stat {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}
	.stat-label {
		font-size: 9px;
		color: var(--nox-text-muted);
		text-transform: uppercase;
		letter-spacing: 1px;
	}
	.stat-value {
		font-size: 14px;
		font-family: var(--nox-font-mono);
		color: var(--nox-accent-primary);
		font-weight: 500;
	}
</style>
