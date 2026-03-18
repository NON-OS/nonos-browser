<script lang="ts"></script>

<div class="mode-selector">
	<div class="mode-label">Nym Mixnet Architecture</div>
	<div class="mode-info">
		<div class="architecture-visual">
			<span class="node">YOU</span><span class="arrow">→</span>
			<span class="node gateway">Gateway</span><span class="arrow">→</span>
			<span class="node mix">Mix 1</span><span class="arrow">→</span>
			<span class="node mix">Mix 2</span><span class="arrow">→</span>
			<span class="node mix">Mix 3</span><span class="arrow">→</span>
			<span class="node gateway">Exit</span><span class="arrow">→</span>
			<span class="node">WEB</span>
		</div>
		<div class="mode-description">All traffic routed through 5 nodes with Sphinx packet encryption. Cover traffic defeats timing analysis. Your IP is never exposed.</div>
	</div>
</div>

<style>
	.mode-selector { margin-bottom: var(--nox-space-lg); padding: var(--nox-space-md); background: var(--nox-bg-tertiary); border-radius: var(--nox-radius-md); }
	.mode-label { font-size: 12px; color: var(--nox-text-muted); text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: var(--nox-space-sm); text-align: center; }
	.mode-info { display: flex; flex-direction: column; gap: var(--nox-space-sm); }
	.architecture-visual { display: flex; align-items: center; justify-content: center; gap: 4px; padding: var(--nox-space-sm) 0; flex-wrap: wrap; }
	.node { padding: 4px 8px; background: var(--nox-bg-secondary); border: 1px solid var(--nox-border); border-radius: var(--nox-radius-sm); font-size: 10px; font-weight: 500; color: var(--nox-text-secondary); }
	.node.gateway { background: rgba(102, 255, 255, 0.1); border-color: rgba(102, 255, 255, 0.3); color: var(--nox-accent-primary); }
	.node.mix { background: rgba(0, 255, 136, 0.1); border-color: rgba(0, 255, 136, 0.3); color: var(--nox-success); }
	.arrow { color: var(--nox-text-muted); font-size: 10px; }
	.mode-description { font-size: 12px; color: var(--nox-text-secondary); text-align: center; line-height: 1.5; }
</style>
