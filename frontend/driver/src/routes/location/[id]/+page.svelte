<script lang="ts">
	import { page } from '$app/state';
	import { api } from '$lib/api/client';
	import { formatVND } from '$lib/utils/format';
	import { createQueueSocket } from '$lib/api/ws';
	import type { QueueStateResponse, ServiceResponse } from '$lib/api/types';

	let locationId = $derived(page.params.id!);
	let queue: QueueStateResponse | null = $state(null);
	let services: ServiceResponse[] = $state([]);
	let loading = $state(true);
	let wsConnection: { close: () => void } | null = null;

	// Booking form state
	let showBookingForm = $state(false);
	let bookingName = $state('');
	let bookingPhone = $state('');
	let bookingVehicle = $state('car');
	let bookingServiceId = $state('');
	let bookingDate = $state(new Date().toISOString().split('T')[0]);
	let bookingTime = $state('09:00');
	let bookingNotes = $state('');
	let bookingSubmitting = $state(false);
	let bookingSuccess = $state(false);
	let bookingError = $state('');

	$effect(() => {
		loadData();
		return () => {
			wsConnection?.close();
		};
	});

	async function loadData() {
		loading = true;
		try {
			const [q, s] = await Promise.all([
				api.get<QueueStateResponse>(`/queue/locations/${locationId}`),
				api.get<ServiceResponse[]>(`/catalog/locations/${locationId}/services`)
			]);
			queue = q;
			services = s;

			wsConnection?.close();
			wsConnection = createQueueSocket(locationId, refreshQueue);
		} catch {
			// ignore
		}
		loading = false;
	}

	async function submitBooking() {
		bookingSubmitting = true;
		bookingError = '';
		try {
			await api.post('/bookings', {
				location_id: locationId,
				service_id: bookingServiceId,
				customer_name: bookingName,
				customer_phone: bookingPhone,
				vehicle_type: bookingVehicle,
				booking_date: bookingDate,
				time_slot: bookingTime + ':00',
				notes: bookingNotes || null
			});
			bookingSuccess = true;
			showBookingForm = false;
			bookingName = '';
			bookingPhone = '';
			bookingNotes = '';
		} catch (e: any) {
			bookingError = e.message || 'Dat lich that bai';
		}
		bookingSubmitting = false;
	}

	async function refreshQueue() {
		try {
			queue = await api.get<QueueStateResponse>(`/queue/locations/${locationId}`);
		} catch {
			// ignore
		}
	}
</script>

