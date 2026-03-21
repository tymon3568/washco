<script lang="ts">
	import { api } from '$lib/api/client';
	import { formatDistance } from '$lib/utils/format';
	import type { NearbyLocation } from '$lib/api/types';
	import LocationCard from '$lib/components/LocationCard.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import Skeleton from '$lib/components/Skeleton.svelte';

	let locations: NearbyLocation[] = $state([]);
	let loading = $state(true);
	let error = $state('');
	let geoError = $state('');
	let searchQuery = $state('');
	let filterOpen = $state(false);
	let sortBy = $state<'distance' | 'name'>('distance');
	let showOpenOnly = $state(false);
	let searchRadius = $state(10000);
	let userCoords: { lat: number; lng: number } | null = $state(null);

	let filtered = $derived.by(() => {
		let result = locations;
		if (searchQuery.trim()) {
			const q = searchQuery.toLowerCase();
			result = result.filter(
				(l) =>
					l.name.toLowerCase().includes(q) ||
					l.address.toLowerCase().includes(q) ||
					l.district.toLowerCase().includes(q)
			);
		}
		if (showOpenOnly) {
			result = result.filter((l) => l.status === 'active');
		}
		if (sortBy === 'name') {
			result = [...result].sort((a, b) => a.name.localeCompare(b.name));
		}
		return result;
	});

	$effect(() => {
		findNearby();
	});

	async function findNearby() {
		loading = true;
		error = '';
		geoError = '';

		if (!navigator.geolocation) {
			geoError = 'Trinh duyet khong ho tro dinh vi.';
			loading = false;
			return;
		}

		navigator.geolocation.getCurrentPosition(
			async (pos) => {
				userCoords = { lat: pos.coords.latitude, lng: pos.coords.longitude };
				try {
					locations = await api.get<NearbyLocation[]>(
						`/locations/nearby?lat=${pos.coords.latitude}&lng=${pos.coords.longitude}&radius=${searchRadius}`
					);
				} catch (e: any) {
					error = e.message || 'Khong the tai danh sach.';
				}
				loading = false;
			},
			() => {
				geoError = 'Vui long cho phep truy cap vi tri de tim tiem gan ban.';
				loading = false;
			},
			{ enableHighAccuracy: true, timeout: 10000 }
		);
	}

	async function refresh() {
		if (!userCoords) {
			findNearby();
			return;
		}
		loading = true;
		error = '';
		try {
			locations = await api.get<NearbyLocation[]>(
				`/locations/nearby?lat=${userCoords.lat}&lng=${userCoords.lng}&radius=${searchRadius}`
			);
		} catch (e: any) {
			error = e.message || 'Khong the tai danh sach.';
		}
		loading = false;
	}
</script>

<div class="pb-4">
	<!-- Search bar -->
	<div class="relative">
		<svg class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
			<circle cx="11" cy="11" r="8" />
			<path d="m21 21-4.3-4.3" />
		</svg>
		<input
			bind:value={searchQuery}
			type="search"
			placeholder="Tim tiem rua xe..."
			class="min-h-12 w-full rounded-xl border border-border bg-card pl-10 pr-12 text-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
		/>
		<button
			onclick={() => { filterOpen = !filterOpen; }}
			class="absolute right-2 top-1/2 -translate-y-1/2 rounded-lg p-2 text-muted-foreground hover:bg-muted {filterOpen ? 'text-primary' : ''}"
			aria-label="Bo loc"
		>
			<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
				<path d="M22 3H2l8 9.46V19l4 2v-8.54L22 3z" />
			</svg>
		</button>
	</div>

	<!-- Filters -->
	{#if filterOpen}
		<div class="mt-2 rounded-xl border border-border bg-card p-4">
			<div class="flex flex-wrap gap-2">
				<button
					onclick={() => { showOpenOnly = !showOpenOnly; }}
					class="rounded-full border px-3 py-1.5 text-xs font-medium transition-colors {showOpenOnly ? 'border-primary bg-primary text-primary-foreground' : 'border-border text-muted-foreground'}"
				>
					Dang mo
				</button>
				<button
					onclick={() => { sortBy = sortBy === 'distance' ? 'name' : 'distance'; }}
					class="rounded-full border border-border px-3 py-1.5 text-xs font-medium text-muted-foreground"
				>
					Sap xep: {sortBy === 'distance' ? 'Khoang cach' : 'Ten'}
				</button>
			</div>
			<div class="mt-3">
				<label for="radius-slider" class="text-xs text-muted-foreground">Ban kinh: {searchRadius / 1000}km</label>
				<input
					id="radius-slider"
					bind:value={searchRadius}
					type="range"
					min="1000"
					max="50000"
					step="1000"
					onchange={refresh}
					class="mt-1 w-full accent-primary"
				/>
			</div>
		</div>
	{/if}

	<!-- Header -->
	<div class="mt-4 flex items-center justify-between">
		<div>
			<h1 class="text-xl font-semibold">Tiem rua xe gan ban</h1>
			<p class="text-sm text-muted-foreground">
				{#if !loading && !error && !geoError}
					{filtered.length} tiem trong ban kinh {searchRadius / 1000}km
				{:else}
					Tim va dat lich theo thoi gian thuc
				{/if}
			</p>
		</div>
		{#if !loading}
			<button
				onclick={refresh}
				class="rounded-lg p-2 text-muted-foreground hover:bg-muted"
				aria-label="Lam moi"
			>
				<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path d="M1 4v6h6M23 20v-6h-6" />
					<path d="M20.49 9A9 9 0 0 0 5.64 5.64L1 10m22 4-4.64 4.36A9 9 0 0 1 3.51 15" />
				</svg>
			</button>
		{/if}
	</div>

	<!-- Content -->
	{#if loading}
		<div class="mt-4 space-y-3">
			{#each [1, 2, 3] as _ (_)}
				<div class="rounded-xl border border-border bg-card p-4">
					<Skeleton class="h-5 w-3/4" />
					<Skeleton class="mt-2 h-4 w-1/2" />
					<Skeleton class="mt-3 h-3 w-2/3" />
				</div>
			{/each}
		</div>
	{:else if geoError}
		<div class="mt-6">
			<EmptyState
				icon="📍"
				title="Can quyen truy cap vi tri"
				description={geoError}
			>
				{#snippet action()}
					<button
						onclick={findNearby}
						class="rounded-xl bg-primary px-6 py-2.5 text-sm font-medium text-primary-foreground"
					>
						Thu lai
					</button>
				{/snippet}
			</EmptyState>
		</div>
	{:else if error}
		<div class="mt-6 rounded-xl bg-destructive/10 p-4 text-sm text-destructive">{error}</div>
	{:else if filtered.length === 0}
		<div class="mt-6">
			<EmptyState
				icon="🔍"
				title="Khong tim thay tiem nao"
				description={searchQuery ? 'Thu tu khoa khac' : 'Thu mo rong ban kinh tim kiem'}
			/>
		</div>
	{:else}
		<div class="mt-4 space-y-3">
			{#each filtered as loc (loc.id)}
				<LocationCard location={loc} />
			{/each}
		</div>
	{/if}
</div>
