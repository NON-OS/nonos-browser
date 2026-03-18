<script lang="ts">
</script>

<article class="doc-article">
	<header class="doc-header">
		<h1>System Architecture</h1>
		<p class="tagline">Interconnected components providing privacy-preserving services.</p>
	</header>

	<section class="doc-section">
		<h2>Architecture Layers</h2>
		<div class="layers">
			<div class="layer">
				<span class="layer-name">User Layer</span>
				<div class="layer-items">
					<code>NONOS Browser</code>
					<code>Wallet</code>
					<code>Staking UI</code>
				</div>
			</div>
			<div class="layer">
				<span class="layer-name">Network Layer</span>
				<div class="layer-items">
					<code class="highlight">Nym Mixnet</code>
					<code>P2P Network</code>
					<code>SOCKS5 Proxy</code>
				</div>
			</div>
			<div class="layer">
				<span class="layer-name">Service Layer</span>
				<div class="layer-items">
					<code>NONOS Daemon</code>
					<code>Privacy Mixer</code>
					<code>ZK Credentials</code>
				</div>
			</div>
			<div class="layer">
				<span class="layer-name">Blockchain Layer</span>
				<div class="layer-items">
					<code>Ethereum</code>
					<code>NOX Token</code>
					<code>Smart Contracts</code>
				</div>
			</div>
		</div>
	</section>

	<section class="doc-section">
		<h2>Data Flow</h2>
		<p>User requests flow through the Nym mixnet for network-level privacy, then to the NONOS daemon which handles privacy operations. All economic activity settles on Ethereum smart contracts.</p>
	</section>
</article>

<style>
	.doc-article { max-width: 800px; line-height: 1.7; }
	.doc-header { margin-bottom: 3rem; padding-bottom: 2rem; border-bottom: 1px solid #1a1a1a; }
	.doc-header h1 { font-size: 2rem; font-weight: 600; margin-bottom: 0.5rem; }
	.tagline { color: #66FFFF; font-size: 1.1rem; }
	.doc-section { margin-bottom: 2.5rem; }
	.doc-section h2 { font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #e6edf3; }
	.doc-section p { color: #9ca3af; }

	.layers { display: flex; flex-direction: column; gap: 0.75rem; }
	.layer { display: flex; align-items: center; gap: 1.5rem; padding: 1rem; background: #0a0a0a; border-radius: 8px; }
	.layer-name { color: #9ca3af; font-size: 0.875rem; min-width: 120px; }
	.layer-items { display: flex; gap: 0.5rem; flex-wrap: wrap; }
	.layer-items code { font-family: 'SF Mono', monospace; font-size: 0.75rem; color: #e6edf3; background: #111; padding: 0.25rem 0.75rem; border-radius: 4px; }
	.layer-items code.highlight { color: #66FFFF; border: 1px solid #66FFFF; background: rgba(102, 255, 255, 0.1); }

	@media (max-width: 600px) {
		.layer { flex-direction: column; align-items: flex-start; gap: 0.5rem; }
	}
</style>
