<script lang="ts">
	import { api } from '$lib/api/client';
	import { formatDistance } from '$lib/utils/format';
	import type { NearbyLocation } from '$lib/api/types';

	let locations: NearbyLocation[] = $state([]);
	let loading = $state(true);
	let error = $state('');
	let geoError = $state('');

	$effect(() => {
		findNearby();
	});

	async function findNearby() {
		loading = true;
		error = '';

		if (!navigator.geolocation) {
			geoError = 'Trình duyệt không hỗ trợ định vị.';
			loading = false;
			return;
		}

		navigator.geolocation.getCurrentPosition(
			async (pos) => {
				try {
					locations = await api.get<NearbyLocation[]>(
						`/locations/nearby?lat=${pos.coords.latitude}&lng=${pos.coords.longitude}&radius=10000`
					);
				} catch (e: any) {
					error = e.message || 'Không thể tải danh sách.';
				}
				loading = false;
			},
			() => {
				geoError = 'Vui lòng cho phép truy cập vị trí để tìm tiệm gần bạn.';
				loading = false;
			}
		);
	}
</script>

<div class="pb-20">
	<h1 class="text-xl font-semibold">Tiệm rửa xe gần bạn</h1>
	<p class="mt-1 text-sm text-muted-foreground">Tìm và xem hàng đợi theo thời gian thực.</p>

	{#if loading}
		<div class="mt-8 text-center text-sm text-muted-foreground">
			Đang tìm tiệm gần bạn...
		</div>
	{:else if geoError}
		<div class="mt-8 rounded-xl bg-card p-6 text-center">
			<p class="text-sm text-muted-foreground">{geoError}</p>
			<button
				onclick={findNearby}
				class="mt-4 rounded-lg bg-primary px-6 py-2.5 text-sm font-medium text-primary-foreground"
			>
				Thử lại
			</button>
		</div>
	{:else if error}
		<div class="mt-8 rounded-xl bg-destructive/10 p-4 text-sm text-destructive">{error}</div>
	{:else if locations.length === 0}
		<div class="mt-8 rounded-xl bg-card p-6 text-center">
			<p class="text-sm text-muted-foreground">Không tìm thấy tiệm rửa xe nào gần bạn.</p>
		</div>
	{:else}
		<div class="mt-4 space-y-3">
			{#each locations as loc (loc.id)}
				<a
					href="/location/{loc.id}"
					class="block rounded-xl border border-border bg-card p-4 transition-shadow hover:shadow-sm active:bg-muted"
				>
					<div class="flex items-start justify-between">
						<div>
							<h2 class="font-semibold">{loc.name}</h2>
							<p class="mt-0.5 text-sm text-muted-foreground">
								{loc.address}, {loc.district}
							</p>
						</div>
						<span class="shrink-0 rounded-full bg-primary/10 px-2.5 py-1 text-xs font-medium text-primary">
							{formatDistance(loc.distance)}
						</span>
					</div>
					<div class="mt-2 flex items-center gap-3 text-xs text-muted-foreground">
						<span>{loc.bay_count} bay</span>
						{#if loc.phone}
							<span>{loc.phone}</span>
						{/if}
						<span class="rounded-full px-2 py-0.5 {loc.status === 'active' ? 'bg-success/10 text-success' : 'bg-muted text-muted-foreground'}">
							{loc.status === 'active' ? 'Đang mở' : 'Đóng'}
						</span>
					</div>
				</a>
			{/each}
		</div>
	{/if}
</div>
