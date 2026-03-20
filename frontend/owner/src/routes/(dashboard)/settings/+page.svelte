<script lang="ts">
	import { api } from '$lib/api/client';
	import type { LocationResponse } from '$lib/api/types';

	let location: LocationResponse | null = $state(null);
	let saving = $state(false);
	let success = $state(false);

	// Form fields
	let name = $state('');
	let address = $state('');
	let district = $state('');
	let city = $state('');
	let phone = $state('');
	let bayCount = $state(1);
	let queueMode = $state('hybrid');

	$effect(() => {
		loadLocation();
	});

	async function loadLocation() {
		try {
			const locations = await api.get<LocationResponse[]>('/locations');
			if (locations.length > 0) {
				location = locations[0];
				name = location.name;
				address = location.address;
				district = location.district;
				city = location.city;
				phone = location.phone ?? '';
				bayCount = location.bay_count;
				queueMode = location.queue_mode;
			}
		} catch {
			// API not available
		}
	}

	async function saveSettings() {
		if (!location) return;
		saving = true;
		success = false;
		try {
			await api.put(`/locations/${location.id}`, {
				name,
				address,
				district,
				city,
				phone: phone || undefined,
				bay_count: bayCount,
				queue_mode: queueMode
			});
			success = true;
			setTimeout(() => (success = false), 3000);
		} catch (e: any) {
			alert(e.message);
		}
		saving = false;
	}

	async function createLocation() {
		saving = true;
		try {
			const created = await api.post<LocationResponse>('/locations', {
				name,
				address,
				district,
				city,
				latitude: 10.7769,
				longitude: 106.7009,
				phone: phone || undefined,
				bay_count: bayCount,
				queue_mode: queueMode
			});
			location = created;
			success = true;
			setTimeout(() => (success = false), 3000);
		} catch (e: any) {
			alert(e.message);
		}
		saving = false;
	}
</script>

<div>
	<h1 class="text-2xl font-semibold">Cai dat cua hang</h1>
	<p class="mt-1 text-sm text-muted-foreground">Quan ly thong tin va cau hinh.</p>

	{#if success}
		<div class="mt-4 rounded-md bg-success/10 p-3 text-sm text-success">Da luu thanh cong!</div>
	{/if}

	<div class="mt-6 max-w-2xl space-y-6">
		<div class="rounded-lg border border-border bg-card p-6">
			<h2 class="text-lg font-medium">Thong tin cua hang</h2>
			<div class="mt-4 space-y-4">
				<div>
					<label for="name" class="block text-sm font-medium">Ten cua hang</label>
					<input id="name" type="text" bind:value={name} class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" placeholder="VD: Sparkle Car Wash" />
				</div>
				<div>
					<label for="address" class="block text-sm font-medium">Dia chi</label>
					<input id="address" type="text" bind:value={address} class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
				</div>
				<div class="grid grid-cols-2 gap-4">
					<div>
						<label for="district" class="block text-sm font-medium">Quan/Huyen</label>
						<input id="district" type="text" bind:value={district} class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
					</div>
					<div>
						<label for="city" class="block text-sm font-medium">Thanh pho</label>
						<input id="city" type="text" bind:value={city} class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
					</div>
				</div>
				<div>
					<label for="phone" class="block text-sm font-medium">So dien thoai</label>
					<input id="phone" type="tel" bind:value={phone} class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
				</div>
			</div>
		</div>

		<div class="rounded-lg border border-border bg-card p-6">
			<h2 class="text-lg font-medium">Cau hinh</h2>
			<div class="mt-4 space-y-4">
				<div>
					<label for="bay_count" class="block text-sm font-medium">So luong bay rua xe</label>
					<input id="bay_count" type="number" min="1" max="20" bind:value={bayCount} class="mt-1 w-32 rounded-md border border-input bg-background px-3 py-2 text-sm" />
				</div>
				<div>
					<label for="queue_mode" class="block text-sm font-medium">Che do hang doi</label>
					<select id="queue_mode" bind:value={queueMode} class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm">
						<option value="hybrid">Ket hop (dat lich + walk-in)</option>
						<option value="walkin_only">Chi walk-in</option>
						<option value="booking_only">Chi dat lich</option>
					</select>
				</div>
			</div>
		</div>

		<button
			onclick={location ? saveSettings : createLocation}
			disabled={saving}
			class="rounded-md bg-primary px-6 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
		>
			{saving ? 'Dang luu...' : location ? 'Luu thay doi' : 'Tao cua hang'}
		</button>
	</div>
</div>
