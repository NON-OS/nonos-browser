<script lang="ts">
</script>

<article class="doc-article">
	<header class="doc-header">
		<h1>Nym Mixnet</h1>
		<p class="tagline">Network-level privacy through mix network infrastructure.</p>
	</header>

	<section class="doc-section">
		<h2>How It Works</h2>
		<p>NONOS routes all traffic through the Nym mixnet, a decentralized network that provides strong anonymity guarantees against traffic analysis.</p>
	</section>

	<section class="doc-section">
		<h2>Privacy Features</h2>
		<div class="features-list">
			<div class="feature-item">
				<span class="feature-name">Mix Network</span>
				<p>Traffic routed through multiple mix nodes, making analysis impossible.</p>
			</div>
			<div class="feature-item">
				<span class="feature-name">Sphinx Packets</span>
				<p>Layered encryption with uniform packet sizes prevents correlation.</p>
			</div>
			<div class="feature-item">
				<span class="feature-name">Cover Traffic</span>
				<p>Decoy packets mixed with real traffic prevent timing analysis.</p>
			</div>
			<div class="feature-item">
				<span class="feature-name">SOCKS5 Proxy</span>
				<p>Applications connect via local proxy on port 1080.</p>
			</div>
		</div>
	</section>

	<section class="doc-section">
		<h2>Connection Details</h2>
		<div class="connection-table">
			<div class="conn-row"><span>Protocol</span><code>SOCKS5</code></div>
			<div class="conn-row"><span>Local Address</span><code>127.0.0.1:1080</code></div>
			<div class="conn-row"><span>Client Binary</span><code>nym-socks5-client</code></div>
		</div>
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

	.features-list { display: flex; flex-direction: column; gap: 0.75rem; }
	.feature-item { padding: 1rem; background: #0a0a0a; border-radius: 8px; }
	.feature-name { display: block; font-weight: 600; color: #e6edf3; margin-bottom: 0.25rem; }
	.feature-item p { margin: 0; font-size: 0.875rem; color: #9ca3af; }

	.connection-table { display: flex; flex-direction: column; gap: 0.5rem; }
	.conn-row { display: flex; justify-content: space-between; padding: 0.75rem 1rem; background: #0a0a0a; border-radius: 6px; }
	.conn-row span { color: #9ca3af; }
	.conn-row code { font-family: 'SF Mono', monospace; font-size: 0.875rem; color: #66FFFF; }
</style>
