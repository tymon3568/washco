<script lang="ts">
	import { onDestroy } from 'svelte';
	import { api, ApiError } from '$lib/api/client';
	import { toast } from '$lib/toast.svelte';
	import { createQueueSocket } from '$lib/api/ws';
	import type { QueueEntryResponse, QueueStateResponse, ServiceResponse, BayResponse } from '$lib/api/types';
	import { formatVND } from '$lib/utils/format';

	let queue: QueueStateResponse | null = $state(null);
	let services: ServiceResponse[] = $state([]);
	let bays: BayResponse[] = $state([]);
	let locationId = $state('');
	let showAddForm = $state(false);
	let loading = $state(false);
	let wsConnection: { close: () => void } | null = null;

	// Bay selection for advance
	let advancingEntryId: string | null = $state(null);
	let selectedBayId = $state('');

	// Add walk-in form
	let customerName = $state('');
	let customerPhone = $state('');
	let vehicleType = $state('sedan');
	let selectedServiceId = $state('');

	$effect(() => {
		loadData();
		return () => {
			wsConnection?.close();
		};
	});

	async function loadData() {
		try {
			const locations = await api.get<any[]>('/locations');
			if (locations.length > 0) {
				locationId = locations[0].id;
				await refreshQueue();
				services = await api.get<ServiceResponse[]>(`/catalog/locations/${locationId}/services`);
				if (services.length > 0) selectedServiceId = services[0].id;
				bays = await api.get<BayResponse[]>(`/locations/${locationId}/bays`);
				bays = bays.filter((b) => b.is_active);

				// Connect WebSocket for real-time updates
				wsConnection?.close();
				wsConnection = createQueueSocket(locationId, refreshQueue);
			}
		} catch {
			// API not available
		}
	}

	async function refreshQueue() {
		if (!locationId) return;
		try {
			queue = await api.get<QueueStateResponse>(`/queue/locations/${locationId}`);
		} catch {
			// ignore
		}
	}

	async function addWalkin() {
		if (!locationId || !selectedServiceId) return;
		loading = true;
		try {
			await api.post(`/queue/locations/${locationId}/join`, {
				customer_name: customerName,
				customer_phone: customerPhone || undefined,
				vehicle_type: vehicleType,
				service_id: selectedServiceId
			});
			customerName = '';
			customerPhone = '';
			showAddForm = false;
			await refreshQueue();
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
		loading = false;
	}

	function showBaySelect(entryId: string) {
		if (bays.length === 0) {
			// No bays configured, advance without bay
			advanceEntry(entryId, undefined);
			return;
		}
		advancingEntryId = entryId;
		selectedBayId = bays.length > 0 ? bays[0].id : '';
	}

	async function advanceEntry(id: string, bayId: string | undefined) {
		try {
			await api.put(`/queue/${id}/advance`, { bay_id: bayId || undefined });
			advancingEntryId = null;
			selectedBayId = '';
			await refreshQueue();
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
	}

	function cancelAdvance() {
		advancingEntryId = null;
		selectedBayId = '';
	}

	async function completeEntry(id: string) {
		await api.put(`/queue/${id}/complete`, {});
		await refreshQueue();
	}

	async function cancelEntry(id: string) {
		await api.put(`/queue/${id}/cancel`, {});
		await refreshQueue();
	}

	function getBayName(bayId: string | null): string | null {
		if (!bayId) return null;
		const bay = bays.find((b) => b.id === bayId);
		return bay?.name ?? null;
	}

	const vehicleTypes = [
		{ value: 'motorbike', label: 'Xe may' },
		{ value: 'sedan', label: 'Sedan' },
		{ value: 'suv', label: 'SUV' },
		{ value: 'truck', label: 'Xe tai' },
		{ value: 'van', label: 'Van' }
	];
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Hang doi</h1>
			<p class="mt-1 text-sm text-muted-foreground">
				Quan ly hang doi rua xe.
				{#if queue}
					<span class="font-medium text-foreground">
						Cho khoang {queue.estimated_wait_minutes} phut
					</span>
				{/if}
			</p>
		</div>
		<button
			onclick={() => (showAddForm = !showAddForm)}
			class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
		>
			+ Them walk-in
		</button>
	</div>

	<!-- Add walk-in form -->
	{#if showAddForm}
		<div class="mt-4 rounded-lg border border-border bg-card p-4">
			<h3 class="text-sm font-medium">Them khach walk-in</h3>
			<form onsubmit={(e) => { e.preventDefault(); addWalkin(); }} class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-5">
				<input
					bind:value={customerName}
					placeholder="Ten khach hang"
					class="rounded-md border border-input bg-background px-3 py-2 text-sm"
					required
				/>
				<input
					bind:value={customerPhone}
					placeholder="SDT (tuy chon)"
					type="tel"
					class="rounded-md border border-input bg-background px-3 py-2 text-sm"
				/>
				<select bind:value={vehicleType} class="rounded-md border border-input bg-background px-3 py-2 text-sm">
					{#each vehicleTypes as vt}
						<option value={vt.value}>{vt.label}</option>
					{/each}
				</select>
				<select bind:value={selectedServiceId} class="rounded-md border border-input bg-background px-3 py-2 text-sm">
					{#each services as svc}
						<option value={svc.id}>{svc.name} - {formatVND(svc.base_price)}</option>
					{/each}
				</select>
				<button
					type="submit"
					disabled={loading || !customerName}
					class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
				>
					Them
				</button>
			</form>
		</div>
	{/if}

	<!-- Queue board -->
	<div class="mt-6 grid grid-cols-1 gap-4 md:grid-cols-3">
		<!-- Waiting -->
		<div class="rounded-lg border border-border bg-card">
			<div class="border-b border-border p-4">
				<h2 class="text-sm font-medium">
					Dang cho <span class="text-muted-foreground">({queue?.waiting.length ?? 0})</span>
				</h2>
			</div>
			<div class="space-y-2 p-4">
				{#if queue?.waiting.length}
					{#each queue.waiting as entry}
						<div class="rounded-md border border-border p-3">
							<div class="flex items-center justify-between">
								<span class="text-sm font-bold">#{entry.queue_number}</span>
								<span class="rounded-full bg-warning/10 px-2 py-0.5 text-xs text-warning">Cho</span>
							</div>
							<p class="mt-1 text-sm">{entry.customer_name}</p>
							<p class="text-xs text-muted-foreground">{entry.vehicle_type} - {entry.service_name}</p>
							{#if advancingEntryId === entry.id}
								<div class="mt-2 space-y-2">
									<select bind:value={selectedBayId} class="w-full rounded-md border border-input bg-background px-2 py-1 text-xs">
										<option value="">-- Khong chon bay --</option>
										{#each bays as bay}
											<option value={bay.id}>{bay.name}</option>
										{/each}
									</select>
									<div class="flex gap-1">
										<button
											onclick={() => advanceEntry(entry.id, selectedBayId || undefined)}
											class="rounded bg-primary px-2 py-1 text-xs text-primary-foreground"
										>
											Xac nhan
										</button>
										<button
											onclick={cancelAdvance}
											class="rounded bg-muted px-2 py-1 text-xs"
										>
											Huy
										</button>
									</div>
								</div>
							{:else}
								<div class="mt-2 flex gap-1">
									<button
										onclick={() => showBaySelect(entry.id)}
										class="rounded bg-primary px-2 py-1 text-xs text-primary-foreground"
									>
										Bat dau rua
									</button>
									<button
										onclick={() => cancelEntry(entry.id)}
										class="rounded bg-destructive/10 px-2 py-1 text-xs text-destructive"
									>
										Huy
									</button>
								</div>
							{/if}
						</div>
					{/each}
				{:else}
					<p class="text-center text-sm text-muted-foreground py-4">Khong co ai cho</p>
				{/if}
			</div>
		</div>

		<!-- In Progress -->
		<div class="rounded-lg border border-border bg-card">
			<div class="border-b border-border p-4">
				<h2 class="text-sm font-medium">
					Dang rua <span class="text-muted-foreground">({queue?.in_progress.length ?? 0})</span>
				</h2>
			</div>
			<div class="space-y-2 p-4">
				{#if queue?.in_progress.length}
					{#each queue.in_progress as entry}
						<div class="rounded-md border border-primary/20 bg-primary/5 p-3">
							<div class="flex items-center justify-between">
								<span class="text-sm font-bold">#{entry.queue_number}</span>
								<span class="rounded-full bg-primary/10 px-2 py-0.5 text-xs text-primary">Dang rua</span>
							</div>
							<p class="mt-1 text-sm">{entry.customer_name}</p>
							<p class="text-xs text-muted-foreground">{entry.vehicle_type} - {entry.service_name}</p>
							{#if getBayName(entry.bay_id)}
								<p class="text-xs font-medium text-primary">{getBayName(entry.bay_id)}</p>
							{/if}
							<div class="mt-2">
								<button
									onclick={() => completeEntry(entry.id)}
									class="rounded bg-success px-2 py-1 text-xs text-success-foreground"
								>
									Hoan thanh
								</button>
							</div>
						</div>
					{/each}
				{:else}
					<p class="text-center text-sm text-muted-foreground py-4">Khong co xe dang rua</p>
				{/if}
			</div>
		</div>

		<!-- Completed -->
		<div class="rounded-lg border border-border bg-card">
			<div class="border-b border-border p-4">
				<h2 class="text-sm font-medium">
					Hoan thanh <span class="text-muted-foreground">({queue?.completed_today ?? 0})</span>
				</h2>
			</div>
			<div class="p-4">
				<p class="text-center text-sm text-muted-foreground">
					{queue?.completed_today ?? 0} xe hoan thanh hom nay
				</p>
			</div>
		</div>
	</div>
</div>
