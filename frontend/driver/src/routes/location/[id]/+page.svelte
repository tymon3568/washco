<script lang="ts">
	import { page } from '$app/state';
	import { api } from '$lib/api/client';
	import { formatVND, formatDuration } from '$lib/utils/format';
	import { createQueueSocket } from '$lib/api/ws';
	import { favorites } from '$lib/favorites.svelte';
	import { toast } from '$lib/toast.svelte';
	import type { QueueStateResponse, ServiceResponse, ReviewResponse } from '$lib/api/types';
	import StatusBadge from '$lib/components/StatusBadge.svelte';
	import StarRating from '$lib/components/StarRating.svelte';
	import Skeleton from '$lib/components/Skeleton.svelte';
	import BottomSheet from '$lib/components/BottomSheet.svelte';

	let locationId = $derived(page.params.id!);
	let queue: QueueStateResponse | null = $state(null);
	let services: ServiceResponse[] = $state([]);
	let reviews: ReviewResponse[] = $state([]);
	let loading = $state(true);
	let wsConnection: { close: () => void } | null = null;
	let activeTab = $state<'services' | 'queue' | 'reviews'>('services');
	let isFav = $derived(favorites.isFavorite(locationId));

	// Booking form
	let showBooking = $state(false);
	let bookingName = $state('');
	let bookingPhone = $state('');
	let bookingVehicle = $state('car');
	let bookingServiceId = $state('');
	let bookingDate = $state(new Date().toISOString().split('T')[0]);
	let bookingTime = $state('09:00');
	let bookingNotes = $state('');
	let bookingSubmitting = $state(false);
	let bookingSuccess = $state(false);

	// Queue join
	let showQueueJoin = $state(false);
	let queueName = $state('');
	let queuePhone = $state('');
	let queueVehicle = $state('car');
	let queueServiceId = $state('');
	let queueSubmitting = $state(false);

	$effect(() => {
		loadData();
		return () => {
			wsConnection?.close();
		};
	});

	async function loadData() {
		loading = true;
		try {
			const [q, s, r] = await Promise.all([
				api.get<QueueStateResponse>(`/queue/public/locations/${locationId}`),
				api.get<ServiceResponse[]>(`/catalog/public/locations/${locationId}/services`),
				api.get<ReviewResponse[]>(`/reviews/public/locations/${locationId}`).catch(() => [] as ReviewResponse[])
			]);
			queue = q;
			services = s.filter((s) => s.is_active);
			reviews = r;

			wsConnection?.close();
			wsConnection = createQueueSocket(locationId, refreshQueue);
		} catch {
			toast.error('Khong the tai thong tin tiem');
		}
		loading = false;
	}

	async function refreshQueue() {
		try {
			queue = await api.get<QueueStateResponse>(`/queue/public/locations/${locationId}`);
		} catch {
			// silent
		}
	}

	async function submitBooking() {
		bookingSubmitting = true;
		try {
			await api.post(`/bookings/public/locations/${locationId}`, {
				service_id: bookingServiceId,
				customer_name: bookingName,
				customer_phone: bookingPhone,
				vehicle_type: bookingVehicle,
				booking_date: bookingDate,
				time_slot: bookingTime + ':00',
				notes: bookingNotes || null
			});
			bookingSuccess = true;
			showBooking = false;
			toast.success('Dat lich thanh cong!');
			bookingName = '';
			bookingPhone = '';
			bookingNotes = '';
		} catch (e: any) {
			toast.error(e.message || 'Dat lich that bai');
		}
		bookingSubmitting = false;
	}

	async function submitQueueJoin() {
		queueSubmitting = true;
		try {
			const selectedService = services.find((s) => s.id === queueServiceId);
			await api.post(`/queue/public/locations/${locationId}/join`, {
				customer_name: queueName,
				customer_phone: queuePhone,
				vehicle_type: queueVehicle,
				service_id: queueServiceId,
				service_name: selectedService?.name ?? ''
			});
			showQueueJoin = false;
			toast.success('Da vao hang doi!');
			await refreshQueue();
			activeTab = 'queue';
			queueName = '';
			queuePhone = '';
		} catch (e: any) {
			toast.error(e.message || 'Vao hang doi that bai');
		}
		queueSubmitting = false;
	}

	const vehicleTypes = [
		{ value: 'motorcycle', label: 'Xe may' },
		{ value: 'car', label: 'Xe hoi' },
		{ value: 'suv', label: 'SUV' },
		{ value: 'truck', label: 'Xe tai' }
	];

	let avgRating = $derived.by(() => {
		if (reviews.length === 0) return 0;
		return reviews.reduce((sum, r) => sum + r.rating, 0) / reviews.length;
	});
