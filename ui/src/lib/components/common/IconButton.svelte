<script lang="ts">
	export let variant: 'default' | 'primary' | 'ghost' = 'default';
	export let size: 'sm' | 'md' | 'lg' = 'md';
	export let disabled = false;
	export let title = '';
</script>

<button
	class="icon-btn icon-btn-{variant} icon-btn-{size}"
	{disabled}
	{title}
	on:click
	on:keydown
>
	<slot />
</button>

<style>
	.icon-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.15s ease;
		border: none;
	}
	.icon-btn-sm { width: 28px; height: 28px; }
	.icon-btn-md { width: 36px; height: 36px; }
	.icon-btn-lg { width: 44px; height: 44px; }
	.icon-btn-default {
		background: #1a1a26;
		color: #a0a0b8;
		border: 1px solid #2a2a40;
	}
	.icon-btn-default:hover:not(:disabled) {
		color: #66FFFF;
		border-color: rgba(102, 255, 255, 0.3);
	}
	.icon-btn-primary {
		background: linear-gradient(135deg, #66FFFF 0%, #2E5C5C 100%);
		color: #0a0a0f;
	}
	.icon-btn-primary:hover:not(:disabled) {
		box-shadow: 0 0 15px rgba(102, 255, 255, 0.3);
	}
	.icon-btn-ghost {
		background: transparent;
		color: #6b6b80;
	}
	.icon-btn-ghost:hover:not(:disabled) { color: #66FFFF; }
	.icon-btn:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
