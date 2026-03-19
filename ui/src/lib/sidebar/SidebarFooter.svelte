<script lang="ts">
	import { onMount } from 'svelte';
	let version = '1.0.6';
	onMount(() => { if (window.nonos?.version) version = window.nonos.version; });
</script>

<div class="sidebar-footer">
	<div class="footer-row">
		<span class="version">v{version}</span>
		<a href="https://nonos.systems" target="_blank" rel="noopener" class="footer-link">
			nonos.systems
		</a>
	</div>
</div>

<style>
	.sidebar-footer {
		padding: var(--nox-space-md) var(--nox-space-lg);
		border-top: 1px solid var(--nox-border-subtle);
	}

	.footer-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.version {
		font-size: var(--nox-text-xs);
		font-family: var(--nox-font-mono);
		color: var(--nox-text-disabled);
	}

	.footer-link {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
		text-decoration: none;
		transition: color var(--nox-transition-fast);
	}

	.footer-link:hover {
		color: var(--nox-accent-primary);
	}
</style>
