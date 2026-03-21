<script lang="ts">
	import { api, ApiError } from '$lib/api/client';
	import { auth } from '$lib/auth.svelte';
	import { toast } from '$lib/toast.svelte';
	import { formatVND, formatDate, formatTimeSlot } from '$lib/utils/format';
	import type { BookingResponse } from '$lib/api/types';
	import StatusBadge from '$lib/components/StatusBadge.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import Skeleton from '$lib/components/Skeleton.svelte';

	let phone = $state('');
	let bookings: BookingResponse[] = $state([]);
	let loading = $state(false);
	let searched = $state(false);
	let activeFilter = $state<'all' | 'upcoming' | 'completed'>('all');

	// Auto-fill phone if logged in
	$effect(() => {
		if (auth.isAuthenticated && auth.user?.phone) {
			phone = auth.user.phone;
			fetchHistory();
		}
	});

	let filtered = $derived.by(() => {
		if (activeFilter === 'upcoming') {
			return bookings.filter((b) => ['pending', 'confirmed'].includes(b.status));
		}
		if (activeFilter === 'completed') {
			return bookings.filter((b) => b.status === 'completed');
		}
		return bookings;
	});

	async function fetchHistory() {
		if (!phone.trim()) return;
		loading = true;
		searched = true;
		try {
			const cleanPhone = phone.trim().replace(/\s+/g, '');
			bookings = await api.get<BookingResponse[]>(`/bookings/phone/${encodeURIComponent(cleanPhone)}`);
		} catch (e: any) {
			toast.error(e.message || 'Khong the tai lich su.');
			bookings = [];
		}
		loading = false;
	}

	async function cancelBooking(id: string) {
		try {
			await api.put(`/bookings/${id}/cancel`, {});
			toast.success('Da huy lich hen');
			await fetchHistory();
		} catch (e: any) {
			toast.error(e.message || 'Huy that bai');
		}
	}
</script>

<div class="pb-4">
	<h1 class="text-xl font-semibold">Lich su dat lich</h1>

	{#if !auth.isAuthenticated}
		<p class="mt-1 text-sm text-muted-foreground">Nhap so dien thoai de xem lich su.</p>
		<form onsubmit={(e) => { e.preventDefault(); fetchHistory(); }} class="mt-3 flex gap-2">
			<input
				bind:value={phone}
				type="tel"
				inputmode="tel"
				placeholder="So dien thoai"
				class="min-h-12 flex-1 rounded-xl border border-border bg-card px-4 text-sm focus:border-primary focus:outline-none"
			/>
			<button
				type="submit"
				disabled={loading || !phone.trim()}
				class="min-h-12 shrink-0 rounded-xl bg-primary px-5 text-sm font-medium text-primary-foreground disabled:opacity-50"
			>
				{loading ? 'Dang tim...' : 'Tim'}
			</button>
		</form>
	{/if}

	{#if loading}
		<div class="mt-4 space-y-3">
			{#each [1, 2, 3] as _ (_)}
				<div class="rounded-xl bg-card p-4">
					<Skeleton class="h-5 w-2/3" />
					<Skeleton class="mt-2 h-4 w-1/2" />
					<Skeleton class="mt-3 h-3 w-1/3" />
				</div>
			{/each}
		</div>
	{:else if searched}
		<!-- Filter tabs -->
		{#if bookings.length > 0}
			<div class="mt-4 flex gap-2">
				{#each [
					{ key: 'all', label: 'Tat ca' },
					{ key: 'upcoming', label: 'Sap toi' },
					{ key: 'completed', label: 'Da xong' }
				] as tab (tab.key)}
					<button
						onclick={() => { activeFilter = tab.key as typeof activeFilter; }}
						class="rounded-full border px-3 py-1.5 text-xs font-medium transition-colors {activeFilter === tab.key ? 'border-primary bg-primary text-primary-foreground' : 'border-border text-muted-foreground'}"
					>
						{tab.label}
					</button>
				{/each}
			</div>
		{/if}

		{#if filtered.length === 0}
			<div class="mt-6">
				<EmptyState
					icon="📋"
					title="Khong co lich hen nao"
					description={bookings.length > 0 ? 'Khong co lich hen phu hop voi bo loc' : 'Ban chua co lich hen nao. Dat lich ngay!'}
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
				{#each filtered as booking (booking.id)}
					<div class="rounded-xl border border-border bg-card p-4 shadow-xs">
						<div class="flex items-start justify-between">
							<div class="min-w-0 flex-1">
								<p class="truncate font-semibold">{booking.location_name ?? 'Tiem rua xe'}</p>
								<p class="mt-0.5 text-sm text-muted-foreground">{booking.service_name ?? 'Dich vu'}</p>
							</div>
							<StatusBadge status={booking.status} />
						</div>
						<div class="mt-3 flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-muted-foreground">
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
							<span class="capitalize">{booking.vehicle_type === 'motorcycle' ? 'Xe may' : booking.vehicle_type === 'car' ? 'Xe hoi' : booking.vehicle_type}</span>
							{#if booking.estimated_price}
								<span class="font-medium text-primary">{formatVND(booking.estimated_price)}</span>
							{/if}
						</div>
						{#if booking.notes}
							<p class="mt-2 text-xs italic text-muted-foreground">{booking.notes}</p>
						{/if}

						<!-- Actions -->
						{#if booking.status === 'pending' || booking.status === 'confirmed'}
							<div class="mt-3 flex gap-2">
								<a
									href="/location/{booking.location_id}"
									class="flex-1 rounded-lg border border-border py-2 text-center text-xs font-medium text-muted-foreground"
								>
									Xem tiem
								</a>
								<button
									onclick={() => cancelBooking(booking.id)}
									class="rounded-lg border border-destructive/30 px-4 py-2 text-xs font-medium text-destructive"
								>
									Huy
								</button>
							</div>
						{:else if booking.status === 'completed'}
							<div class="mt-3">
								<a
									href="/location/{booking.location_id}/review"
									class="block rounded-lg border border-border py-2 text-center text-xs font-medium text-primary"
								>
									Danh gia
								</a>
							</div>
						{/if}
					</div>
				{/each}
			</div>
		{/if}
	{/if}
</div>
