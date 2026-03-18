<script lang="ts"></script>

<div class="emission-info">
	<h4>Bootstrap Pool Emission (Year 1)</h4>
	<div class="emission-stats">
		<div class="emission-stat"><span class="stat-label">Annual</span><span class="stat-value">14,000,000 NOX</span></div>
		<div class="emission-stat"><span class="stat-label">Per Epoch (7 days)</span><span class="stat-value">269,231 NOX</span></div>
		<div class="emission-stat"><span class="stat-label">Bootstrap Pool</span><span class="stat-value">40,000,000 NOX</span></div>
		<div class="emission-stat"><span class="stat-label">Yearly Decay</span><span class="stat-value">30%</span></div>
	</div>
</div>

<style>
	.emission-info { background: var(--nox-bg-tertiary); border-radius: var(--nox-radius-lg); padding: var(--nox-space-lg); }
	.emission-info h4 { font-size: var(--nox-text-base); margin-bottom: var(--nox-space-md); }
	.emission-stats { display: grid; grid-template-columns: repeat(4, 1fr); gap: var(--nox-space-md); }
	.emission-stat { text-align: center; }
	.stat-label { display: block; font-size: var(--nox-text-xs); color: var(--nox-text-muted); margin-bottom: var(--nox-space-xs); }
	.stat-value { font-size: var(--nox-text-sm); font-weight: var(--nox-font-semibold); font-family: var(--nox-font-mono); }
	@media (max-width: 800px) { .emission-stats { grid-template-columns: repeat(2, 1fr); } }
</style>
