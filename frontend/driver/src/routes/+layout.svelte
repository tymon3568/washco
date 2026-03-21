<script lang="ts">
	import '../app.css';
	import { page } from '$app/state';
	import { auth } from '$lib/auth.svelte';
	import Toast from '$lib/components/Toast.svelte';

	let { children } = $props();

	let currentPath = $derived(page.url.pathname);
	let showNav = $derived(!currentPath.startsWith('/login'));

	$effect(() => {
		auth.init();
	});

	// Register service worker
	$effect(() => {
		if (typeof navigator !== 'undefined' && 'serviceWorker' in navigator) {
			navigator.serviceWorker.register('/sw.js').catch(() => {});
		}
	});

	const navItems = [
		{
			href: '/',
			label: 'Tim kiem',
			icon: 'M21 21l-4.3-4.3M11 19a8 8 0 100-16 8 8 0 000 16z'
		},
		{
			href: '/history',
			label: 'Lich su',
			icon: 'M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10zM12 6v6l4 2'
		},
		{
			href: '/favorites',
			label: 'Yeu thich',
			icon: 'M20.84 4.61a5.5 5.5 0 00-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 00-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 000-7.78z'
		},
		{
			href: '/profile',
			label: 'Tai khoan',
			icon: 'M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2M12 11a4 4 0 100-8 4 4 0 000 8z'
		}
	];
</script>

<Toast />

<div class="min-h-screen {showNav ? 'pb-20' : ''}">
	{#if showNav}
		<header class="sticky top-0 z-50 border-b border-border bg-card/95 px-4 py-3 backdrop-blur-sm">
			<div class="mx-auto flex max-w-lg items-center justify-between">
				<a href="/" class="text-lg font-bold text-primary">WashCo</a>
				<span class="text-xs text-muted-foreground">Tim tiem rua xe</span>
			</div>
		</header>
	{/if}

	<main class="mx-auto max-w-lg px-4 py-4">
		{@render children()}
	</main>
</div>

{#if showNav}
	<nav class="fixed bottom-0 left-0 right-0 z-50 border-t border-border bg-card/95 pb-safe backdrop-blur-sm">
		<div class="mx-auto flex max-w-lg">
			{#each navItems as item (item.href)}
				{@const isActive = item.href === '/' ? currentPath === '/' : currentPath.startsWith(item.href)}
				<a
					href={item.href}
					class="flex flex-1 flex-col items-center gap-1 py-2.5 text-xs transition-colors {isActive ? 'text-primary' : 'text-muted-foreground'}"
				>
					<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path d={item.icon} />
					</svg>
					{item.label}
				</a>
			{/each}
		</div>
	</nav>
{/if}
