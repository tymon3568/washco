<script lang="ts">
	import { goto } from '$app/navigation';
	import { api } from '$lib/api/client';
	import { formatDistance } from '$lib/utils/format';
	import type { NearbyLocation } from '$lib/api/types';
	import LocationCard from '$lib/components/LocationCard.svelte';
	import LocationMap from '$lib/components/LocationMap.svelte';
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
	let searchRadius = $state(50000);
	let userCoords: { lat: number; lng: number } | null = $state(null);
	let viewMode = $state<'list' | 'map'>('map');

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

	import { onMount } from 'svelte';
	onMount(() => {
		findNearby();
	});

	const HCM_CENTER = { lat: 10.7769, lng: 106.7009 };

	function haversineMeters(lat1: number, lng1: number, lat2: number, lng2: number): number {
		const R = 6371000;
		const toRad = (d: number) => (d * Math.PI) / 180;
		const dLat = toRad(lat2 - lat1);
		const dLng = toRad(lng2 - lng1);
		const a =
			Math.sin(dLat / 2) ** 2 +
			Math.cos(toRad(lat1)) * Math.cos(toRad(lat2)) * Math.sin(dLng / 2) ** 2;
		return R * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
	}

	function recalcDistances(locs: NearbyLocation[], fromLat: number, fromLng: number): NearbyLocation[] {
		return locs
			.map((l) => ({
				...l,
				distance_meters: haversineMeters(fromLat, fromLng, l.latitude, l.longitude)
			}))
			.sort((a, b) => a.distance_meters - b.distance_meters);
	}

	async function fetchLocations(lat: number, lng: number, radius: number): Promise<NearbyLocation[]> {
		return api.get<NearbyLocation[]>(`/locations/nearby?lat=${lat}&lng=${lng}&radius=${radius}`);
	}

	async function findNearby() {
		loading = true;
		error = '';
		geoError = '';

		if (!navigator.geolocation) {
			userCoords = HCM_CENTER;
			try {
				locations = await fetchLocations(HCM_CENTER.lat, HCM_CENTER.lng, 50000);
			} catch (e: any) {
				error = e.message || 'Khong the tai danh sach.';
			}
			loading = false;
			return;
		}

		navigator.geolocation.getCurrentPosition(
			async (pos) => {
				const realLat = pos.coords.latitude;
				const realLng = pos.coords.longitude;
				userCoords = { lat: realLat, lng: realLng };
				try {
					let result = await fetchLocations(realLat, realLng, searchRadius);
					// If no locations near user, widen search to 200km
					if (result.length === 0) {
						result = await fetchLocations(realLat, realLng, 200000);
					}
					// If still no locations (user far from VN), fallback to HCM center search
					// but recalculate distances from user's real position
					if (result.length === 0) {
						result = await fetchLocations(HCM_CENTER.lat, HCM_CENTER.lng, 50000);
						result = recalcDistances(result, realLat, realLng);
						geoError = 'Hien thi tiem tai TP.HCM (vi tri cua ban o xa)';
					}
					locations = result;
				} catch (e: any) {
					error = e.message || 'Khong the tai danh sach.';
				}
				loading = false;
			},
			async () => {
				userCoords = HCM_CENTER;
				try {
					let result = await fetchLocations(HCM_CENTER.lat, HCM_CENTER.lng, 50000);
					// Widen if needed
					if (result.length === 0) {
						result = await fetchLocations(HCM_CENTER.lat, HCM_CENTER.lng, 200000);
					}
					locations = result;
				} catch (e: any) {
					error = e.message || 'Khong the tai danh sach.';
				}
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
			let result = await fetchLocations(userCoords.lat, userCoords.lng, searchRadius);
			if (result.length === 0) {
				result = await fetchLocations(userCoords.lat, userCoords.lng, 200000);
			}
			if (result.length === 0) {
				userCoords = HCM_CENTER;
				result = await fetchLocations(HCM_CENTER.lat, HCM_CENTER.lng, 50000);
			}
			locations = result;
		} catch (e: any) {
			error = e.message || 'Khong the tai danh sach.';
		}
		loading = false;
	}
</script>

<div class="flex flex-col" style="height: calc(100dvh - 10rem);">
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
		<div class="flex items-center gap-1">
			<!-- View toggle -->
			<div class="flex rounded-lg border border-border">
				<button
					onclick={() => { viewMode = 'list'; }}
					class="rounded-l-lg p-2 {viewMode === 'list' ? 'bg-primary text-primary-foreground' : 'text-muted-foreground hover:bg-muted'}"
					aria-label="Danh sach"
				>
					<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path d="M8 6h13M8 12h13M8 18h13M3 6h.01M3 12h.01M3 18h.01" />
					</svg>
				</button>
				<button
					onclick={() => { viewMode = 'map'; }}
					class="rounded-r-lg p-2 {viewMode === 'map' ? 'bg-primary text-primary-foreground' : 'text-muted-foreground hover:bg-muted'}"
					aria-label="Ban do"
				>
					<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path d="M1 6v16l7-4 8 4 7-4V2l-7 4-8-4-7 4z" />
						<path d="M8 2v16M16 6v16" />
					</svg>
				</button>
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
	{:else if error}
		<div class="mt-6 rounded-xl bg-destructive/10 p-4 text-sm text-destructive">{error}</div>
	{:else if viewMode === 'map'}
		<div class="mt-4 flex min-h-0 flex-1 flex-col">
			<div class="min-h-0 flex-1 overflow-hidden rounded-xl border border-border" style="isolation: isolate;">
				{#key viewMode}
					<LocationMap
						locations={filtered}
						{userCoords}
						onSelectLocation={(id) => goto(`/location/${id}`)}
					/>
				{/key}
			</div>
			{#if filtered.length > 0}
				<div class="mt-3 shrink-0 space-y-2 overflow-y-auto" style="max-height: 30vh;">
					{#each filtered.slice(0, 5) as loc (loc.id)}
						<a href="/location/{loc.id}" class="relative z-10 flex cursor-pointer items-center gap-3 rounded-xl border border-border bg-card p-3 hover:shadow-sm active:bg-muted">
							<div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-primary/10 text-lg">🚿</div>
							<div class="min-w-0 flex-1">
								<p class="truncate text-sm font-semibold">{loc.name}</p>
								<p class="truncate text-xs text-muted-foreground">{loc.address}, {loc.district}</p>
							</div>
							<span class="shrink-0 text-xs font-medium text-primary">{formatDistance(loc.distance_meters)}</span>
						</a>
					{/each}
					{#if filtered.length > 5}
						<p class="text-center text-xs text-muted-foreground">
							+{filtered.length - 5} tiem khac tren ban do
						</p>
					{/if}
				</div>
			{/if}
		</div>
	{:else if filtered.length === 0}
		<div class="mt-6">
			<EmptyState
				icon="🔍"
				title="Khong tim thay tiem nao"
				description={searchQuery ? 'Thu tu khoa khac' : 'Thu mo rong ban kinh tim kiem'}
			/>
		</div>
	{:else}
		<div class="mt-4 space-y-3 overflow-y-auto">
			{#each filtered as loc (loc.id)}
				<LocationCard location={loc} />
			{/each}
		</div>
	{/if}
</div>
