<script lang="ts">
	import { api } from '$lib/api/client';
	import type { BookingResponse } from '$lib/api/types';

	let phone = $state('');
	let bookings: BookingResponse[] = $state([]);
	let loading = $state(false);
	let error = $state('');
	let searched = $state(false);

	async function fetchHistory() {
		if (!phone.trim()) return;

		loading = true;
		error = '';
		searched = true;

		try {
			const cleanPhone = phone.trim().replace(/\s+/g, '');
			bookings = await api.get<BookingResponse[]>(`/bookings/phone/${encodeURIComponent(cleanPhone)}`);
		} catch (e: any) {
			error = e.message || 'Khong the tai lich su.';
			bookings = [];
		}
		loading = false;
	}

	function statusColor(status: string): string {
		switch (status) {
			case 'pending':
				return 'bg-yellow-100 text-yellow-700';
			case 'confirmed':
				return 'bg-blue-100 text-blue-700';
			case 'completed':
				return 'bg-green-100 text-green-700';
			case 'cancelled':
				return 'bg-red-100 text-red-700';
			default:
				return 'bg-muted text-muted-foreground';
		}
	}

	function statusLabel(status: string): string {
		switch (status) {
			case 'pending':
				return 'Cho xac nhan';
			case 'confirmed':
				return 'Da xac nhan';
			case 'completed':
				return 'Hoan thanh';
			case 'cancelled':
				return 'Da huy';
			default:
				return status;
		}
	}

	function formatDate(dateStr: string): string {
		const d = new Date(dateStr);
		return d.toLocaleDateString('vi-VN', { day: '2-digit', month: '2-digit', year: 'numeric' });
	}

	function formatTimeSlot(slot: string): string {
		return slot.slice(0, 5);
	}
</script>

<div class="pb-20">
	<h1 class="text-xl font-semibold">Lich su dat lich</h1>
	<p class="mt-1 text-sm text-muted-foreground">Nhap so dien thoai de xem lich su.</p>

	<form
		onsubmit={(e) => { e.preventDefault(); fetchHistory(); }}
		class="mt-4 flex gap-2"
	>
		<input
			bind:value={phone}
			type="tel"
			inputmode="tel"
			placeholder="So dien thoai"
			class="min-h-12 flex-1 rounded-lg border border-border bg-background px-4 text-sm"
		/>
		<button
			type="submit"
			disabled={loading || !phone.trim()}
			class="min-h-12 shrink-0 rounded-lg bg-primary px-5 text-sm font-medium text-primary-foreground disabled:opacity-50"
		>
			{loading ? 'Dang tim...' : 'Tim'}
		</button>
	</form>

	{#if error}
		<div class="mt-4 rounded-xl bg-destructive/10 p-4 text-sm text-destructive">{error}</div>
	{/if}

	{#if searched && !loading && !error}
		{#if bookings.length === 0}
			<div class="mt-6 rounded-xl bg-card p-6 text-center">
				<p class="text-sm text-muted-foreground">Khong tim thay lich su dat lich nao.</p>
			</div>
		{:else}
			<div class="mt-4 space-y-3">
				{#each bookings as booking (booking.id)}
					<div class="rounded-xl border border-border bg-card p-4">
						<div class="flex items-start justify-between">
							<div>
								<p class="text-sm font-semibold">{booking.location_name ?? 'Tiem rua xe'}</p>
								<p class="mt-0.5 text-xs text-muted-foreground">
									{booking.service_name ?? 'Dich vu'}
								</p>
							</div>
							<span class="shrink-0 rounded-full px-2.5 py-1 text-xs font-medium {statusColor(booking.status)}">
								{statusLabel(booking.status)}
							</span>
						</div>
						<div class="mt-3 flex items-center gap-3 text-xs text-muted-foreground">
							<span class="flex items-center gap-1">
								<svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
									<rect x="3" y="4" width="18" height="18" rx="2" />
									<path d="M16 2v4M8 2v4M3 10h18" />
								</svg>
								{formatDate(booking.booking_date)}
							</span>
							<span class="flex items-center gap-1">
								<svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
									<circle cx="12" cy="12" r="10" />
									<path d="M12 6v6l4 2" />
								</svg>
								{formatTimeSlot(booking.time_slot)}
							</span>
							<span class="capitalize">{booking.vehicle_type}</span>
						</div>
						{#if booking.notes}
							<p class="mt-2 text-xs text-muted-foreground italic">{booking.notes}</p>
						{/if}
					</div>
				{/each}
			</div>
		{/if}
	{/if}
</div>
