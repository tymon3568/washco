<script lang="ts">
	import { locationState } from '$lib/location.svelte';

	let open = $state(false);
</script>

{#if locationState.locations.length > 1}
	<div class="relative">
		<button
			onclick={() => (open = !open)}
			class="flex w-full items-center gap-2 rounded-md border border-border bg-background px-3 py-1.5 text-sm transition-colors hover:bg-muted"
		>
			<svg class="h-4 w-4 shrink-0 text-muted-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
				<path stroke-linecap="round" stroke-linejoin="round" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
				<path stroke-linecap="round" stroke-linejoin="round" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
			</svg>
			<span class="truncate">{locationState.current?.name ?? 'Chọn cơ sở'}</span>
			<svg class="ml-auto h-4 w-4 shrink-0 text-muted-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
				<path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
			</svg>
		</button>

		{#if open}
			<!-- backdrop -->
			<button class="fixed inset-0 z-40" onclick={() => (open = false)} aria-label="Đóng"></button>
			<div class="absolute left-0 right-0 z-50 mt-1 rounded-md border border-border bg-card shadow-lg">
				{#each locationState.locations as loc}
					<button
						onclick={() => {
							locationState.select(loc.id);
							open = false;
						}}
						class="flex w-full items-center gap-2 px-3 py-2 text-left text-sm transition-colors hover:bg-muted {loc.id === locationState.selectedId
							? 'bg-primary/10 text-primary'
							: 'text-foreground'}"
					>
						<span class="truncate">{loc.name}</span>
						<span class="ml-auto text-xs text-muted-foreground">{loc.city}</span>
					</button>
				{/each}
			</div>
		{/if}
	</div>
{:else if locationState.current}
	<div class="flex items-center gap-2 px-3 py-1.5 text-sm text-muted-foreground">
		<svg class="h-4 w-4 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
			<path stroke-linecap="round" stroke-linejoin="round" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
			<path stroke-linecap="round" stroke-linejoin="round" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
		</svg>
		<span class="truncate">{locationState.current.name}</span>
	</div>
{/if}
