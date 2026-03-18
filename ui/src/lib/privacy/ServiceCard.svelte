<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { PrivacyService } from './types';

	const dispatch = createEventDispatcher();

	export let service: PrivacyService;
	export let networkConnected = false;

	function start() {
		dispatch('start', { serviceId: service.id });
	}

	function stop() {
		dispatch('stop', { serviceId: service.id });
	}
</script>

<div class="service-card" class:active={service.active}>
	<div class="service-header">
		<div class="service-icon" class:active={service.active}>
			{#if service.icon === 'globe'}
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<circle cx="12" cy="12" r="10"/>
					<path d="M2 12h20M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/>
				</svg>
			{:else if service.icon === 'shield'}
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
					<path d="M9 12l2 2 4-4"/>
				</svg>
			{:else if service.icon === 'shuffle'}
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<polyline points="16 3 21 3 21 8"/>
					<line x1="4" y1="20" x2="21" y2="3"/>
					<polyline points="21 16 21 21 16 21"/>
					<line x1="15" y1="15" x2="21" y2="21"/>
					<line x1="4" y1="4" x2="9" y2="9"/>
				</svg>
			{:else if service.icon === 'dice'}
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<rect x="3" y="3" width="18" height="18" rx="2"/>
					<circle cx="8" cy="8" r="1.5" fill="currentColor"/>
					<circle cx="16" cy="8" r="1.5" fill="currentColor"/>
					<circle cx="8" cy="16" r="1.5" fill="currentColor"/>
					<circle cx="16" cy="16" r="1.5" fill="currentColor"/>
					<circle cx="12" cy="12" r="1.5" fill="currentColor"/>
				</svg>
			{:else if service.icon === 'key'}
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"/>
				</svg>
			{/if}
		</div>
		<div class="service-weight">
			<span class="weight-value">{service.weight}</span>
			<span class="weight-label">of rewards</span>
		</div>
	</div>

	<div class="service-body">
		<h3>{service.name}</h3>
		<p class="service-desc">{service.description}</p>
		<div class="service-category">{service.category}</div>
	</div>

	<div class="service-footer">
		<div class="service-price">
			<span class="price-value">{service.pricePerUnit} NOX</span>
			<span class="price-unit">per {service.unit}</span>
		</div>
		{#if service.active}
			<button class="btn active-btn" on:click={stop}>
				<span class="active-dot"></span>
				Active
			</button>
		{:else}
			<button
				class="btn start-btn"
				on:click={start}
				disabled={!networkConnected && service.id !== 'traffic_relay'}
			>
				Start Service
			</button>
		{/if}
	</div>
</div>

<style>
	.service-card {
		background: var(--nox-bg-secondary);
		border: 1px solid var(--nox-border);
		border-radius: var(--nox-radius-xl);
		padding: var(--nox-space-lg);
		display: flex;
		flex-direction: column;
		transition: all var(--nox-transition-fast);
	}

	.service-card:hover {
		border-color: var(--nox-border-light);
	}

	.service-card.active {
		border-color: var(--nox-success);
		background: rgba(34, 197, 94, 0.03);
	}

	.service-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: var(--nox-space-md);
	}

	.service-icon {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--nox-bg-tertiary);
		border-radius: var(--nox-radius-lg);
		color: var(--nox-text-secondary);
	}

	.service-icon.active {
		background: var(--nox-success-bg);
		color: var(--nox-success);
	}

	.service-icon svg {
		width: 24px;
		height: 24px;
	}

	.service-weight {
		text-align: right;
	}

	.weight-value {
		display: block;
		font-size: var(--nox-text-lg);
		font-weight: var(--nox-font-semibold);
		color: var(--nox-accent-primary);
	}

	.weight-label {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
	}

	.service-body {
		flex: 1;
		margin-bottom: var(--nox-space-md);
	}

	.service-body h3 {
		font-size: var(--nox-text-base);
		font-weight: var(--nox-font-semibold);
		margin-bottom: var(--nox-space-sm);
	}

	.service-desc {
		font-size: var(--nox-text-sm);
		color: var(--nox-text-secondary);
		line-height: 1.5;
		margin-bottom: var(--nox-space-md);
	}

	.service-category {
		display: inline-block;
		padding: var(--nox-space-2xs) var(--nox-space-sm);
		background: var(--nox-accent-glow);
		color: var(--nox-accent-primary);
		font-size: var(--nox-text-xs);
		font-weight: var(--nox-font-medium);
		border-radius: var(--nox-radius-sm);
	}

	.service-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-top: var(--nox-space-md);
		border-top: 1px solid var(--nox-border);
	}

	.service-price {
		display: flex;
		flex-direction: column;
	}

	.price-value {
		font-family: var(--nox-font-mono);
		font-weight: var(--nox-font-semibold);
	}

	.price-unit {
		font-size: var(--nox-text-xs);
		color: var(--nox-text-muted);
	}

	.btn.start-btn {
		padding: var(--nox-space-sm) var(--nox-space-lg);
		background: var(--nox-accent-gradient);
		color: var(--nox-bg-primary);
		border: none;
		border-radius: var(--nox-radius-md);
		font-weight: var(--nox-font-medium);
		cursor: pointer;
		transition: all var(--nox-transition-fast);
	}

	.btn.start-btn:hover:not(:disabled) {
		box-shadow: var(--nox-shadow-glow);
	}

	.btn.start-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn.active-btn {
		display: flex;
		align-items: center;
		gap: var(--nox-space-sm);
		padding: var(--nox-space-sm) var(--nox-space-lg);
		background: var(--nox-success-bg);
		color: var(--nox-success);
		border: 1px solid var(--nox-success);
		border-radius: var(--nox-radius-md);
		font-weight: var(--nox-font-medium);
		cursor: pointer;
	}

	.active-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--nox-success);
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.5; }
	}
</style>
