<script lang="ts">
	import { onMount, untrack } from 'svelte';
	import type { NearbyLocation } from '$lib/api/types';
	import { formatDistance } from '$lib/utils/format';

	let {
		locations,
		userCoords,
		onSelectLocation
	}: {
		locations: NearbyLocation[];
		userCoords: { lat: number; lng: number } | null;
		onSelectLocation?: (id: string) => void;
	} = $props();

	let mapEl: HTMLDivElement | undefined = $state();
	let map: any = null;
	let currentMarkers: any[] = [];
	let L: any;

	onMount(() => {
		import('leaflet').then((leaflet) => {
			L = leaflet.default ?? leaflet;

			const center = userCoords
				? [userCoords.lat, userCoords.lng]
				: [10.7769, 106.7009];

			map = L.map(mapEl!, {
				zoomControl: false
			}).setView(center as [number, number], 13);

			L.control.zoom({ position: 'bottomright' }).addTo(map);

			L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
				attribution: '&copy; OpenStreetMap',
				maxZoom: 18
			}).addTo(map);

			// Fix for flex layouts where container size changes after mount
			setTimeout(() => map?.invalidateSize(), 200);

			if (userCoords) {
				const userIcon = L.divIcon({
					html: '<div style="width:14px;height:14px;background:#3b82f6;border:3px solid white;border-radius:50%;box-shadow:0 0 6px rgba(59,130,246,0.5);"></div>',
					iconSize: [14, 14],
					iconAnchor: [7, 7],
					className: ''
				});
				L.marker([userCoords.lat, userCoords.lng], { icon: userIcon })
					.addTo(map)
					.bindPopup('Vi tri cua ban');
			}

			addMarkers(locations);
		});

		return () => {
			if (map) {
				map.remove();
				map = null;
			}
		};
	});

	// React to locations changes only
	$effect(() => {
		const locs = locations;
		untrack(() => {
			if (map && L) {
				addMarkers(locs);
			}
		});
	});

	function addMarkers(locs: NearbyLocation[]) {
		if (!map || !L) return;

		// Clear old markers
		for (const m of currentMarkers) {
			map.removeLayer(m);
		}
		currentMarkers = [];

		const washIcon = L.divIcon({
			html: '<div style="width:32px;height:32px;background:#2563eb;border:2px solid white;border-radius:50%;display:flex;align-items:center;justify-content:center;box-shadow:0 2px 8px rgba(0,0,0,0.3);font-size:16px;">🚿</div>',
			iconSize: [32, 32],
			iconAnchor: [16, 16],
			popupAnchor: [0, -18],
			className: ''
		});

		const bounds: [number, number][] = [];

		if (userCoords) {
			bounds.push([userCoords.lat, userCoords.lng]);
		}

		for (const loc of locs) {
			const lat = loc.latitude;
			const lng = loc.longitude;
			if (!lat || !lng) continue;

			bounds.push([lat, lng]);

			const popup = `
				<div style="min-width:160px;font-family:system-ui,sans-serif;">
					<strong style="font-size:13px;">${loc.name}</strong>
					<p style="margin:4px 0 2px;font-size:11px;color:#666;">${loc.address}, ${loc.district}</p>
					<div style="display:flex;gap:8px;margin-top:4px;font-size:11px;">
						<span>${loc.bay_count} bay</span>
						<span style="color:#2563eb;font-weight:600;">${formatDistance(loc.distance_meters)}</span>
					</div>
					<button onclick="window.__selectLocation__('${loc.id}')"
						style="margin-top:8px;width:100%;padding:6px;background:#2563eb;color:white;border:none;border-radius:8px;font-size:12px;font-weight:600;cursor:pointer;">
						Xem chi tiet
					</button>
				</div>
			`;

			const marker = L.marker([lat, lng], { icon: washIcon })
				.addTo(map)
				.bindPopup(popup);

			currentMarkers.push(marker);
		}

		if (bounds.length > 1) {
			map.fitBounds(bounds, { padding: [40, 40], maxZoom: 14 });
		}
	}

	// Global handler for popup button clicks
	if (typeof window !== 'undefined') {
		(window as any).__selectLocation__ = (id: string) => {
			onSelectLocation?.(id);
		};
	}
</script>

<svelte:head>
	<link
		rel="stylesheet"
		href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
		crossorigin=""
	/>
</svelte:head>

<div
	bind:this={mapEl}
	class="h-full w-full rounded-xl"
	style="min-height: 280px;"
></div>
