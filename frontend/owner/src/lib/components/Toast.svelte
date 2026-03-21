<script lang="ts">
	import { toast } from '$lib/toast.svelte';
</script>

{#if toast.items.length > 0}
	<div class="fixed top-4 right-4 z-[100] flex flex-col gap-2">
		{#each toast.items as item (item.id)}
			<div
				class="flex items-center gap-3 rounded-lg border px-4 py-3 text-sm shadow-lg backdrop-blur-sm transition-all
					{item.type === 'success'
					? 'border-green-500/30 bg-green-500/10 text-green-400'
					: item.type === 'error'
						? 'border-destructive/30 bg-destructive/10 text-destructive'
						: 'border-border bg-card text-foreground'}"
			>
				{#if item.type === 'success'}
					<svg class="h-4 w-4 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
					</svg>
				{:else if item.type === 'error'}
					<svg class="h-4 w-4 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
					</svg>
				{/if}
				<span>{item.message}</span>
				<button
					onclick={() => toast.dismiss(item.id)}
					aria-label="Dong thong bao"
					class="ml-2 shrink-0 text-muted-foreground hover:text-foreground"
				>
					<svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>
		{/each}
	</div>
{/if}
