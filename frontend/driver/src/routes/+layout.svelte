<script lang="ts">
	import '../app.css';
	import { page } from '$app/state';

	let { children } = $props();

	let currentPath = $derived(page.url.pathname);
</script>

<div class="min-h-screen pb-16">
	<header class="sticky top-0 z-50 border-b border-border bg-card px-4 py-3">
		<div class="mx-auto flex max-w-lg items-center justify-between">
			<a href="/" class="text-lg font-bold text-primary">WashCo</a>
			<span class="text-xs text-muted-foreground">Tim tiem rua xe</span>
		</div>
	</header>
	<main class="mx-auto max-w-lg px-4 py-4">
		{@render children()}
	</main>
</div>

<nav class="fixed bottom-0 left-0 right-0 z-50 border-t border-border bg-card">
	<div class="mx-auto flex max-w-lg">
		<a
			href="/"
			class="flex flex-1 flex-col items-center gap-1 py-3 text-xs transition-colors {currentPath === '/' ? 'text-primary' : 'text-muted-foreground'}"
		>
			<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
				<circle cx="11" cy="11" r="8" />
				<path d="m21 21-4.3-4.3" />
			</svg>
			Tim kiem
		</a>
		<a
			href="/history"
			class="flex flex-1 flex-col items-center gap-1 py-3 text-xs transition-colors {currentPath === '/history' ? 'text-primary' : 'text-muted-foreground'}"
		>
			<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
				<circle cx="12" cy="12" r="10" />
				<path d="M12 6v6l4 2" />
			</svg>
			Lich su
		</a>
	</div>
</nav>
