<script lang="ts">
	import '../app.css';
	import { goto } from '$app/navigation';
	import { auth } from '$lib/auth.svelte';
	import { page } from '$app/state';
	import Toast from '$lib/components/Toast.svelte';

	let { children } = $props();

	const publicRoutes = ['/login'];
	let isPublicRoute = $derived(publicRoutes.includes(page.url.pathname));

	$effect(() => {
		auth.init();
	});

	$effect(() => {
		if (!auth.isLoading && !auth.isAuthenticated && !isPublicRoute) {
			goto('/login');
		}
	});
</script>

<Toast />

{#if auth.isLoading}
	<div class="flex min-h-screen items-center justify-center">
		<div class="text-sm text-muted-foreground">Dang tai...</div>
	</div>
{:else if auth.isAuthenticated || isPublicRoute}
	{@render children()}
{/if}
