<script lang="ts">
	export let variant: 'default' | 'elevated' | 'bordered' | 'glow' = 'default';
	export let padding: 'none' | 'sm' | 'md' | 'lg' = 'md';
	export let hoverable = false;
</script>

<div class="card card-{variant} pad-{padding}" class:hoverable>
	<slot />
</div>

<style>
	.card {
		background: #1a1a26;
		border-radius: 8px;
		transition: all 0.2s ease;
	}
	.pad-none { padding: 0; }
	.pad-sm { padding: 12px; }
	.pad-md { padding: 16px; }
	.pad-lg { padding: 24px; }
	.card-default {
		border: 1px solid #2a2a40;
	}
	.card-elevated {
		background: #1e1e32;
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
	}
	.card-bordered {
		border: 1px solid rgba(102, 255, 255, 0.2);
	}
	.card-glow {
		border: 1px solid rgba(102, 255, 255, 0.3);
		box-shadow: 0 0 20px rgba(102, 255, 255, 0.1);
	}
	.hoverable:hover {
		border-color: rgba(102, 255, 255, 0.5);
		box-shadow: 0 0 30px rgba(102, 255, 255, 0.15);
		transform: translateY(-2px);
	}
</style>
