<script lang="ts">
	import { page } from '$app/state';
	import { auth } from '$lib/auth.svelte';

	let { children } = $props();
	let mobileMenuOpen = $state(false);

	const navItems = [
		{ href: '/', label: 'Dashboard', icon: 'home' },
		{ href: '/queue', label: 'Hang doi', icon: 'queue' },
		{ href: '/bookings', label: 'Dat lich', icon: 'calendar' },
		{ href: '/catalog', label: 'Dich vu', icon: 'catalog' },
		{ href: '/reviews', label: 'Danh gia', icon: 'star' },
		{ href: '/promotions', label: 'Khuyen mai', icon: 'tag' },
		{ href: '/analytics', label: 'Bao cao', icon: 'chart' },
		{ href: '/settings', label: 'Cai dat', icon: 'settings' }
	];

	function isActive(href: string) {
		if (href === '/') return page.url.pathname === '/';
		return page.url.pathname.startsWith(href);
	}
</script>

<div class="flex min-h-screen">
	<!-- Desktop Sidebar -->
	<aside class="hidden w-64 shrink-0 border-r border-border bg-card lg:block">
		<div class="flex h-14 items-center border-b border-border px-6">
			<span class="text-lg font-bold text-primary">WashCo</span>
		</div>
		<nav class="space-y-1 p-4">
			{#each navItems as item}
				<a
					href={item.href}
					class="block rounded-md px-3 py-2 text-sm font-medium transition-colors {isActive(item.href)
						? 'bg-primary/10 text-primary'
						: 'text-foreground hover:bg-muted'}"
				>
					{item.label}
				</a>
			{/each}
		</nav>
		<div class="absolute bottom-4 left-4 right-4 lg:left-0 lg:right-0 lg:px-4">
			<button
				onclick={() => auth.logout()}
				class="w-full rounded-md px-3 py-2 text-left text-sm text-muted-foreground hover:bg-muted hover:text-foreground"
			>
				Dang xuat
			</button>
		</div>
	</aside>

	<div class="flex flex-1 flex-col">
		<!-- Top bar -->
		<header class="flex h-14 items-center justify-between border-b border-border px-4 lg:px-6">
			<div class="flex items-center gap-3">
				<span class="text-lg font-bold text-primary lg:hidden">WashCo</span>
			</div>
			<div class="hidden text-sm text-muted-foreground sm:block">{auth.user?.name ?? ''}</div>
			<button
				onclick={() => auth.logout()}
				class="text-sm text-muted-foreground hover:text-foreground lg:hidden"
			>
				Dang xuat
			</button>
			<button
				onclick={() => auth.logout()}
				class="hidden text-sm text-muted-foreground hover:text-foreground lg:block"
			>
				Dang xuat
			</button>
		</header>

		<!-- Main content -->
		<main class="flex-1 overflow-y-auto p-4 pb-20 lg:p-6 lg:pb-6">
			{@render children()}
		</main>
	</div>
</div>

<!-- Mobile bottom navigation -->
<nav class="fixed bottom-0 left-0 right-0 z-50 border-t border-border bg-card lg:hidden">
	<div class="flex items-center justify-around">
		{#each navItems as item}
			<a
				href={item.href}
				class="flex flex-1 flex-col items-center gap-0.5 py-2 text-xs transition-colors {isActive(item.href)
					? 'text-primary'
					: 'text-muted-foreground'}"
			>
				{#if item.icon === 'home'}
					<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
					</svg>
				{:else if item.icon === 'queue'}
					<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
					</svg>
				{:else if item.icon === 'calendar'}
					<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
					</svg>
				{:else if item.icon === 'catalog'}
					<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
					</svg>
				{:else if item.icon === 'star'}
					<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z" />
					</svg>
				{:else if item.icon === 'tag'}
					<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
					</svg>
				{:else if item.icon === 'chart'}
					<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
					</svg>
				{:else if item.icon === 'settings'}
					<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
						<path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
					</svg>
				{/if}
				<span>{item.label}</span>
			</a>
		{/each}
	</div>
</nav>
