<script lang="ts">
	import type { Snippet } from 'svelte';

	let { icon, title, description, action }: {
		icon?: string;
		title: string;
		description?: string;
		action?: Snippet;
	} = $props();
</script>

<div class="flex flex-col items-center justify-center rounded-xl bg-card px-6 py-10 text-center">
	{#if icon}
		<span class="text-4xl">{icon}</span>
	{/if}
	<p class="mt-3 font-medium text-foreground">{title}</p>
	{#if description}
		<p class="mt-1 text-sm text-muted-foreground">{description}</p>
	{/if}
	{#if action}
		<div class="mt-4">
			{@render action()}
		</div>
	{/if}
</div>
