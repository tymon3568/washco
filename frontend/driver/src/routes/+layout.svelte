<script lang="ts">
	import '../app.css';
	let { children } = $props();
</script>

<div class="min-h-screen">
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
