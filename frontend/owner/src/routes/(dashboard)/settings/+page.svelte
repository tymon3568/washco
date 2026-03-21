<script lang="ts">
	import { api, ApiError } from '$lib/api/client';
	import { auth } from '$lib/auth.svelte';
	import { toast } from '$lib/toast.svelte';
	import type {
		LocationResponse,
		OperatingHoursEntry,
		BayResponse,
		UserResponse
	} from '$lib/api/types';
	import { untrack } from 'svelte';

	// --- Business Profile ---
	let profile: UserResponse | null = $state(null);
	let profileLoading = $state(true);

	// --- Location ---
	let location: LocationResponse | null = $state(null);
	let saving = $state(false);
	let success = $state(false);

	// Location form fields
	let name = $state('');
	let address = $state('');
	let district = $state('');
	let city = $state('');
	let phone = $state('');
	let bayCount = $state(1);
	let queueMode = $state('hybrid');

	// --- Operating Hours ---
	const dayNames = ['Chu nhat', 'Thu 2', 'Thu 3', 'Thu 4', 'Thu 5', 'Thu 6', 'Thu 7'];

	function defaultHours(): OperatingHoursEntry[] {
		return Array.from({ length: 7 }, (_, i) => ({
			day_of_week: i,
			open_time: '08:00',
			close_time: '18:00',
			is_closed: false
		}));
	}

	let hours: OperatingHoursEntry[] = $state(defaultHours());
	let savingHours = $state(false);
	let hoursSuccess = $state(false);

	// --- Bay Management ---
	let bays: BayResponse[] = $state([]);
	let newBayName = $state('');
	let addingBay = $state(false);
	let editingBayId: string | null = $state(null);
	let editingBayName = $state('');

	// --- Active section for mobile-friendly tabs ---
	let activeSection = $state<'profile' | 'location' | 'hours' | 'bays'>('profile');

	const statusLabels: Record<string, { label: string; classes: string }> = {
		active: { label: 'Hoat dong', classes: 'bg-green-500/15 text-green-400' },
		inactive: { label: 'Tam ngung', classes: 'bg-muted text-muted-foreground' },
		suspended: { label: 'Bi khoa', classes: 'bg-red-500/15 text-red-400' }
	};

	const queueModeLabels: Record<string, string> = {
		hybrid: 'Ket hop (dat lich + walk-in)',
		walkin_only: 'Chi walk-in',
		booking_only: 'Chi dat lich'
	};

	// --- Load all data on mount ---
	$effect(() => {
		untrack(() => {
			loadProfile();
			loadLocation();
		});
	});

	async function loadProfile() {
		profileLoading = true;
		try {
			profile = await api.get<UserResponse>('/auth/me');
		} catch {
			// Use auth state as fallback
			profile = auth.user;
		}
		profileLoading = false;
	}

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
				await loadOperatingHours(location.id);
				await loadBays(location.id);
			}
		} catch {
			// API not available
		}
	}

	async function loadOperatingHours(locationId: string) {
		try {
			const data = await api.get<OperatingHoursEntry[]>(`/locations/${locationId}/hours`);
			if (data.length > 0) {
				const merged = defaultHours();
				for (const entry of data) {
					merged[entry.day_of_week] = {
						day_of_week: entry.day_of_week,
						open_time: entry.open_time,
						close_time: entry.close_time,
						is_closed: entry.is_closed
					};
				}
				hours = merged;
			}
		} catch {
			// Keep defaults
		}
	}

	async function saveOperatingHours() {
		if (!location) return;
		savingHours = true;
		hoursSuccess = false;
		try {
			await api.put(`/locations/${location.id}/hours`, { hours });
			hoursSuccess = true;
			toast.success('Da luu gio hoat dong!');
			setTimeout(() => (hoursSuccess = false), 3000);
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
		savingHours = false;
	}

	function validateLocation(): string | null {
		if (!name.trim()) return 'Vui long nhap ten cua hang';
		if (!address.trim()) return 'Vui long nhap dia chi';
		if (!district.trim()) return 'Vui long nhap quan/huyen';
		if (!city.trim()) return 'Vui long nhap thanh pho';
		if (bayCount < 1 || bayCount > 50) return 'So bay phai tu 1 den 50';
		return null;
	}

	async function saveSettings() {
		if (!location) return;
		const validationError = validateLocation();
		if (validationError) {
			toast.error(validationError);
			return;
		}
		saving = true;
		success = false;
		try {
			await api.put(`/locations/${location.id}`, {
				name: name.trim(),
				address: address.trim(),
				district: district.trim(),
				city: city.trim(),
				phone: phone.trim() || undefined,
				bay_count: bayCount,
				queue_mode: queueMode
			});
			success = true;
			toast.success('Da luu thay doi!');
			setTimeout(() => (success = false), 3000);
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
		saving = false;
	}

	async function createLocation() {
		const validationError = validateLocation();
		if (validationError) {
			toast.error(validationError);
			return;
		}
		saving = true;
		try {
			const created = await api.post<LocationResponse>('/locations', {
				name: name.trim(),
				address: address.trim(),
				district: district.trim(),
				city: city.trim(),
				latitude: 10.7769,
				longitude: 106.7009,
				phone: phone.trim() || undefined,
				bay_count: bayCount,
				queue_mode: queueMode
			});
			location = created;
			toast.success('Da tao cua hang thanh cong!');
			success = true;
			setTimeout(() => (success = false), 3000);
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
		saving = false;
	}

	async function loadBays(locationId: string) {
		try {
			bays = await api.get<BayResponse[]>(`/locations/${locationId}/bays`);
		} catch {
			// Keep empty
		}
	}

	async function addBay() {
		if (!location || !newBayName.trim()) return;
		addingBay = true;
		try {
			const bay = await api.post<BayResponse>(`/locations/${location.id}/bays`, {
				name: newBayName.trim()
			});
			bays = [...bays, bay];
			newBayName = '';
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
		addingBay = false;
	}

	async function toggleBayActive(bay: BayResponse) {
		try {
			const updated = await api.put<BayResponse>(`/locations/bays/${bay.id}`, {
				is_active: !bay.is_active
			});
			bays = bays.map((b) => (b.id === bay.id ? updated : b));
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
	}

	function startEditBay(bay: BayResponse) {
		editingBayId = bay.id;
		editingBayName = bay.name;
	}

	async function saveEditBay() {
		if (!editingBayId || !editingBayName.trim()) return;
		try {
			const updated = await api.put<BayResponse>(`/locations/bays/${editingBayId}`, {
				name: editingBayName.trim()
			});
			bays = bays.map((b) => (b.id === editingBayId ? updated : b));
			editingBayId = null;
			editingBayName = '';
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
	}

	function cancelEditBay() {
		editingBayId = null;
		editingBayName = '';
	}

	async function deleteBay(bay: BayResponse) {
		if (!confirm(`Xoa bay "${bay.name}"?`)) return;
		try {
			await api.del(`/locations/bays/${bay.id}`);
			bays = bays.filter((b) => b.id !== bay.id);
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
	}
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Cai dat</h1>
			<p class="mt-1 text-sm text-muted-foreground">Quan ly thong tin doanh nghiep va cau hinh cua hang.</p>
		</div>
	</div>

	<!-- Section tabs -->
	<div class="mt-6 flex gap-1 overflow-x-auto rounded-lg border border-border bg-muted p-1">
		<button
			onclick={() => (activeSection = 'profile')}
			class="rounded-md px-4 py-2 text-sm font-medium transition-colors {activeSection === 'profile'
				? 'bg-card text-foreground shadow-sm'
				: 'text-muted-foreground hover:text-foreground'}"
		>
			Ho so doanh nghiep
		</button>
		<button
			onclick={() => (activeSection = 'location')}
			class="rounded-md px-4 py-2 text-sm font-medium transition-colors {activeSection === 'location'
				? 'bg-card text-foreground shadow-sm'
				: 'text-muted-foreground hover:text-foreground'}"
		>
			Cua hang
		</button>
		<button
			onclick={() => (activeSection = 'hours')}
			class="rounded-md px-4 py-2 text-sm font-medium transition-colors {activeSection === 'hours'
				? 'bg-card text-foreground shadow-sm'
				: 'text-muted-foreground hover:text-foreground'}"
		>
			Gio hoat dong
		</button>
		<button
			onclick={() => (activeSection = 'bays')}
			class="rounded-md px-4 py-2 text-sm font-medium transition-colors {activeSection === 'bays'
				? 'bg-card text-foreground shadow-sm'
				: 'text-muted-foreground hover:text-foreground'}"
		>
			Bay rua xe
		</button>
	</div>

	<div class="mt-6 max-w-2xl space-y-6">
		<!-- ===== SECTION 1: Business Profile ===== -->
		{#if activeSection === 'profile'}
			<div class="rounded-lg border border-border bg-card p-6">
				<h2 class="text-lg font-medium">Ho so doanh nghiep</h2>
				<p class="mt-1 text-sm text-muted-foreground">Thong tin tai khoan chu cua hang.</p>

				{#if profileLoading}
					<div class="mt-6 flex items-center justify-center py-8">
						<div class="h-6 w-6 animate-spin rounded-full border-2 border-primary border-t-transparent"></div>
					</div>
				{:else if profile}
					<div class="mt-6 space-y-4">
						<div class="flex items-center gap-4">
							<div class="flex h-16 w-16 items-center justify-center rounded-full bg-primary/10 text-xl font-bold text-primary">
								{profile.name.charAt(0).toUpperCase()}
							</div>
							<div>
								<p class="text-lg font-medium">{profile.name}</p>
								<p class="text-sm text-muted-foreground">{profile.role === 'owner' ? 'Chu cua hang' : profile.role}</p>
							</div>
						</div>

						<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
							<div class="rounded-md border border-border p-4">
								<p class="text-xs font-medium text-muted-foreground">So dien thoai</p>
								<p class="mt-1 text-sm font-medium">{profile.phone}</p>
							</div>
							<div class="rounded-md border border-border p-4">
								<p class="text-xs font-medium text-muted-foreground">Trang thai xac minh</p>
								<p class="mt-1 text-sm font-medium">
									{#if profile.is_verified}
										<span class="inline-flex items-center gap-1.5 text-green-400">
											<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
												<path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
											</svg>
											Da xac minh
										</span>
									{:else}
										<span class="text-warning">Chua xac minh</span>
									{/if}
								</p>
							</div>
							<div class="rounded-md border border-border p-4">
								<p class="text-xs font-medium text-muted-foreground">Ma tai khoan</p>
								<p class="mt-1 font-mono text-xs text-muted-foreground">{profile.id}</p>
							</div>
							<div class="rounded-md border border-border p-4">
								<p class="text-xs font-medium text-muted-foreground">Ma doanh nghiep</p>
								<p class="mt-1 font-mono text-xs text-muted-foreground">{profile.tenant_id}</p>
							</div>
						</div>

						<div class="rounded-md border border-dashed border-border bg-muted/50 p-4">
							<p class="text-sm text-muted-foreground">
								Chuc nang chinh sua ho so se duoc cap nhat trong phien ban tiep theo.
							</p>
						</div>
					</div>
				{:else}
					<div class="mt-6 rounded-md bg-destructive/10 p-4 text-sm text-destructive">
						Khong the tai thong tin ho so. Vui long thu lai.
					</div>
				{/if}
			</div>
		{/if}

		<!-- ===== SECTION 2: Location Settings ===== -->
		{#if activeSection === 'location'}
			{#if success}
				<div class="rounded-md bg-success/10 p-3 text-sm text-success">Da luu thanh cong!</div>
			{/if}

			<!-- Location info card -->
			{#if location}
			{@const statusInfo = statusLabels[location.status] ?? statusLabels['active']}
				<div class="rounded-lg border border-border bg-card p-6">
					<div class="flex items-center justify-between">
						<h2 class="text-lg font-medium">Thong tin cua hang</h2>
						<span class="rounded-full px-2.5 py-0.5 text-xs font-medium {statusInfo.classes}">
							{statusInfo.label}
						</span>
					</div>

					<div class="mt-4 space-y-4">
						<div>
							<label for="name" class="block text-sm font-medium">Ten cua hang</label>
							<input
								id="name"
								type="text"
								bind:value={name}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
								placeholder="VD: Sparkle Car Wash"
							/>
						</div>
						<div>
							<label for="address" class="block text-sm font-medium">Dia chi</label>
							<input
								id="address"
								type="text"
								bind:value={address}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							/>
						</div>
						<div class="grid grid-cols-2 gap-4">
							<div>
								<label for="district" class="block text-sm font-medium">Quan/Huyen</label>
								<input
									id="district"
									type="text"
									bind:value={district}
									class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
								/>
							</div>
							<div>
								<label for="city" class="block text-sm font-medium">Thanh pho</label>
								<input
									id="city"
									type="text"
									bind:value={city}
									class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
								/>
							</div>
						</div>
						<div>
							<label for="phone" class="block text-sm font-medium">So dien thoai</label>
							<input
								id="phone"
								type="tel"
								bind:value={phone}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							/>
						</div>
					</div>
				</div>

				<div class="rounded-lg border border-border bg-card p-6">
					<h2 class="text-lg font-medium">Cau hinh</h2>
					<div class="mt-4 space-y-4">
						<div>
							<label for="bay_count" class="block text-sm font-medium">So luong bay rua xe</label>
							<input
								id="bay_count"
								type="number"
								min="1"
								max="20"
								bind:value={bayCount}
								class="mt-1 w-32 rounded-md border border-input bg-background px-3 py-2 text-sm"
							/>
						</div>
						<div>
							<label for="queue_mode" class="block text-sm font-medium">Che do hang doi</label>
							<select
								id="queue_mode"
								bind:value={queueMode}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							>
								<option value="hybrid">Ket hop (dat lich + walk-in)</option>
								<option value="walkin_only">Chi walk-in</option>
								<option value="booking_only">Chi dat lich</option>
							</select>
						</div>
					</div>
				</div>

				<button
					onclick={saveSettings}
					disabled={saving}
					class="rounded-md bg-primary px-6 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
				>
					{saving ? 'Dang luu...' : 'Luu thay doi'}
				</button>
			{:else}
				<!-- No location yet - create form -->
				<div class="rounded-lg border border-border bg-card p-6">
					<h2 class="text-lg font-medium">Tao cua hang</h2>
					<p class="mt-1 text-sm text-muted-foreground">Ban chua co cua hang nao. Dien thong tin ben duoi de bat dau.</p>

					<div class="mt-4 space-y-4">
						<div>
							<label for="name" class="block text-sm font-medium">Ten cua hang</label>
							<input
								id="name"
								type="text"
								bind:value={name}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
								placeholder="VD: Sparkle Car Wash"
							/>
						</div>
						<div>
							<label for="address" class="block text-sm font-medium">Dia chi</label>
							<input
								id="address"
								type="text"
								bind:value={address}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							/>
						</div>
						<div class="grid grid-cols-2 gap-4">
							<div>
								<label for="district" class="block text-sm font-medium">Quan/Huyen</label>
								<input
									id="district"
									type="text"
									bind:value={district}
									class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
								/>
							</div>
							<div>
								<label for="city" class="block text-sm font-medium">Thanh pho</label>
								<input
									id="city"
									type="text"
									bind:value={city}
									class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
								/>
							</div>
						</div>
						<div>
							<label for="phone" class="block text-sm font-medium">So dien thoai</label>
							<input
								id="phone"
								type="tel"
								bind:value={phone}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							/>
						</div>
						<div>
							<label for="bay_count" class="block text-sm font-medium">So luong bay rua xe</label>
							<input
								id="bay_count"
								type="number"
								min="1"
								max="20"
								bind:value={bayCount}
								class="mt-1 w-32 rounded-md border border-input bg-background px-3 py-2 text-sm"
							/>
						</div>
						<div>
							<label for="queue_mode" class="block text-sm font-medium">Che do hang doi</label>
							<select
								id="queue_mode"
								bind:value={queueMode}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							>
								<option value="hybrid">Ket hop (dat lich + walk-in)</option>
								<option value="walkin_only">Chi walk-in</option>
								<option value="booking_only">Chi dat lich</option>
							</select>
						</div>
					</div>

					<button
						onclick={createLocation}
						disabled={saving}
						class="mt-4 rounded-md bg-primary px-6 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
					>
						{saving ? 'Dang tao...' : 'Tao cua hang'}
					</button>
				</div>
			{/if}
		{/if}

		<!-- ===== SECTION 3: Operating Hours ===== -->
		{#if activeSection === 'hours'}
			{#if location}
				<div class="rounded-lg border border-border bg-card p-6">
					<h2 class="text-lg font-medium">Gio hoat dong</h2>
					<p class="mt-1 text-sm text-muted-foreground">Cai dat gio mo cua va dong cua cho tung ngay trong tuan.</p>

					{#if hoursSuccess}
						<div class="mt-3 rounded-md bg-success/10 p-3 text-sm text-success">Da luu gio hoat dong!</div>
					{/if}

					<div class="mt-4 space-y-3">
						{#each hours as entry, i (entry.day_of_week)}
							<div class="flex flex-wrap items-center gap-4 rounded-md border border-border p-3">
								<span class="w-20 text-sm font-medium">{dayNames[entry.day_of_week]}</span>

								<label class="flex items-center gap-2 text-sm">
									<input
										type="checkbox"
										checked={entry.is_closed}
										onchange={(e) => {
											hours[i].is_closed = (e.target as HTMLInputElement).checked;
										}}
										class="h-4 w-4 rounded border-input accent-primary"
									/>
									Dong cua
								</label>

								{#if !entry.is_closed}
									<div class="flex items-center gap-2">
										<label for="open-{i}" class="text-sm text-muted-foreground">Mo:</label>
										<input
											id="open-{i}"
											type="time"
											bind:value={hours[i].open_time}
											class="rounded-md border border-input bg-background px-2 py-1 text-sm"
										/>
									</div>
									<div class="flex items-center gap-2">
										<label for="close-{i}" class="text-sm text-muted-foreground">Dong:</label>
										<input
											id="close-{i}"
											type="time"
											bind:value={hours[i].close_time}
											class="rounded-md border border-input bg-background px-2 py-1 text-sm"
										/>
									</div>
								{:else}
									<span class="text-sm text-muted-foreground">Nghi</span>
								{/if}
							</div>
						{/each}
					</div>

					<button
						onclick={saveOperatingHours}
						disabled={savingHours}
						class="mt-4 rounded-md bg-primary px-6 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
					>
						{savingHours ? 'Dang luu...' : 'Luu gio hoat dong'}
					</button>
				</div>
			{:else}
				<div class="rounded-lg border border-border bg-card p-8 text-center text-sm text-muted-foreground">
					Vui long tao cua hang truoc khi cai dat gio hoat dong.
				</div>
			{/if}
		{/if}

		<!-- ===== SECTION 4: Bay Management ===== -->
		{#if activeSection === 'bays'}
			{#if location}
				<div class="rounded-lg border border-border bg-card p-6">
					<h2 class="text-lg font-medium">Quan ly bay rua xe</h2>
					<p class="mt-1 text-sm text-muted-foreground">Them, sua hoac tat bay rua xe.</p>

					<div class="mt-4 space-y-2">
						{#each bays as bay (bay.id)}
							<div class="flex items-center gap-3 rounded-md border border-border p-3">
								{#if editingBayId === bay.id}
									<input
										type="text"
										bind:value={editingBayName}
										class="flex-1 rounded-md border border-input bg-background px-3 py-1.5 text-sm"
										onkeydown={(e) => {
											if (e.key === 'Enter') saveEditBay();
											if (e.key === 'Escape') cancelEditBay();
										}}
									/>
									<button
										onclick={saveEditBay}
										class="rounded bg-primary px-3 py-1.5 text-xs text-primary-foreground"
									>
										Luu
									</button>
									<button
										onclick={cancelEditBay}
										class="rounded bg-muted px-3 py-1.5 text-xs"
									>
										Huy
									</button>
								{:else}
									<span class="flex-1 text-sm font-medium" class:text-muted-foreground={!bay.is_active}>
										{bay.name}
										{#if !bay.is_active}
											<span class="ml-2 rounded-full bg-muted px-2 py-0.5 text-xs text-muted-foreground">Tat</span>
										{/if}
									</span>
									<button
										onclick={() => startEditBay(bay)}
										class="rounded bg-muted px-3 py-1.5 text-xs hover:bg-muted/80"
									>
										Sua
									</button>
									<button
										onclick={() => toggleBayActive(bay)}
										class="rounded px-3 py-1.5 text-xs {bay.is_active
											? 'bg-warning/10 text-warning'
											: 'bg-success/10 text-success'}"
									>
										{bay.is_active ? 'Tat' : 'Bat'}
									</button>
									<button
										onclick={() => deleteBay(bay)}
										class="rounded bg-destructive/10 px-3 py-1.5 text-xs text-destructive hover:bg-destructive/20"
									>
										Xoa
									</button>
								{/if}
							</div>
						{/each}

						{#if bays.length === 0}
							<p class="py-4 text-center text-sm text-muted-foreground">Chua co bay nao. Them bay moi ben duoi.</p>
						{/if}
					</div>

					<div class="mt-4 flex gap-2">
						<input
							type="text"
							bind:value={newBayName}
							placeholder="Ten bay moi (VD: Bay 1)"
							class="flex-1 rounded-md border border-input bg-background px-3 py-2 text-sm"
							onkeydown={(e) => {
								if (e.key === 'Enter') addBay();
							}}
						/>
						<button
							onclick={addBay}
							disabled={addingBay || !newBayName.trim()}
							class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
						>
							{addingBay ? 'Dang them...' : 'Them bay'}
						</button>
					</div>
				</div>
			{:else}
				<div class="rounded-lg border border-border bg-card p-8 text-center text-sm text-muted-foreground">
					Vui long tao cua hang truoc khi quan ly bay rua xe.
				</div>
			{/if}
		{/if}
	</div>
</div>
