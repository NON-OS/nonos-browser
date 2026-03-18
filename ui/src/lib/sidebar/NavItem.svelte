<script lang="ts">
	export let path: string;
	export let label: string;
	export let icon: string;
	export let active: boolean;
</script>

<a
	href={path}
	class="nav-item"
	class:active
	aria-current={active ? 'page' : undefined}
>
	<span class="nav-icon">
		{#if icon === 'globe'}
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<circle cx="12" cy="12" r="10"/>
				<path d="M2 12h20M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/>
			</svg>
		{:else if icon === 'wallet'}
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<rect x="2" y="6" width="20" height="14" rx="2"/>
				<path d="M2 10h20"/>
				<circle cx="16" cy="14" r="2"/>
			</svg>
		{:else if icon === 'network'}
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<circle cx="12" cy="5" r="3"/>
				<circle cx="5" cy="19" r="3"/>
				<circle cx="19" cy="19" r="3"/>
				<path d="M12 8v4M8.5 14.5L5.5 17M15.5 14.5l3 2.5"/>
				<circle cx="12" cy="14" r="2"/>
			</svg>
		{:else if icon === 'server'}
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<rect x="3" y="4" width="18" height="6" rx="1"/>
				<rect x="3" y="14" width="18" height="6" rx="1"/>
				<circle cx="7" cy="7" r="1" fill="currentColor"/>
				<circle cx="7" cy="17" r="1" fill="currentColor"/>
			</svg>
		{:else if icon === 'staking'}
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/>
			</svg>
		{:else if icon === 'shield'}
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
			</svg>
		{:else if icon === 'docs'}
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
				<polyline points="14 2 14 8 20 8"/>
				<line x1="16" y1="13" x2="8" y2="13"/>
				<line x1="16" y1="17" x2="8" y2="17"/>
				<polyline points="10 9 9 9 8 9"/>
			</svg>
		{:else if icon === 'settings'}
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<circle cx="12" cy="12" r="3"/>
				<path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
			</svg>
		{/if}
	</span>
	<span class="nav-label">{label}</span>
	{#if active}
		<span class="nav-indicator"></span>
	{/if}
</a>

<style>
	.nav-item {
		display: flex;
		align-items: center;
		gap: var(--nox-space-md);
		padding: 10px 14px;
		border-radius: 8px;
		color: rgba(160, 160, 184, 0.8);
		text-decoration: none;
		transition: all 0.2s ease;
		position: relative;
		overflow: hidden;
	}
	.nav-item::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(90deg, transparent, rgba(102, 255, 255, 0.05), transparent);
		transform: translateX(-100%);
		transition: transform 0.4s ease;
	}
	.nav-item:hover::before {
		transform: translateX(100%);
	}
	.nav-item:hover {
		background: rgba(102, 255, 255, 0.05);
		color: #e0e0f0;
	}
	.nav-item.active {
		background: linear-gradient(135deg, rgba(102, 255, 255, 0.12), rgba(102, 255, 255, 0.06));
		color: #66FFFF;
		border: 1px solid rgba(102, 255, 255, 0.2);
	}
	.nav-icon {
		width: 22px;
		height: 22px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		transition: all 0.2s;
	}
	.nav-item:hover .nav-icon {
		transform: scale(1.1);
	}
	.nav-item.active .nav-icon {
		filter: drop-shadow(0 0 6px currentColor);
	}
	.nav-icon svg {
		width: 18px;
		height: 18px;
	}
	.nav-label {
		font-size: 13px;
		font-weight: 500;
		letter-spacing: 0.3px;
	}
	.nav-indicator {
		position: absolute;
		left: 0;
		top: 50%;
		transform: translateY(-50%);
		width: 3px;
		height: 24px;
		background: linear-gradient(180deg, #66FFFF, #00cc99);
		border-radius: 0 3px 3px 0;
		box-shadow: 0 0 12px #66FFFF, 0 0 24px rgba(102, 255, 255, 0.3);
	}
</style>