<div>
	<a href="/" class="text-sm text-primary">&larr; Quay lai</a>

	{#if loading}
		<div class="mt-8 text-center text-sm text-muted-foreground">Dang tai...</div>
	{:else}
		<!-- Queue status -->
		<div class="mt-4 grid grid-cols-3 gap-3">
			<div class="rounded-xl bg-card p-4 text-center">
				<p class="text-2xl font-bold">{queue?.waiting.length ?? 0}</p>
				<p class="text-xs text-muted-foreground">Dang cho</p>
			</div>
			<div class="rounded-xl bg-card p-4 text-center">
				<p class="text-2xl font-bold">{queue?.in_progress.length ?? 0}</p>
				<p class="text-xs text-muted-foreground">Dang rua</p>
			</div>
			<div class="rounded-xl bg-card p-4 text-center">
				<p class="text-2xl font-bold text-primary">~{queue?.estimated_wait_minutes ?? 0}</p>
				<p class="text-xs text-muted-foreground">phut cho</p>
			</div>
		</div>

		<!-- Live queue -->
		{#if queue && (queue.waiting.length > 0 || queue.in_progress.length > 0)}
			<div class="mt-4 rounded-xl bg-card p-4">
				<h2 class="text-sm font-semibold">Hang doi hien tai</h2>
				<div class="mt-3 space-y-2">
					{#each queue.in_progress as entry}
						<div class="flex items-center justify-between rounded-lg bg-primary/5 px-3 py-2">
							<div class="flex items-center gap-2">
								<span class="text-sm font-bold">#{entry.queue_number}</span>
								<span class="text-xs text-muted-foreground">{entry.service_name}</span>
							</div>
							<span class="rounded-full bg-primary/10 px-2 py-0.5 text-xs text-primary">Dang rua</span>
						</div>
					{/each}
					{#each queue.waiting as entry}
						<div class="flex items-center justify-between rounded-lg px-3 py-2">
							<div class="flex items-center gap-2">
								<span class="text-sm font-bold">#{entry.queue_number}</span>
								<span class="text-xs text-muted-foreground">{entry.service_name}</span>
							</div>
							<span class="text-xs text-muted-foreground">Cho</span>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Services menu -->
		{#if services.length > 0}
			<div class="mt-4 rounded-xl bg-card p-4">
				<h2 class="text-sm font-semibold">Bang gia dich vu</h2>
				<div class="mt-3 divide-y divide-border">
					{#each services as svc}
						<div class="flex items-center justify-between py-3">
							<div>
								<p class="text-sm font-medium">{svc.name}</p>
								{#if svc.description}
									<p class="text-xs text-muted-foreground">{svc.description}</p>
								{/if}
								<p class="mt-0.5 text-xs text-muted-foreground">
									{svc.vehicle_type} &middot; {svc.duration_minutes} phut
								</p>
							</div>
							<span class="text-sm font-semibold text-primary">{formatVND(svc.base_price)}</span>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Booking section -->
		{#if bookingSuccess}
			<div class="mt-4 rounded-xl bg-green-500/10 p-4 text-center">
				<p class="text-sm font-medium text-green-400">Dat lich thanh cong!</p>
				<p class="mt-1 text-xs text-muted-foreground">Chung toi se xac nhan som nhat.</p>
				<button
					onclick={() => { bookingSuccess = false; }}
					class="mt-3 text-sm text-primary"
				>
					Dat lich khac
				</button>
			</div>
		{:else if showBookingForm}
			<div class="mt-4 rounded-xl bg-card p-4">
				<h2 class="text-sm font-semibold">Dat lich rua xe</h2>
				{#if bookingError}
					<p class="mt-2 text-xs text-red-400">{bookingError}</p>
				{/if}
				<div class="mt-3 space-y-3">
					<input bind:value={bookingName} placeholder="Ho ten" class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm" />
					<input bind:value={bookingPhone} placeholder="So dien thoai" type="tel" class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm" />
					<select bind:value={bookingVehicle} class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm">
						<option value="car">Xe hoi</option>
						<option value="motorcycle">Xe may</option>
						<option value="suv">SUV</option>
						<option value="truck">Xe tai</option>
					</select>
					<select bind:value={bookingServiceId} class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm">
						<option value="">-- Chon dich vu --</option>
						{#each services as svc}
							<option value={svc.id}>{svc.name} - {formatVND(svc.base_price)}</option>
						{/each}
					</select>
					<div class="grid grid-cols-2 gap-3">
						<input bind:value={bookingDate} type="date" class="rounded-lg border border-border bg-background px-3 py-2 text-sm" />
						<input bind:value={bookingTime} type="time" class="rounded-lg border border-border bg-background px-3 py-2 text-sm" />
					</div>
					<input bind:value={bookingNotes} placeholder="Ghi chu (tuy chon)" class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm" />
					<div class="flex gap-2">
						<button
							onclick={submitBooking}
							disabled={bookingSubmitting || !bookingName || !bookingPhone || !bookingServiceId}
							class="flex-1 rounded-lg bg-primary px-4 py-2.5 text-sm font-medium text-primary-foreground disabled:opacity-50"
						>
							{bookingSubmitting ? 'Dang gui...' : 'Xac nhan dat lich'}
						</button>
						<button
							onclick={() => { showBookingForm = false; }}
							class="rounded-lg border border-border px-4 py-2.5 text-sm"
						>
							Huy
						</button>
					</div>
				</div>
			</div>
		{:else}
			<div class="mt-4">
				<button
					onclick={() => { showBookingForm = true; }}
					class="w-full rounded-xl bg-primary px-4 py-3 text-sm font-medium text-primary-foreground"
				>
					Dat lich rua xe
				</button>
			</div>
		{/if}

		<!-- Today stats -->
		<div class="mt-4 rounded-xl bg-card p-4 text-center">
			<p class="text-sm text-muted-foreground">Hom nay da hoan thanh</p>
			<p class="text-2xl font-bold">{queue?.completed_today ?? 0} xe</p>
		</div>

		<!-- Review link -->
		<div class="mt-4">
			<a
				href="/location/{locationId}/review"
				class="block w-full rounded-xl border border-border bg-card p-3 text-center text-sm font-medium text-muted-foreground hover:text-foreground"
			>
				⭐ Danh gia dich vu
			</a>
		</div>
	{/if}
</div>
