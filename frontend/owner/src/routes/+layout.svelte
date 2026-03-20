<script lang="ts">
	import '../app.css';
	import { auth } from '$lib/auth.svelte';
	import { page } from '$app/state';

	let { children } = $props();

	const publicRoutes = ['/login'];
	let isPublicRoute = $derived(publicRoutes.includes(page.url.pathname));

	$effect(() => {
		auth.init();
	});
</script>

{#if auth.isLoading}
	<div class="flex min-h-screen items-center justify-center">
		<div class="text-sm text-muted-foreground">Dang tai...</div>
	</div>
{:else if !auth.isAuthenticated && !isPublicRoute}
	<script>
		import { goto } from '$app/navigation';
		goto('/login');
	</script>
{:else}
	{@render children()}
{/if}
