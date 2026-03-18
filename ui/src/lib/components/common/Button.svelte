<script lang="ts">
	export let variant: 'primary' | 'secondary' | 'ghost' | 'danger' = 'primary';
	export let size: 'sm' | 'md' | 'lg' = 'md';
	export let disabled = false;
	export let loading = false;
	export let type: 'button' | 'submit' = 'button';
</script>

<button
	{type}
	{disabled}
	class="btn btn-{variant} btn-{size}"
	class:loading
	on:click
	on:keydown
>
	{#if loading}
		<span class="spinner"></span>
	{/if}
	<slot />
</button>

<style>
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		font-weight: 500;
		border-radius: 6px;
		transition: all 0.2s ease;
		cursor: pointer;
	}
	.btn-sm { padding: 6px 12px; font-size: 13px; }
	.btn-md { padding: 10px 20px; font-size: 14px; }
	.btn-lg { padding: 14px 28px; font-size: 16px; }
	.btn-primary {
		background: linear-gradient(135deg, #66FFFF 0%, #2E5C5C 100%);
		color: #0a0a0f;
	}
	.btn-primary:hover:not(:disabled) {
		box-shadow: 0 0 20px rgba(102, 255, 255, 0.3);
		transform: translateY(-1px);
	}
	.btn-secondary {
		background: #1a1a26;
		border: 1px solid rgba(102, 255, 255, 0.3);
		color: #66FFFF;
	}
	.btn-secondary:hover:not(:disabled) {
		border-color: #66FFFF;
		background: rgba(102, 255, 255, 0.1);
	}
	.btn-ghost {
		background: transparent;
		color: #a0a0b8;
	}
	.btn-ghost:hover:not(:disabled) { color: #66FFFF; }
	.btn-danger {
		background: linear-gradient(135deg, #ff4466 0%, #aa2244 100%);
		color: white;
	}
	.btn:disabled { opacity: 0.5; cursor: not-allowed; }
	.spinner {
		width: 14px;
		height: 14px;
		border: 2px solid currentColor;
		border-top-color: transparent;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}
	@keyframes spin {
		to { transform: rotate(360deg); }
	}
</style>
