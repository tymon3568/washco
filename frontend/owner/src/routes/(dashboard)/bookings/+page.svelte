<script lang="ts">
	import { api, ApiError } from '$lib/api/client';
	import { toast } from '$lib/toast.svelte';

	interface Location {
		id: string;
		name: string;
	}

	interface Booking {
		id: string;
		location_id: string;
		service_id: string;
		customer_name: string;
		customer_phone: string;
		vehicle_type: string;
		booking_date: string;
		time_slot: string;
		status: string;
		notes: string | null;
		created_at: string;
	}

	let locations: Location[] = $state([]);
	let selectedLocationId = $state('');
	let selectedDate = $state(todayString());
	let bookings: Booking[] = $state([]);
	let loading = $state(false);
	let actionLoading: string | null = $state(null);

	function todayString(): string {
		const d = new Date();
		return d.toISOString().split('T')[0];
	}

	let sortedBookings = $derived(
		[...bookings].sort((a, b) => a.time_slot.localeCompare(b.time_slot))
	);

	let stats = $derived({
		pending: bookings.filter((b) => b.status === 'pending').length,
		confirmed: bookings.filter((b) => b.status === 'confirmed').length,
		completed: bookings.filter((b) => b.status === 'completed').length,
		cancelled: bookings.filter((b) => b.status === 'cancelled').length
	});

	$effect(() => {
		loadLocations();
	});

	$effect(() => {
		if (selectedLocationId && selectedDate) {
			loadBookings();
		}
	});

	async function loadLocations() {
		try {
			locations = await api.get<Location[]>('/locations');
			if (locations.length > 0) {
				selectedLocationId = locations[0].id;
			}
		} catch {
			// API not available
		}
	}

	async function loadBookings() {
		if (!selectedLocationId || !selectedDate) return;
		loading = true;
		try {
			bookings = await api.get<Booking[]>(
				`/bookings/locations/${selectedLocationId}?date=${selectedDate}`
			);
		} catch {
			bookings = [];
		}
		loading = false;
	}

	async function confirmBooking(id: string) {
		actionLoading = id;
		try {
			await api.put(`/bookings/${id}/confirm`, {});
			await loadBookings();
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
		actionLoading = null;
	}

	async function completeBooking(id: string) {
		actionLoading = id;
		try {
			await api.put(`/bookings/${id}/complete`, {});
			await loadBookings();
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
		actionLoading = null;
	}

	async function cancelBooking(id: string) {
		actionLoading = id;
		try {
			await api.put(`/bookings/${id}/cancel`, {});
			await loadBookings();
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
		actionLoading = null;
	}

	function statusBadgeClass(status: string): string {
		switch (status) {
			case 'pending':
				return 'bg-yellow-500/10 text-yellow-500';
			case 'confirmed':
				return 'bg-blue-500/10 text-blue-500';
			case 'completed':
				return 'bg-green-500/10 text-green-500';
			case 'cancelled':
				return 'bg-red-500/10 text-red-500';
			default:
				return 'bg-muted text-muted-foreground';
		}
	}

	function statusLabel(status: string): string {
		switch (status) {
			case 'pending':
				return 'Chờ xác nhận';
			case 'confirmed':
				return 'Đã xác nhận';
			case 'completed':
				return 'Hoàn thành';
			case 'cancelled':
				return 'Đã hủy';
			default:
				return status;
		}
	}

	function vehicleLabel(type: string): string {
		switch (type) {
			case 'motorbike':
				return 'Xe máy';
			case 'sedan':
				return 'Sedan';
			case 'suv':
				return 'SUV';
			case 'truck':
				return 'Xe tải';
			case 'van':
				return 'Van';
			default:
				return type;
		}
	}

	function isDimmed(status: string): boolean {
		return status === 'completed' || status === 'cancelled';
	}
</script>

<div>
	<!-- Header -->
	<div>
		<h1 class="text-2xl font-semibold">Đặt lịch</h1>
		<p class="mt-1 text-sm text-muted-foreground">Quản lý lịch hẹn rửa xe.</p>
	</div>

	<!-- Filters -->
	<div class="mt-4 flex flex-col gap-3 sm:flex-row sm:items-center">
		<div class="flex items-center gap-2">
			<label for="date-picker" class="text-sm font-medium text-muted-foreground">Ngày</label>
			<input
				id="date-picker"
				type="date"
				bind:value={selectedDate}
				class="rounded-md border border-input bg-background px-3 py-2 text-sm"
			/>
		</div>
		{#if locations.length > 1}
			<div class="flex items-center gap-2">
				<label for="location-select" class="text-sm font-medium text-muted-foreground">Chi nhánh</label>
				<select
					id="location-select"
					bind:value={selectedLocationId}
					class="rounded-md border border-input bg-background px-3 py-2 text-sm"
				>
					{#each locations as loc (loc.id)}
						<option value={loc.id}>{loc.name}</option>
					{/each}
				</select>
			</div>
		{/if}
	</div>

	<!-- Stats summary -->
	<div class="mt-4 grid grid-cols-2 gap-3 sm:grid-cols-4">
		<div class="rounded-lg border border-border bg-card p-3 text-center">
			<p class="text-2xl font-bold text-yellow-500">{stats.pending}</p>
			<p class="text-xs text-muted-foreground">Chờ xác nhận</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-3 text-center">
			<p class="text-2xl font-bold text-blue-500">{stats.confirmed}</p>
			<p class="text-xs text-muted-foreground">Đã xác nhận</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-3 text-center">
			<p class="text-2xl font-bold text-green-500">{stats.completed}</p>
			<p class="text-xs text-muted-foreground">Hoàn thành</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-3 text-center">
			<p class="text-2xl font-bold text-red-500">{stats.cancelled}</p>
			<p class="text-xs text-muted-foreground">Đã hủy</p>
		</div>
	</div>

	<!-- Bookings list -->
	<div class="mt-6 space-y-3">
		{#if loading}
			<div class="flex items-center justify-center py-12">
				<p class="text-sm text-muted-foreground">Đang tải...</p>
			</div>
		{:else if sortedBookings.length === 0}
			<div class="flex flex-col items-center justify-center rounded-lg border border-border bg-card py-16">
				<svg class="h-12 w-12 text-muted-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
					<path stroke-linecap="round" stroke-linejoin="round" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
				</svg>
				<p class="mt-3 text-sm font-medium text-muted-foreground">Không có lịch hẹn</p>
				<p class="mt-1 text-xs text-muted-foreground">Không có lịch hẹn nào cho ngày này.</p>
			</div>
		{:else}
			{#each sortedBookings as booking (booking.id)}
				<div
					class="rounded-lg border border-border bg-card p-4 transition-opacity {isDimmed(booking.status) ? 'opacity-50' : ''}"
				>
					<div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
						<!-- Booking info -->
						<div class="flex-1 space-y-1">
							<div class="flex items-center gap-2">
								<span class="text-sm font-bold">{booking.customer_name}</span>
								<span class="rounded-full px-2 py-0.5 text-xs font-medium {statusBadgeClass(booking.status)}">
									{statusLabel(booking.status)}
								</span>
							</div>
							<div class="flex flex-wrap items-center gap-x-4 gap-y-1 text-xs text-muted-foreground">
								<span class="flex items-center gap-1">
									<svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
										<path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
									</svg>
									{booking.time_slot}
								</span>
								<span class="flex items-center gap-1">
									<svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
										<path stroke-linecap="round" stroke-linejoin="round" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
									</svg>
									{booking.customer_phone}
								</span>
								<span>{vehicleLabel(booking.vehicle_type)}</span>
							</div>
							{#if booking.notes}
								<p class="text-xs text-muted-foreground italic">Ghi chú: {booking.notes}</p>
							{/if}
						</div>

						<!-- Actions -->
						{#if booking.status === 'pending'}
							<div class="flex gap-2">
								<button
									onclick={() => confirmBooking(booking.id)}
									disabled={actionLoading === booking.id}
									class="rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
								>
									Xác nhận
								</button>
								<button
									onclick={() => cancelBooking(booking.id)}
									disabled={actionLoading === booking.id}
									class="rounded-md bg-destructive/10 px-3 py-1.5 text-xs font-medium text-destructive hover:bg-destructive/20 disabled:opacity-50"
								>
									Hủy
								</button>
							</div>
						{:else if booking.status === 'confirmed'}
							<div class="flex gap-2">
								<button
									onclick={() => completeBooking(booking.id)}
									disabled={actionLoading === booking.id}
									class="rounded-md bg-green-600 px-3 py-1.5 text-xs font-medium text-white hover:bg-green-700 disabled:opacity-50"
								>
									Hoàn thành
								</button>
								<button
									onclick={() => cancelBooking(booking.id)}
									disabled={actionLoading === booking.id}
									class="rounded-md bg-destructive/10 px-3 py-1.5 text-xs font-medium text-destructive hover:bg-destructive/20 disabled:opacity-50"
								>
									Hủy
								</button>
							</div>
						{/if}
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>
