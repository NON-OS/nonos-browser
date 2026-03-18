<script lang="ts">
	import { page } from '$app/stores';
	import { SidebarLogo, NavItem, StatusPanel, SidebarFooter } from '$lib/sidebar';

	export let networkStatus: { connected: boolean; bootstrap_progress: number; circuits: number };

	const navItems = [
		{ path: '/', label: 'Browser', icon: 'globe' },
		{ path: '/wallet', label: 'Wallet', icon: 'wallet' },
		{ path: '/staking', label: 'Staking', icon: 'staking' },
		{ path: '/network', label: 'Network', icon: 'network' },
		{ path: '/privacy', label: 'Privacy', icon: 'shield' },
		{ path: '/docs', label: 'Docs', icon: 'docs' },
		{ path: '/settings', label: 'Settings', icon: 'settings' }
	];

	$: currentPath = $page.url.pathname;
</script>

<aside class="sidebar">
	<SidebarLogo />

	<nav class="sidebar-nav">
		{#each navItems as item}
			<NavItem
				path={item.path}
				label={item.label}
				icon={item.icon}
				active={currentPath === item.path}
			/>
		{/each}
	</nav>

	<StatusPanel {networkStatus} />

	<SidebarFooter />
</aside>

<style>
	.sidebar {
		width: 240px;
		min-width: 240px;
		height: 100%;
		display: flex;
		flex-direction: column;
		background: linear-gradient(180deg, rgba(13, 13, 26, 0.98) 0%, rgba(10, 10, 20, 0.99) 100%);
		border-right: 1px solid rgba(102, 255, 255, 0.08);
		position: relative;
	}

	.sidebar::after {
		content: '';
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: 1px;
		background: linear-gradient(180deg, rgba(102, 255, 255, 0.2), transparent 30%, transparent 70%, rgba(102, 255, 255, 0.1));
		pointer-events: none;
	}

	.sidebar-nav {
		flex: 1;
		padding: var(--nox-space-md);
		display: flex;
		flex-direction: column;
		gap: var(--nox-space-2xs);
	}
</style>