</script>

<div class="pb-4">
	<!-- Back + actions -->
	<div class="flex items-center justify-between">
		<a href="/" class="inline-flex min-h-11 items-center gap-1 text-sm text-primary">
			<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
				<path d="M19 12H5M12 19l-7-7 7-7" />
			</svg>
			Quay lai
		</a>
		<button
			onclick={() => favorites.toggle(locationId)}
			class="rounded-lg p-2"
			aria-label={isFav ? 'Bo yeu thich' : 'Yeu thich'}
		>
			<svg class="h-5 w-5 {isFav ? 'fill-destructive text-destructive' : 'text-muted-foreground'}" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" fill={isFav ? 'currentColor' : 'none'}>
				<path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
			</svg>
		</button>
	</div>

	{#if loading}
		<div class="mt-4 space-y-4">
			<Skeleton class="h-8 w-2/3" />
			<div class="grid grid-cols-3 gap-3">
				<Skeleton class="h-20 rounded-xl" />
				<Skeleton class="h-20 rounded-xl" />
				<Skeleton class="h-20 rounded-xl" />
			</div>
			<Skeleton class="h-40 rounded-xl" />
		</div>
	{:else}
		<!-- Queue status cards -->
		<div class="mt-4 grid grid-cols-3 gap-3">
			<div class="rounded-xl bg-card p-3 text-center shadow-xs">
				<p class="text-2xl font-bold text-warning">{queue?.waiting.length ?? 0}</p>
				<p class="text-xs text-muted-foreground">Dang cho</p>
			</div>
			<div class="rounded-xl bg-card p-3 text-center shadow-xs">
				<p class="text-2xl font-bold text-primary">{queue?.in_progress.length ?? 0}</p>
				<p class="text-xs text-muted-foreground">Dang rua</p>
			</div>
			<div class="rounded-xl bg-card p-3 text-center shadow-xs">
				<p class="text-2xl font-bold text-accent">~{queue?.estimated_wait_minutes ?? 0}</p>
				<p class="text-xs text-muted-foreground">phut cho</p>
			</div>
		</div>

		<!-- Stats row -->
		<div class="mt-3 flex items-center gap-4 text-sm text-muted-foreground">
			<span class="flex items-center gap-1">
				<span class="text-yellow-400">&#9733;</span>
				{avgRating > 0 ? avgRating.toFixed(1) : '--'}
				<span class="text-xs">({reviews.length})</span>
			</span>
			<span>{queue?.completed_today ?? 0} hoan thanh hom nay</span>
		</div>

		<!-- Tabs -->
		<div class="mt-4 flex rounded-xl bg-card p-1 shadow-xs">
			{#each [
				{ key: 'services', label: 'Dich vu' },
				{ key: 'queue', label: 'Hang doi' },
				{ key: 'reviews', label: 'Danh gia' }
			] as tab (tab.key)}
				<button
					onclick={() => { activeTab = tab.key as typeof activeTab; }}
					class="flex-1 rounded-lg py-2 text-sm font-medium transition-colors {activeTab === tab.key ? 'bg-primary text-primary-foreground' : 'text-muted-foreground'}"
				>
					{tab.label}
					{#if tab.key === 'queue' && (queue?.waiting.length ?? 0) > 0}
						<span class="ml-1 inline-flex h-5 w-5 items-center justify-center rounded-full bg-warning text-[10px] text-white">
							{queue?.waiting.length}
						</span>
					{/if}
				</button>
			{/each}
		</div>

		<!-- Tab content -->
		<div class="mt-4">
			{#if activeTab === 'services'}
				{#if services.length === 0}
					<div class="rounded-xl bg-card p-6 text-center text-sm text-muted-foreground">
						Chua co dich vu nao.
					</div>
				{:else}
					<div class="rounded-xl bg-card shadow-xs">
						<div class="divide-y divide-border">
							{#each services as svc (svc.id)}
								<div class="flex items-center justify-between p-4">
									<div class="min-w-0 flex-1">
										<p class="font-medium">{svc.name}</p>
										{#if svc.description}
											<p class="mt-0.5 text-xs text-muted-foreground">{svc.description}</p>
										{/if}
										<div class="mt-1 flex items-center gap-2 text-xs text-muted-foreground">
											<span class="capitalize">{svc.vehicle_type === 'motorcycle' ? 'Xe may' : svc.vehicle_type === 'car' ? 'Xe hoi' : svc.vehicle_type}</span>
											<span>&#183;</span>
											<span>{formatDuration(svc.duration_minutes)}</span>
										</div>
									</div>
									<span class="shrink-0 text-base font-bold text-primary">{formatVND(svc.base_price)}</span>
								</div>
							{/each}
						</div>
					</div>
				{/if}

			{:else if activeTab === 'queue'}
				{#if queue && (queue.waiting.length > 0 || queue.in_progress.length > 0)}
					<div class="space-y-2">
						{#each queue.in_progress as entry (entry.id)}
							<div class="flex items-center justify-between rounded-xl bg-primary/5 px-4 py-3">
								<div class="flex items-center gap-3">
									<span class="flex h-8 w-8 items-center justify-center rounded-full bg-primary text-sm font-bold text-primary-foreground">
										{entry.queue_number}
									</span>
									<div>
										<p class="text-sm font-medium">{entry.customer_name}</p>
										<p class="text-xs text-muted-foreground">{entry.service_name}</p>
									</div>
								</div>
								<StatusBadge status="in_progress" />
							</div>
						{/each}
						{#each queue.waiting as entry, i (entry.id)}
							<div class="flex items-center justify-between rounded-xl bg-card px-4 py-3 shadow-xs">
								<div class="flex items-center gap-3">
									<span class="flex h-8 w-8 items-center justify-center rounded-full bg-muted text-sm font-bold">
										{entry.queue_number}
									</span>
									<div>
										<p class="text-sm font-medium">{entry.customer_name}</p>
										<p class="text-xs text-muted-foreground">{entry.service_name}</p>
									</div>
								</div>
								<span class="text-xs text-muted-foreground">#{i + 1} trong hang</span>
							</div>
						{/each}
					</div>
				{:else}
					<div class="rounded-xl bg-card p-6 text-center">
						<p class="text-2xl">&#128522;</p>
						<p class="mt-2 text-sm font-medium">Hang doi trong!</p>
						<p class="mt-1 text-xs text-muted-foreground">Vao hang ngay de duoc phuc vu lien.</p>
					</div>
				{/if}

			{:else if activeTab === 'reviews'}
				{#if reviews.length === 0}
					<div class="rounded-xl bg-card p-6 text-center text-sm text-muted-foreground">
						Chua co danh gia nao.
					</div>
				{:else}
					<div class="space-y-3">
						{#each reviews as review (review.id)}
							<div class="rounded-xl bg-card p-4 shadow-xs">
								<div class="flex items-center justify-between">
									<p class="text-sm font-medium">{review.customer_name}</p>
									<StarRating value={review.rating} readonly size="sm" />
								</div>
								{#if review.comment}
									<p class="mt-2 text-sm text-muted-foreground">{review.comment}</p>
								{/if}
								{#if review.reply}
									<div class="mt-2 rounded-lg bg-muted p-3">
										<p class="text-xs font-medium text-primary">Phan hoi tu tiem:</p>
										<p class="mt-1 text-xs text-muted-foreground">{review.reply}</p>
									</div>
								{/if}
							</div>
						{/each}
					</div>
				{/if}
				<a
					href="/location/{locationId}/review"
					class="mt-3 block rounded-xl border border-border bg-card p-3 text-center text-sm font-medium text-primary shadow-xs"
				>
					Viet danh gia
				</a>
			{/if}
		</div>

		<!-- Sticky CTAs -->
		{#if bookingSuccess}
			<div class="mt-4 rounded-xl bg-success/10 p-4 text-center">
				<p class="font-medium text-success">Dat lich thanh cong!</p>
				<p class="mt-1 text-xs text-muted-foreground">Chung toi se xac nhan som nhat.</p>
				<button onclick={() => { bookingSuccess = false; }} class="mt-3 text-sm text-primary">
					Dat lich khac
				</button>
			</div>
		{:else}
			<div class="mt-4 grid grid-cols-2 gap-3">
				<button
					onclick={() => { showQueueJoin = true; }}
					class="min-h-12 rounded-xl border-2 border-primary bg-primary/5 text-sm font-semibold text-primary"
				>
					Vao hang doi
				</button>
				<button
					onclick={() => { showBooking = true; }}
					class="min-h-12 rounded-xl bg-primary text-sm font-semibold text-primary-foreground"
				>
					Dat lich hen
				</button>
			</div>
		{/if}
	{/if}
</div>

<!-- Booking Bottom Sheet -->
<BottomSheet bind:open={showBooking} title="Dat lich rua xe">
	<form onsubmit={(e) => { e.preventDefault(); submitBooking(); }} class="space-y-3">
		<div>
			<label for="bk-name" class="text-sm font-medium">Ho ten</label>
			<input id="bk-name" bind:value={bookingName} autocomplete="name" placeholder="Nguyen Van A" class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm" />
		</div>
		<div>
			<label for="bk-phone" class="text-sm font-medium">So dien thoai</label>
			<input id="bk-phone" bind:value={bookingPhone} type="tel" inputmode="tel" autocomplete="tel" placeholder="0912 345 678" class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm" />
		</div>
		<div class="grid grid-cols-2 gap-3">
			<div>
				<label for="bk-vehicle" class="text-sm font-medium">Loai xe</label>
				<select id="bk-vehicle" bind:value={bookingVehicle} class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm">
					{#each vehicleTypes as vt (vt.value)}
						<option value={vt.value}>{vt.label}</option>
					{/each}
				</select>
			</div>
			<div>
				<label for="bk-service" class="text-sm font-medium">Dich vu</label>
				<select id="bk-service" bind:value={bookingServiceId} class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm">
					<option value="">-- Chon --</option>
					{#each services as svc (svc.id)}
						<option value={svc.id}>{svc.name} - {formatVND(svc.base_price)}</option>
					{/each}
				</select>
			</div>
		</div>
		<div class="grid grid-cols-2 gap-3">
			<div>
				<label for="bk-date" class="text-sm font-medium">Ngay</label>
				<input id="bk-date" bind:value={bookingDate} type="date" class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm" />
			</div>
			<div>
				<label for="bk-time" class="text-sm font-medium">Gio</label>
				<input id="bk-time" bind:value={bookingTime} type="time" class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm" />
			</div>
		</div>
		<div>
			<label for="bk-notes" class="text-sm font-medium">Ghi chu</label>
			<input id="bk-notes" bind:value={bookingNotes} placeholder="Tuy chon" class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm" />
		</div>
		{#if bookingServiceId}
			{@const selected = services.find((s) => s.id === bookingServiceId)}
			{#if selected}
				<div class="rounded-lg bg-muted p-3">
					<div class="flex items-center justify-between text-sm">
						<span class="text-muted-foreground">Tam tinh:</span>
						<span class="font-bold text-primary">{formatVND(selected.base_price)}</span>
					</div>
				</div>
			{/if}
		{/if}
		<button
			type="submit"
			disabled={bookingSubmitting || !bookingName || !bookingPhone || !bookingServiceId}
			class="min-h-12 w-full rounded-xl bg-primary text-sm font-semibold text-primary-foreground disabled:opacity-50"
		>
			{bookingSubmitting ? 'Dang gui...' : 'Xac nhan dat lich'}
		</button>
	</form>
</BottomSheet>

<!-- Queue Join Bottom Sheet -->
<BottomSheet bind:open={showQueueJoin} title="Vao hang doi">
	<form onsubmit={(e) => { e.preventDefault(); submitQueueJoin(); }} class="space-y-3">
		<div>
			<label for="q-name" class="text-sm font-medium">Ho ten</label>
			<input id="q-name" bind:value={queueName} autocomplete="name" placeholder="Nguyen Van A" class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm" />
		</div>
		<div>
			<label for="q-phone" class="text-sm font-medium">So dien thoai</label>
			<input id="q-phone" bind:value={queuePhone} type="tel" inputmode="tel" autocomplete="tel" placeholder="0912 345 678" class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm" />
		</div>
		<div class="grid grid-cols-2 gap-3">
			<div>
				<label for="q-vehicle" class="text-sm font-medium">Loai xe</label>
				<select id="q-vehicle" bind:value={queueVehicle} class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm">
					{#each vehicleTypes as vt (vt.value)}
						<option value={vt.value}>{vt.label}</option>
					{/each}
				</select>
			</div>
			<div>
				<label for="q-service" class="text-sm font-medium">Dich vu</label>
				<select id="q-service" bind:value={queueServiceId} class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm">
					<option value="">-- Chon --</option>
					{#each services as svc (svc.id)}
						<option value={svc.id}>{svc.name}</option>
					{/each}
				</select>
			</div>
		</div>
		{#if queue}
			<div class="rounded-lg bg-muted p-3 text-sm">
				<p class="text-muted-foreground">
					Hien co <span class="font-medium text-foreground">{queue.waiting.length}</span> nguoi dang cho
					&#183; ~{queue.estimated_wait_minutes} phut
				</p>
			</div>
		{/if}
		<button
			type="submit"
			disabled={queueSubmitting || !queueName || !queuePhone || !queueServiceId}
			class="min-h-12 w-full rounded-xl bg-primary text-sm font-semibold text-primary-foreground disabled:opacity-50"
		>
			{queueSubmitting ? 'Dang xu ly...' : 'Xac nhan vao hang'}
		</button>
	</form>
</BottomSheet>
