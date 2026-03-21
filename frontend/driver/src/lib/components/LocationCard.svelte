<script lang="ts">
	import type { NearbyLocation } from '$lib/api/types';
	import { formatDistance } from '$lib/utils/format';
	import { favorites } from '$lib/favorites.svelte';

	let { location }: { location: NearbyLocation } = $props();
	let isFav = $derived(favorites.isFavorite(location.id));
</script>

<a
	href="/location/{location.id}"
	class="block rounded-xl border border-border bg-card p-4 transition-shadow hover:shadow-sm active:bg-muted/50"
>
	<div class="flex items-start justify-between gap-3">
		<div class="min-w-0 flex-1">
			<h2 class="truncate font-semibold">{location.name}</h2>
			<p class="mt-0.5 truncate text-sm text-muted-foreground">
				{location.address}, {location.district}
			</p>
		</div>
		<div class="flex shrink-0 items-center gap-2">
			<button
				onclick={(e: MouseEvent) => { e.preventDefault(); e.stopPropagation(); favorites.toggle(location.id); }}
				class="p-1"
				aria-label={isFav ? 'Bo yeu thich' : 'Yeu thich'}
			>
				<svg class="h-5 w-5 {isFav ? 'fill-destructive text-destructive' : 'text-muted-foreground'}" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" fill={isFav ? 'currentColor' : 'none'}>
					<path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
				</svg>
			</button>
			<span class="rounded-full bg-primary/10 px-2.5 py-1 text-xs font-medium text-primary">
				{formatDistance(location.distance)}
			</span>
		</div>
	</div>
	<div class="mt-2.5 flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-muted-foreground">
		<span class="flex items-center gap-1">
			<svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
				<rect x="2" y="3" width="20" height="14" rx="2" />
				<path d="M2 10h20" />
			</svg>
			{location.bay_count} bay
		</span>
		{#if location.phone}
			<span class="flex items-center gap-1">
				<svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72c.127.96.361 1.903.7 2.81a2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45c.907.339 1.85.573 2.81.7A2 2 0 0 1 22 16.92z" />
				</svg>
				{location.phone}
			</span>
		{/if}
		<span class="rounded-full px-2 py-0.5 {location.status === 'active' ? 'bg-success/10 text-success' : 'bg-muted text-muted-foreground'}">
			{location.status === 'active' ? 'Dang mo' : 'Dong'}
		</span>
	</div>
</a>
