<script lang="ts">
	import { page } from '$app/state';
	import { auth } from '$lib/auth.svelte';

	let { children } = $props();
	let mobileMenuOpen = $state(false);

	const navItems = [
		{ href: '/', label: 'Dashboard', icon: 'home' },
		{ href: '/queue', label: 'Hang doi', icon: 'queue' },
		{ href: '/catalog', label: 'Dich vu', icon: 'catalog' },
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
	<aside class="hidden w-64 border-r border-border bg-card lg:block">
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
	</aside>

	<div class="flex flex-1 flex-col">
		<!-- Top bar -->
		<header class="flex h-14 items-center justify-between border-b border-border px-6">
			<button
				class="lg:hidden text-foreground"
				onclick={() => (mobileMenuOpen = !mobileMenuOpen)}
			>
				<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
				</svg>
			</button>
			<div class="text-sm text-muted-foreground">{auth.user?.name ?? ''}</div>
			<button
				onclick={() => auth.logout()}
				class="text-sm text-muted-foreground hover:text-foreground"
			>
				Dang xuat
			</button>
		</header>

		<!-- Mobile nav -->
		{#if mobileMenuOpen}
			<nav class="border-b border-border bg-card p-4 lg:hidden">
				{#each navItems as item}
					<a
						href={item.href}
						onclick={() => (mobileMenuOpen = false)}
						class="block rounded-md px-3 py-2 text-sm font-medium {isActive(item.href)
							? 'bg-primary/10 text-primary'
							: 'text-foreground hover:bg-muted'}"
					>
						{item.label}
					</a>
				{/each}
			</nav>
		{/if}

		<!-- Main content -->
		<main class="flex-1 p-6">
			{@render children()}
		</main>
	</div>
</div>
