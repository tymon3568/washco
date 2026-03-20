<script lang="ts">
	import { api } from '$lib/api/client';
	import type { ServiceResponse } from '$lib/api/types';
	import { formatVND } from '$lib/utils/format';

	let services: ServiceResponse[] = $state([]);
	let locationId = $state('');
	let showForm = $state(false);
	let editingId: string | null = $state(null);
	let loading = $state(false);

	// Form fields
	let name = $state('');
	let description = $state('');
	let vehicleType = $state('sedan');
	let basePrice = $state(0);
	let durationMinutes = $state(30);

	$effect(() => {
		loadData();
	});

	async function loadData() {
		try {
			const locations = await api.get<any[]>('/locations');
			if (locations.length > 0) {
				locationId = locations[0].id;
				await refreshServices();
			}
		} catch {
			// API not available
		}
	}

	async function refreshServices() {
		if (!locationId) return;
		services = await api.get<ServiceResponse[]>(`/catalog/locations/${locationId}/services`);
	}

	function openAddForm() {
		editingId = null;
		name = '';
		description = '';
		vehicleType = 'sedan';
		basePrice = 0;
		durationMinutes = 30;
		showForm = true;
	}

	function openEditForm(svc: ServiceResponse) {
		editingId = svc.id;
		name = svc.name;
		description = svc.description ?? '';
		vehicleType = svc.vehicle_type;
		basePrice = svc.base_price;
		durationMinutes = svc.duration_minutes;
		showForm = true;
	}

	async function handleSubmit() {
		loading = true;
		try {
			const body = {
				name,
				description: description || undefined,
				vehicle_type: vehicleType,
				base_price: basePrice,
				duration_minutes: durationMinutes
			};
			if (editingId) {
				await api.put(`/catalog/services/${editingId}`, body);
			} else {
				await api.post(`/catalog/locations/${locationId}/services`, body);
			}
			showForm = false;
			await refreshServices();
		} catch (e: any) {
			alert(e.message);
		}
		loading = false;
	}

	async function deleteService(id: string) {
		if (!confirm('Ban co chac muon xoa dich vu nay?')) return;
		await api.del(`/catalog/services/${id}`);
		await refreshServices();
	}

	const vehicleLabels: Record<string, string> = {
		motorbike: 'Xe may',
		sedan: 'Sedan',
		suv: 'SUV',
		truck: 'Xe tai',
		van: 'Van'
	};
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Dich vu</h1>
			<p class="mt-1 text-sm text-muted-foreground">Quan ly danh sach dich vu va gia.</p>
		</div>
		<button
			onclick={openAddForm}
			class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
		>
			Them dich vu
		</button>
	</div>

	<!-- Add/Edit form -->
	{#if showForm}
		<div class="mt-4 rounded-lg border border-border bg-card p-4">
			<h3 class="text-sm font-medium">{editingId ? 'Sua dich vu' : 'Them dich vu moi'}</h3>
			<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
				<input bind:value={name} placeholder="Ten dich vu" class="rounded-md border border-input bg-background px-3 py-2 text-sm" required />
				<input bind:value={description} placeholder="Mo ta (tuy chon)" class="rounded-md border border-input bg-background px-3 py-2 text-sm" />
				<select bind:value={vehicleType} class="rounded-md border border-input bg-background px-3 py-2 text-sm">
					{#each Object.entries(vehicleLabels) as [val, label]}
						<option value={val}>{label}</option>
					{/each}
				</select>
				<div>
					<label class="block text-xs text-muted-foreground">Gia (VND)</label>
					<input bind:value={basePrice} type="number" min="0" step="1000" class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" required />
				</div>
				<div>
					<label class="block text-xs text-muted-foreground">Thoi gian (phut)</label>
					<input bind:value={durationMinutes} type="number" min="5" class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" required />
				</div>
				<div class="flex items-end gap-2">
					<button type="submit" disabled={loading} class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground hover:bg-primary/90 disabled:opacity-50">
						{editingId ? 'Cap nhat' : 'Them'}
					</button>
					<button type="button" onclick={() => (showForm = false)} class="rounded-md border border-border px-4 py-2 text-sm hover:bg-muted">
						Huy
					</button>
				</div>
			</form>
		</div>
	{/if}

	<!-- Service table -->
	<div class="mt-6 rounded-lg border border-border">
		<table class="w-full">
			<thead>
				<tr class="border-b border-border bg-muted/50">
					<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Ten dich vu</th>
					<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Loai xe</th>
					<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Gia</th>
					<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Thoi gian</th>
					<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"></th>
				</tr>
			</thead>
			<tbody>
				{#if services.length === 0}
					<tr>
						<td colspan="5" class="px-4 py-8 text-center text-sm text-muted-foreground">
							Chua co dich vu nao. Bam "Them dich vu" de bat dau.
						</td>
					</tr>
				{:else}
					{#each services as svc}
						<tr class="border-b border-border last:border-0 hover:bg-muted/30">
							<td class="px-4 py-3 text-sm font-medium">{svc.name}</td>
							<td class="px-4 py-3 text-sm text-muted-foreground">{vehicleLabels[svc.vehicle_type] ?? svc.vehicle_type}</td>
							<td class="px-4 py-3 text-right text-sm font-mono">{formatVND(svc.base_price)}</td>
							<td class="px-4 py-3 text-right text-sm text-muted-foreground">{svc.duration_minutes} phut</td>
							<td class="px-4 py-3 text-right">
								<button onclick={() => openEditForm(svc)} class="text-xs text-primary hover:underline">Sua</button>
								<button onclick={() => deleteService(svc.id)} class="ml-2 text-xs text-destructive hover:underline">Xoa</button>
							</td>
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
	</div>
</div>
