<script lang="ts">
	import { api } from '$lib/api/client';
	import { favorites } from '$lib/favorites.svelte';
	import type { NearbyLocation } from '$lib/api/types';
	import LocationCard from '$lib/components/LocationCard.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import Skeleton from '$lib/components/Skeleton.svelte';

	let locations: NearbyLocation[] = $state([]);
	let loading = $state(true);

	$effect(() => {
		loadFavorites();
	});

	async function loadFavorites() {
		loading = true;
		if (favorites.ids.length === 0) {
			locations = [];
			loading = false;
			return;
		}

		try {
			// Load nearby with large radius to get all, then filter by favorites
			if (navigator.geolocation) {
				navigator.geolocation.getCurrentPosition(
					async (pos) => {
						try {
							const all = await api.get<NearbyLocation[]>(
								`/locations/nearby?lat=${pos.coords.latitude}&lng=${pos.coords.longitude}&radius=100000`
							);
							locations = all.filter((l) => favorites.ids.includes(l.id));
						} catch {
							locations = [];
						}
						loading = false;
					},
					() => {
						loading = false;
					}
				);
			} else {
				loading = false;
			}
		} catch {
			loading = false;
		}
	}
</script>

<div class="pb-4">
	<h1 class="text-xl font-semibold">Yeu thich</h1>
	<p class="mt-1 text-sm text-muted-foreground">Cac tiem rua xe ban da luu</p>

	{#if loading}
		<div class="mt-4 space-y-3">
			{#each [1, 2] as _ (_)}
				<div class="rounded-xl bg-card p-4">
					<Skeleton class="h-5 w-2/3" />
					<Skeleton class="mt-2 h-4 w-1/2" />
				</div>
			{/each}
		</div>
	{:else if locations.length === 0}
		<div class="mt-6">
			<EmptyState
				icon="&#10084;&#65039;"
				title="Chua co tiem yeu thich"
				description="Nhan bieu tuong trai tim de luu tiem ban thich"
			>
				{#snippet action()}
					<a href="/" class="inline-block rounded-xl bg-primary px-5 py-2.5 text-sm font-medium text-primary-foreground">
						Tim tiem rua xe
					</a>
				{/snippet}
			</EmptyState>
		</div>
	{:else}
		<div class="mt-4 space-y-3">
			{#each locations as loc (loc.id)}
				<LocationCard location={loc} />
			{/each}
		</div>
	{/if}
</div>
