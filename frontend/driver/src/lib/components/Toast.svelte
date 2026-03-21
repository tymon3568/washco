<script lang="ts">
	import { toast } from '$lib/toast.svelte';

	const typeStyles: Record<string, string> = {
		success: 'bg-success text-success-foreground',
		error: 'bg-destructive text-destructive-foreground',
		info: 'bg-primary text-primary-foreground'
	};
</script>

{#if toast.items.length > 0}
	<div class="fixed top-4 left-4 right-4 z-[100] mx-auto max-w-lg space-y-2">
		{#each toast.items as item (item.id)}
			<div
				class="flex items-center justify-between rounded-xl px-4 py-3 text-sm font-medium shadow-lg {typeStyles[item.type] ?? typeStyles.info}"
			>
				<span>{item.message}</span>
				<button
					onclick={() => toast.dismiss(item.id)}
					class="ml-3 shrink-0 opacity-70 hover:opacity-100"
					aria-label="Dong"
				>
					<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path d="M18 6 6 18M6 6l12 12" />
					</svg>
				</button>
			</div>
		{/each}
	</div>
{/if}
