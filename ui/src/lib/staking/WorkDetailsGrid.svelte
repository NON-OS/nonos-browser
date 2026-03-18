<script lang="ts">
	import type { WorkMetrics } from './types';
	import { formatBytes, formatNumber } from './types';

	export let workMetrics: WorkMetrics;
</script>

<div class="work-details">
	<div class="detail-card">
		<h4>Traffic Relay</h4>
		<div class="detail-row">
			<span>Bytes Relayed</span>
			<span>{formatBytes(workMetrics.traffic_relay.bytes_relayed)}</span>
		</div>
		<div class="detail-row">
			<span>Sessions</span>
			<span>{formatNumber(workMetrics.traffic_relay.relay_sessions)}</span>
		</div>
		<div class="detail-row">
			<span>Success Rate</span>
			<span>{workMetrics.traffic_relay.relay_sessions > 0 ? ((workMetrics.traffic_relay.successful_relays / workMetrics.traffic_relay.relay_sessions) * 100).toFixed(1) : 0}%</span>
		</div>
	</div>

	<div class="detail-card">
		<h4>ZK Proofs</h4>
		<div class="detail-row">
			<span>Generated</span>
			<span>{formatNumber(workMetrics.zk_proofs.proofs_generated)}</span>
		</div>
		<div class="detail-row">
			<span>Verified</span>
			<span>{formatNumber(workMetrics.zk_proofs.proofs_verified)}</span>
		</div>
		<div class="detail-row">
			<span>Avg Gen Time</span>
			<span>{workMetrics.zk_proofs.avg_generation_time_ms.toFixed(0)}ms</span>
		</div>
	</div>

	<div class="detail-card">
		<h4>Mixer Operations</h4>
		<div class="detail-row">
			<span>Deposits</span>
			<span>{formatNumber(workMetrics.mixer_ops.deposits_processed)}</span>
		</div>
		<div class="detail-row">
			<span>Spends</span>
			<span>{formatNumber(workMetrics.mixer_ops.spends_processed)}</span>
		</div>
		<div class="detail-row">
			<span>Pool Participations</span>
			<span>{formatNumber(workMetrics.mixer_ops.pool_participations)}</span>
		</div>
	</div>

	<div class="detail-card">
		<h4>Entropy</h4>
		<div class="detail-row">
			<span>Bytes Contributed</span>
			<span>{formatBytes(workMetrics.entropy.entropy_bytes_contributed)}</span>
		</div>
		<div class="detail-row">
			<span>Requests Served</span>
			<span>{formatNumber(workMetrics.entropy.entropy_requests_served)}</span>
		</div>
		<div class="detail-row">
			<span>Quality Score</span>
			<span>{workMetrics.entropy.quality_score.toFixed(1)}</span>
		</div>
	</div>
</div>

<style>
	.work-details {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: var(--nox-space-md);
	}

	.detail-card {
		background: var(--nox-bg-tertiary);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-md);
	}

	.detail-card h4 {
		font-size: var(--nox-text-sm);
		margin-bottom: var(--nox-space-md);
	}

	.detail-row {
		display: flex;
		justify-content: space-between;
		font-size: var(--nox-text-xs);
		padding: var(--nox-space-xs) 0;
		border-bottom: 1px solid var(--nox-border);
	}

	.detail-row:last-child {
		border-bottom: none;
	}

	.detail-row span:first-child {
		color: var(--nox-text-muted);
	}

	@media (max-width: 800px) {
		.work-details {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 500px) {
		.work-details {
			grid-template-columns: 1fr;
		}
	}
</style>
