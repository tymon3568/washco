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
	const dayNames = ['Chủ nhật', 'Thu 2', 'Thu 3', 'Thu 4', 'Thu 5', 'Thu 6', 'Thu 7'];

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
		active: { label: 'Hoạt động', classes: 'bg-green-500/15 text-green-400' },
		inactive: { label: 'Tạm ngưng', classes: 'bg-muted text-muted-foreground' },
		suspended: { label: 'Bị khóa', classes: 'bg-red-500/15 text-red-400' }
	};

	const queueModeLabels: Record<string, string> = {
		hybrid: 'Kết hợp (đặt lịch + walk-in)',
		walkin_only: 'Chỉ walk-in',
		booking_only: 'Chỉ đặt lịch'
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
			toast.success('Đã lưu giờ hoạt động!');
			setTimeout(() => (hoursSuccess = false), 3000);
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
		savingHours = false;
	}

	function validateLocation(): string | null {
		if (!name.trim()) return 'Vui lòng nhập tên cửa hàng';
		if (!address.trim()) return 'Vui lòng nhập địa chỉ';
		if (!district.trim()) return 'Vui lòng nhập quận/huyện';
		if (!city.trim()) return 'Vui lòng nhập thành phố';
		if (bayCount < 1 || bayCount > 50) return 'Số bay phải từ 1 đến 50';
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
			toast.success('Đã lưu thay đổi!');
			setTimeout(() => (success = false), 3000);
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
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
			toast.success('Đã tạo cửa hàng thành công!');
			success = true;
			setTimeout(() => (success = false), 3000);
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
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
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
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
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
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
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
	}

	function cancelEditBay() {
		editingBayId = null;
		editingBayName = '';
	}

	async function deleteBay(bay: BayResponse) {
		if (!confirm(`Xóa bay "${bay.name}"?`)) return;
		try {
			await api.del(`/locations/bays/${bay.id}`);
			bays = bays.filter((b) => b.id !== bay.id);
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
	}
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Cài đặt</h1>
			<p class="mt-1 text-sm text-muted-foreground">Quản lý thông tin doanh nghiệp và cấu hình cửa hàng.</p>
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
			Hồ sơ doanh nghiệp
		</button>
		<button
			onclick={() => (activeSection = 'location')}
			class="rounded-md px-4 py-2 text-sm font-medium transition-colors {activeSection === 'location'
				? 'bg-card text-foreground shadow-sm'
				: 'text-muted-foreground hover:text-foreground'}"
		>
			Cửa hàng
		</button>
		<button
			onclick={() => (activeSection = 'hours')}
			class="rounded-md px-4 py-2 text-sm font-medium transition-colors {activeSection === 'hours'
				? 'bg-card text-foreground shadow-sm'
				: 'text-muted-foreground hover:text-foreground'}"
		>
			Giờ hoạt động
		</button>
		<button
			onclick={() => (activeSection = 'bays')}
			class="rounded-md px-4 py-2 text-sm font-medium transition-colors {activeSection === 'bays'
				? 'bg-card text-foreground shadow-sm'
				: 'text-muted-foreground hover:text-foreground'}"
		>
			Bay rửa xe
		</button>
	</div>

	<div class="mt-6 max-w-2xl space-y-6">
		<!-- ===== SECTION 1: Business Profile ===== -->
		{#if activeSection === 'profile'}
			<div class="rounded-lg border border-border bg-card p-6">
				<h2 class="text-lg font-medium">Hồ sơ doanh nghiệp</h2>
				<p class="mt-1 text-sm text-muted-foreground">Thông tin tài khoản chủ cửa hàng.</p>

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
								<p class="text-sm text-muted-foreground">{profile.role === 'owner' ? 'Chủ cửa hàng' : profile.role}</p>
							</div>
						</div>

						<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
							<div class="rounded-md border border-border p-4">
								<p class="text-xs font-medium text-muted-foreground">Số điện thoại</p>
								<p class="mt-1 text-sm font-medium">{profile.phone}</p>
							</div>
							<div class="rounded-md border border-border p-4">
								<p class="text-xs font-medium text-muted-foreground">Trạng thái xác minh</p>
								<p class="mt-1 text-sm font-medium">
									{#if profile.is_verified}
										<span class="inline-flex items-center gap-1.5 text-green-400">
											<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
												<path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
											</svg>
											Đã xác minh
										</span>
									{:else}
										<span class="text-warning">Chưa xác minh</span>
									{/if}
								</p>
							</div>
							<div class="rounded-md border border-border p-4">
								<p class="text-xs font-medium text-muted-foreground">Mã tài khoản</p>
								<p class="mt-1 font-mono text-xs text-muted-foreground">{profile.id}</p>
							</div>
							<div class="rounded-md border border-border p-4">
								<p class="text-xs font-medium text-muted-foreground">Mã doanh nghiệp</p>
								<p class="mt-1 font-mono text-xs text-muted-foreground">{profile.tenant_id}</p>
							</div>
						</div>

						<div class="rounded-md border border-dashed border-border bg-muted/50 p-4">
							<p class="text-sm text-muted-foreground">
								Chức năng chỉnh sửa hồ sơ sẽ được cập nhật trong phiên bản tiếp theo.
							</p>
						</div>
					</div>
				{:else}
					<div class="mt-6 rounded-md bg-destructive/10 p-4 text-sm text-destructive">
						Không thể tải thông tin hồ sơ. Vui lòng thử lại.
					</div>
				{/if}
			</div>
		{/if}

		<!-- ===== SECTION 2: Location Settings ===== -->
		{#if activeSection === 'location'}
			{#if success}
				<div class="rounded-md bg-success/10 p-3 text-sm text-success">Đã lưu thành công!</div>
			{/if}

			<!-- Location info card -->
			{#if location}
			{@const statusInfo = statusLabels[location.status] ?? statusLabels['active']}
				<div class="rounded-lg border border-border bg-card p-6">
					<div class="flex items-center justify-between">
						<h2 class="text-lg font-medium">Thông tin cửa hàng</h2>
						<span class="rounded-full px-2.5 py-0.5 text-xs font-medium {statusInfo.classes}">
							{statusInfo.label}
						</span>
					</div>

					<div class="mt-4 space-y-4">
						<div>
							<label for="name" class="block text-sm font-medium">Tên cửa hàng</label>
							<input
								id="name"
								type="text"
								bind:value={name}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
								placeholder="VD: Sparkle Car Wash"
							/>
						</div>
						<div>
							<label for="address" class="block text-sm font-medium">Địa chỉ</label>
							<input
								id="address"
								type="text"
								bind:value={address}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							/>
						</div>
						<div class="grid grid-cols-2 gap-4">
							<div>
								<label for="district" class="block text-sm font-medium">Quận/Huyện</label>
								<input
									id="district"
									type="text"
									bind:value={district}
									class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
								/>
							</div>
							<div>
								<label for="city" class="block text-sm font-medium">Thành phố</label>
								<input
									id="city"
									type="text"
									bind:value={city}
									class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
								/>
							</div>
						</div>
						<div>
							<label for="phone" class="block text-sm font-medium">Số điện thoại</label>
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
					<h2 class="text-lg font-medium">Cấu hình</h2>
					<div class="mt-4 space-y-4">
						<div>
							<label for="bay_count" class="block text-sm font-medium">Số lượng bay rửa xe</label>
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
							<label for="queue_mode" class="block text-sm font-medium">Chế độ hàng đợi</label>
							<select
								id="queue_mode"
								bind:value={queueMode}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							>
								<option value="hybrid">Kết hợp (đặt lịch + walk-in)</option>
								<option value="walkin_only">Chỉ walk-in</option>
								<option value="booking_only">Chỉ đặt lịch</option>
							</select>
						</div>
					</div>
				</div>

				<button
					onclick={saveSettings}
					disabled={saving}
					class="rounded-md bg-primary px-6 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
				>
					{saving ? 'Đang lưu...' : 'Lưu thay đổi'}
				</button>
			{:else}
				<!-- No location yet - create form -->
				<div class="rounded-lg border border-border bg-card p-6">
					<h2 class="text-lg font-medium">Tạo cửa hàng</h2>
					<p class="mt-1 text-sm text-muted-foreground">Bạn chưa có cửa hàng nào. Điền thông tin bên dưới để bắt đầu.</p>

					<div class="mt-4 space-y-4">
						<div>
							<label for="name" class="block text-sm font-medium">Tên cửa hàng</label>
							<input
								id="name"
								type="text"
								bind:value={name}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
								placeholder="VD: Sparkle Car Wash"
							/>
						</div>
						<div>
							<label for="address" class="block text-sm font-medium">Địa chỉ</label>
							<input
								id="address"
								type="text"
								bind:value={address}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							/>
						</div>
						<div class="grid grid-cols-2 gap-4">
							<div>
								<label for="district" class="block text-sm font-medium">Quận/Huyện</label>
								<input
									id="district"
									type="text"
									bind:value={district}
									class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
								/>
							</div>
							<div>
								<label for="city" class="block text-sm font-medium">Thành phố</label>
								<input
									id="city"
									type="text"
									bind:value={city}
									class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
								/>
							</div>
						</div>
						<div>
							<label for="phone" class="block text-sm font-medium">Số điện thoại</label>
							<input
								id="phone"
								type="tel"
								bind:value={phone}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							/>
						</div>
						<div>
							<label for="bay_count" class="block text-sm font-medium">Số lượng bay rửa xe</label>
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
							<label for="queue_mode" class="block text-sm font-medium">Chế độ hàng đợi</label>
							<select
								id="queue_mode"
								bind:value={queueMode}
								class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							>
								<option value="hybrid">Kết hợp (đặt lịch + walk-in)</option>
								<option value="walkin_only">Chỉ walk-in</option>
								<option value="booking_only">Chỉ đặt lịch</option>
							</select>
						</div>
					</div>

					<button
						onclick={createLocation}
						disabled={saving}
						class="mt-4 rounded-md bg-primary px-6 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
					>
						{saving ? 'Đang tạo...' : 'Tạo cửa hàng'}
					</button>
				</div>
			{/if}
		{/if}

		<!-- ===== SECTION 3: Operating Hours ===== -->
		{#if activeSection === 'hours'}
			{#if location}
				<div class="rounded-lg border border-border bg-card p-6">
					<h2 class="text-lg font-medium">Giờ hoạt động</h2>
					<p class="mt-1 text-sm text-muted-foreground">Cài đặt giờ mở cửa và đóng cửa cho từng ngày trong tuần.</p>

					{#if hoursSuccess}
						<div class="mt-3 rounded-md bg-success/10 p-3 text-sm text-success">Đã lưu giờ hoạt động!</div>
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
									Đóng cửa
								</label>

								{#if !entry.is_closed}
									<div class="flex items-center gap-2">
										<label for="open-{i}" class="text-sm text-muted-foreground">Mở:</label>
										<input
											id="open-{i}"
											type="time"
											bind:value={hours[i].open_time}
											class="rounded-md border border-input bg-background px-2 py-1 text-sm"
										/>
									</div>
									<div class="flex items-center gap-2">
										<label for="close-{i}" class="text-sm text-muted-foreground">Đóng:</label>
										<input
											id="close-{i}"
											type="time"
											bind:value={hours[i].close_time}
											class="rounded-md border border-input bg-background px-2 py-1 text-sm"
										/>
									</div>
								{:else}
									<span class="text-sm text-muted-foreground">Nghỉ</span>
								{/if}
							</div>
						{/each}
					</div>

					<button
						onclick={saveOperatingHours}
						disabled={savingHours}
						class="mt-4 rounded-md bg-primary px-6 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
					>
						{savingHours ? 'Đang lưu...' : 'Lưu giờ hoạt động'}
					</button>
				</div>
			{:else}
				<div class="rounded-lg border border-border bg-card p-8 text-center text-sm text-muted-foreground">
					Vui lòng tạo cửa hàng trước khi cài đặt giờ hoạt động.
				</div>
			{/if}
		{/if}

		<!-- ===== SECTION 4: Bay Management ===== -->
		{#if activeSection === 'bays'}
			{#if location}
				<div class="rounded-lg border border-border bg-card p-6">
					<h2 class="text-lg font-medium">Quản lý bay rửa xe</h2>
					<p class="mt-1 text-sm text-muted-foreground">Thêm, sửa hoặc tắt bay rửa xe.</p>

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
										Lưu
									</button>
									<button
										onclick={cancelEditBay}
										class="rounded bg-muted px-3 py-1.5 text-xs"
									>
										Hủy
									</button>
								{:else}
									<span class="flex-1 text-sm font-medium" class:text-muted-foreground={!bay.is_active}>
										{bay.name}
										{#if !bay.is_active}
											<span class="ml-2 rounded-full bg-muted px-2 py-0.5 text-xs text-muted-foreground">Tắt</span>
										{/if}
									</span>
									<button
										onclick={() => startEditBay(bay)}
										class="rounded bg-muted px-3 py-1.5 text-xs hover:bg-muted/80"
									>
										Sửa
									</button>
									<button
										onclick={() => toggleBayActive(bay)}
										class="rounded px-3 py-1.5 text-xs {bay.is_active
											? 'bg-warning/10 text-warning'
											: 'bg-success/10 text-success'}"
									>
										{bay.is_active ? 'Tắt' : 'Bật'}
									</button>
									<button
										onclick={() => deleteBay(bay)}
										class="rounded bg-destructive/10 px-3 py-1.5 text-xs text-destructive hover:bg-destructive/20"
									>
										Xóa
									</button>
								{/if}
							</div>
						{/each}

						{#if bays.length === 0}
							<p class="py-4 text-center text-sm text-muted-foreground">Chưa có bay nào. Thêm bay mới bên dưới.</p>
						{/if}
					</div>

					<div class="mt-4 flex gap-2">
						<input
							type="text"
							bind:value={newBayName}
							placeholder="Tên bay mới (VD: Bay 1)"
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
							{addingBay ? 'Đang thêm...' : 'Thêm bay'}
						</button>
					</div>
				</div>
			{:else}
				<div class="rounded-lg border border-border bg-card p-8 text-center text-sm text-muted-foreground">
					Vui lòng tạo cửa hàng trước khi quản lý bay rửa xe.
				</div>
			{/if}
		{/if}
	</div>
</div>
