<script lang="ts">
</script>

<div class="info-card">
	<h3>About Nym Mixnet</h3>
	<p>
		Nym provides next generation privacy through a decentralized mixnet.
		Your traffic is encrypted with Sphinx packets, mixed with cover traffic, and routed
		through multiple mix nodes, defeating metadata surveillance.
	</p>
	<ul>
		<li>Sphinx packet encryption with layered decryption</li>
		<li>Traffic mixing defeats timing analysis</li>
		<li>Decentralized mix node infrastructure</li>
		<li>Cover traffic hides communication patterns</li>
	</ul>
</div>

<style>
	.info-card {
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-lg);
		padding: var(--nox-space-lg);
	}

	.info-card h3 {
		font-size: 16px;
		margin-bottom: var(--nox-space-md);
	}

	.info-card p {
		color: var(--nox-text-secondary);
		font-size: 14px;
		margin-bottom: var(--nox-space-md);
	}

	.info-card ul {
		list-style: none;
	}

	.info-card li {
		padding: var(--nox-space-xs) 0;
		font-size: 13px;
		color: var(--nox-text-secondary);
		display: flex;
		align-items: center;
		gap: var(--nox-space-sm);
	}

	.info-card li::before {
		content: '';
		width: 16px;
		height: 16px;
		background: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%2300ff88' stroke-width='2'%3E%3Cpolyline points='20 6 9 17 4 12'/%3E%3C/svg%3E") center/contain no-repeat;
		flex-shrink: 0;
	}
</style>
