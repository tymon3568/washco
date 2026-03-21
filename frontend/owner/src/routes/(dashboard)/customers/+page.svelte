<script lang="ts">
	import { api, ApiError } from '$lib/api/client';
	import { toast } from '$lib/toast.svelte';
	import type { CustomerResponse, VehicleResponse, MembershipResponse } from '$lib/api/types';
	import { formatVND } from '$lib/utils/format';

	// Customer list
	let customers: CustomerResponse[] = $state([]);
	let loading = $state(false);
	let activeSegment = $state('all');

	// Add customer form
	let showAddForm = $state(false);
	let formPhone = $state('');
	let formName = $state('');
	let formEmail = $state('');
	let formNotes = $state('');
	let formLoading = $state(false);

	// Detail panel
	let selectedCustomer: CustomerResponse | null = $state(null);
	let vehicles: VehicleResponse[] = $state([]);
	let memberships: MembershipResponse[] = $state([]);
	let detailLoading = $state(false);

	// Add vehicle form
	let showVehicleForm = $state(false);
	let vPlateNumber = $state('');
	let vVehicleType = $state('sedan');
	let vBrand = $state('');
	let vModel = $state('');
	let vColor = $state('');
	let vYear = $state('');
	let vNotes = $state('');
	let vehicleFormLoading = $state(false);

	const segments = [
		{ value: 'all', label: 'Tat ca' },
		{ value: 'new', label: 'Moi' },
		{ value: 'regular', label: 'Thuong xuyen' },
		{ value: 'vip', label: 'VIP' },
		{ value: 'dormant', label: 'Ngung' }
	];

	const segmentBadge: Record<string, { bg: string; text: string; label: string }> = {
		new: { bg: 'bg-blue-100', text: 'text-blue-700', label: 'Moi' },
		regular: { bg: 'bg-green-100', text: 'text-green-700', label: 'Thuong xuyen' },
		vip: { bg: 'bg-amber-100', text: 'text-amber-700', label: 'VIP' },
		dormant: { bg: 'bg-gray-100', text: 'text-gray-600', label: 'Ngung' }
	};

	const vehicleTypes = [
		{ value: 'motorbike', label: 'Xe may' },
		{ value: 'sedan', label: 'Sedan' },
		{ value: 'suv', label: 'SUV' },
		{ value: 'truck', label: 'Xe tai' },
		{ value: 'van', label: 'Van' }
	];

	let filteredCustomers = $derived(
		activeSegment === 'all'
			? customers
			: customers.filter((c) => c.segment === activeSegment)
	);

	$effect(() => {
		loadCustomers();
	});

	async function loadCustomers() {
		loading = true;
		try {
			customers = await api.get<CustomerResponse[]>('/customers/customers?limit=200&offset=0');
		} catch {
			// API not available
		}
		loading = false;
	}

	async function addCustomer() {
		formLoading = true;
		try {
			await api.post('/customers/customers', {
				phone: formPhone,
				name: formName,
				email: formEmail || undefined,
				notes: formNotes || undefined,
				tags: []
			});
			toast.success('Them khach hang thanh cong');
			formPhone = '';
			formName = '';
			formEmail = '';
			formNotes = '';
			showAddForm = false;
			await loadCustomers();
		} catch (e: unknown) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
		formLoading = false;
	}

	async function selectCustomer(customer: CustomerResponse) {
		if (selectedCustomer?.id === customer.id) {
			selectedCustomer = null;
			return;
		}
		selectedCustomer = customer;
		detailLoading = true;
		vehicles = [];
		memberships = [];
		showVehicleForm = false;
		try {
			const [v, m] = await Promise.all([
				api.get<VehicleResponse[]>(`/customers/customers/${customer.id}/vehicles`),
				api.get<MembershipResponse[]>(`/customers/customers/${customer.id}/memberships`)
			]);
			vehicles = v;
			memberships = m;
		} catch {
			// ignore
		}
		detailLoading = false;
	}

	async function addVehicle() {
		if (!selectedCustomer) return;
		vehicleFormLoading = true;
		try {
			await api.post(`/customers/customers/${selectedCustomer.id}/vehicles`, {
				plate_number: vPlateNumber || undefined,
				vehicle_type: vVehicleType,
				brand: vBrand || undefined,
				model: vModel || undefined,
				color: vColor || undefined,
				year: vYear ? parseInt(vYear) : undefined,
				notes: vNotes || undefined
			});
			toast.success('Them xe thanh cong');
			vehicles = await api.get<VehicleResponse[]>(
				`/customers/customers/${selectedCustomer.id}/vehicles`
			);
			resetVehicleForm();
		} catch (e: unknown) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
		vehicleFormLoading = false;
	}

	function resetVehicleForm() {
		showVehicleForm = false;
		vPlateNumber = '';
		vVehicleType = 'sedan';
		vBrand = '';
		vModel = '';
		vColor = '';
		vYear = '';
		vNotes = '';
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return '-';
		const d = new Date(dateStr);
		return d.toLocaleDateString('vi-VN');
	}

	function membershipStatusBadge(status: string): { bg: string; text: string } {
		switch (status) {
			case 'active':
				return { bg: 'bg-green-100', text: 'text-green-700' };
			case 'expired':
				return { bg: 'bg-gray-100', text: 'text-gray-600' };
			case 'cancelled':
				return { bg: 'bg-red-100', text: 'text-red-700' };
			default:
				return { bg: 'bg-gray-100', text: 'text-gray-600' };
		}
	}
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Khach hang</h1>
			<p class="mt-1 text-sm text-muted-foreground">
				Quan ly thong tin khach hang, xe va the thanh vien.
			</p>
		</div>
		<button
			onclick={() => (showAddForm = !showAddForm)}
			class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
		>
			+ Them khach hang
		</button>
	</div>

	<!-- Add customer form -->
	{#if showAddForm}
		<div class="mt-4 rounded-lg border border-border bg-card p-4">
			<h3 class="text-sm font-medium">Them khach hang moi</h3>
			<form
				onsubmit={(e) => {
					e.preventDefault();
					addCustomer();
				}}
				class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-5"
			>
				<input
					bind:value={formPhone}
					placeholder="So dien thoai"
					type="tel"
					pattern={"0[0-9]{9}"}
					class="rounded-md border border-input bg-background px-3 py-2 text-sm"
					required
				/>
				<input
					bind:value={formName}
					placeholder="Ho ten"
					class="rounded-md border border-input bg-background px-3 py-2 text-sm"
					required
				/>
				<input
					bind:value={formEmail}
					placeholder="Email (tuy chon)"
					type="email"
					class="rounded-md border border-input bg-background px-3 py-2 text-sm"
				/>
				<input
					bind:value={formNotes}
					placeholder="Ghi chu (tuy chon)"
					class="rounded-md border border-input bg-background px-3 py-2 text-sm"
				/>
				<div class="flex items-end gap-2">
					<button
						type="submit"
						disabled={formLoading || !formPhone || !formName}
						class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
					>
						Them
					</button>
					<button
						type="button"
						onclick={() => (showAddForm = false)}
						class="rounded-md border border-border px-4 py-2 text-sm hover:bg-muted"
					>
						Huy
					</button>
				</div>
			</form>
		</div>
	{/if}

	<!-- Segment filter tabs -->
	<div class="mt-6 flex gap-1 rounded-lg border border-border bg-muted/30 p-1">
		{#each segments as seg (seg.value)}
			<button
				onclick={() => (activeSegment = seg.value)}
				class="rounded-md px-4 py-2 text-sm font-medium transition-colors {activeSegment ===
				seg.value
					? 'bg-background text-foreground shadow-sm'
					: 'text-muted-foreground hover:text-foreground'}"
			>
				{seg.label}
			</button>
		{/each}
	</div>

	<!-- Customer table + detail layout -->
	<div class="mt-4 flex gap-4">
		<!-- Table -->
		<div
			class="rounded-lg border border-border {selectedCustomer
				? 'w-1/2'
				: 'w-full'} transition-all"
		>
			{#if loading}
				<div class="px-4 py-8 text-center text-sm text-muted-foreground">Dang tai...</div>
			{:else}
				<table class="min-w-full divide-y divide-border">
					<thead>
						<tr class="bg-muted/50">
							<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
								>Ten</th
							>
							<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
								>SDT</th
							>
							<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
								>Phan khuc</th
							>
							<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
								>Luot ghe</th
							>
							<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
								>Tong chi</th
							>
							<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
								>Diem</th
							>
						</tr>
					</thead>
					<tbody>
						{#if filteredCustomers.length === 0}
							<tr>
								<td
									colspan="6"
									class="px-4 py-8 text-center text-sm text-muted-foreground"
								>
									Khong co khach hang nao.
								</td>
							</tr>
						{:else}
							{#each filteredCustomers as customer (customer.id)}
								<tr
									onclick={() => selectCustomer(customer)}
									class="cursor-pointer border-b border-border last:border-0 hover:bg-muted/30 {selectedCustomer?.id ===
									customer.id
										? 'bg-primary/5'
										: ''}"
								>
									<td class="px-4 py-3 text-sm font-medium">{customer.name}</td>
									<td class="px-4 py-3 text-sm text-muted-foreground"
										>{customer.phone}</td
									>
									<td class="px-4 py-3 text-sm">
										{#if segmentBadge[customer.segment]}
											{@const badge = segmentBadge[customer.segment]}
											<span
												class="inline-flex rounded-full px-2 py-0.5 text-xs font-medium {badge.bg} {badge.text}"
											>
												{badge.label}
											</span>
										{:else}
											<span class="text-xs text-muted-foreground"
												>{customer.segment}</span
											>
										{/if}
									</td>
									<td class="px-4 py-3 text-right text-sm font-mono"
										>{customer.total_visits}</td
									>
									<td class="px-4 py-3 text-right text-sm font-mono"
										>{formatVND(customer.total_spent)}</td
									>
									<td class="px-4 py-3 text-right text-sm font-mono"
										>{customer.loyalty_points}</td
									>
								</tr>
							{/each}
						{/if}
					</tbody>
				</table>
			{/if}
		</div>

		<!-- Detail panel -->
		{#if selectedCustomer}
			<div class="w-1/2 rounded-lg border border-border bg-card p-6">
				<div class="flex items-start justify-between">
					<div>
						<h2 class="text-lg font-semibold">{selectedCustomer.name}</h2>
						<p class="text-sm text-muted-foreground">{selectedCustomer.phone}</p>
					</div>
					<button
						onclick={() => (selectedCustomer = null)}
						aria-label="Dong"
						class="rounded-md p-1 text-muted-foreground hover:bg-muted hover:text-foreground"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-5 w-5"
							viewBox="0 0 20 20"
							fill="currentColor"
						>
							<path
								fill-rule="evenodd"
								d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
								clip-rule="evenodd"
							/>
						</svg>
					</button>
				</div>

				<!-- Customer info -->
				<div class="mt-4 space-y-2 text-sm">
					<div class="flex justify-between">
						<span class="text-muted-foreground">Email</span>
						<span>{selectedCustomer.email ?? '-'}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-muted-foreground">Phan khuc</span>
						{#if segmentBadge[selectedCustomer.segment]}
							{@const badge = segmentBadge[selectedCustomer.segment]}
							<span
								class="inline-flex rounded-full px-2 py-0.5 text-xs font-medium {badge.bg} {badge.text}"
							>
								{badge.label}
							</span>
						{:else}
							<span>{selectedCustomer.segment}</span>
						{/if}
					</div>
					<div class="flex justify-between">
						<span class="text-muted-foreground">Tong chi tieu</span>
						<span class="font-mono">{formatVND(selectedCustomer.total_spent)}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-muted-foreground">Luot ghe tham</span>
						<span class="font-mono">{selectedCustomer.total_visits}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-muted-foreground">Diem tich luy</span>
						<span class="font-mono">{selectedCustomer.loyalty_points}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-muted-foreground">Lan ghe cuoi</span>
						<span>{formatDate(selectedCustomer.last_visit_at)}</span>
					</div>
					{#if selectedCustomer.notes}
						<div class="flex justify-between">
							<span class="text-muted-foreground">Ghi chu</span>
							<span class="max-w-[60%] text-right">{selectedCustomer.notes}</span>
						</div>
					{/if}
					{#if selectedCustomer.tags.length > 0}
						<div class="flex justify-between">
							<span class="text-muted-foreground">Tags</span>
							<div class="flex flex-wrap justify-end gap-1">
								{#each selectedCustomer.tags as tag (tag)}
									<span
										class="rounded-full bg-muted px-2 py-0.5 text-xs"
										>{tag}</span
									>
								{/each}
							</div>
						</div>
					{/if}
				</div>

				{#if detailLoading}
					<div class="mt-6 text-center text-sm text-muted-foreground">Dang tai...</div>
				{:else}
					<!-- Vehicles section -->
					<div class="mt-6 border-t border-border pt-4">
						<div class="flex items-center justify-between">
							<h3 class="text-sm font-semibold">Xe ({vehicles.length})</h3>
							<button
								onclick={() => (showVehicleForm = !showVehicleForm)}
								class="text-xs text-primary hover:underline"
							>
								+ Them xe
							</button>
						</div>

						{#if showVehicleForm}
							<form
								onsubmit={(e) => {
									e.preventDefault();
									addVehicle();
								}}
								class="mt-3 space-y-2"
							>
								<div class="grid grid-cols-2 gap-2">
									<input
										bind:value={vPlateNumber}
										placeholder="Bien so (tuy chon)"
										class="rounded-md border border-input bg-background px-3 py-1.5 text-sm"
									/>
									<select
										bind:value={vVehicleType}
										class="rounded-md border border-input bg-background px-3 py-1.5 text-sm"
									>
										{#each vehicleTypes as vt (vt.value)}
											<option value={vt.value}>{vt.label}</option>
										{/each}
									</select>
									<input
										bind:value={vBrand}
										placeholder="Hang xe"
										class="rounded-md border border-input bg-background px-3 py-1.5 text-sm"
									/>
									<input
										bind:value={vModel}
										placeholder="Dong xe"
										class="rounded-md border border-input bg-background px-3 py-1.5 text-sm"
									/>
									<input
										bind:value={vColor}
										placeholder="Mau sac"
										class="rounded-md border border-input bg-background px-3 py-1.5 text-sm"
									/>
									<input
										bind:value={vYear}
										placeholder="Nam san xuat"
										type="number"
										min="1990"
										max="2030"
										class="rounded-md border border-input bg-background px-3 py-1.5 text-sm"
									/>
								</div>
								<input
									bind:value={vNotes}
									placeholder="Ghi chu xe (tuy chon)"
									class="w-full rounded-md border border-input bg-background px-3 py-1.5 text-sm"
								/>
								<div class="flex gap-2">
									<button
										type="submit"
										disabled={vehicleFormLoading}
										class="rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
									>
										Them xe
									</button>
									<button
										type="button"
										onclick={resetVehicleForm}
										class="rounded-md border border-border px-3 py-1.5 text-xs hover:bg-muted"
									>
										Huy
									</button>
								</div>
							</form>
						{/if}

						{#if vehicles.length > 0}
							<div class="mt-3 space-y-2">
								{#each vehicles as vehicle (vehicle.id)}
									<div class="rounded-md border border-border p-3">
										<div class="flex items-center justify-between">
											<span class="text-sm font-medium">
												{vehicle.plate_number ?? 'Khong co bien so'}
											</span>
											<span class="text-xs text-muted-foreground">
												{vehicleTypes.find((v) => v.value === vehicle.vehicle_type)?.label ?? vehicle.vehicle_type}
											</span>
										</div>
										{#if vehicle.brand || vehicle.model || vehicle.color}
											<p class="mt-1 text-xs text-muted-foreground">
												{[vehicle.brand, vehicle.model, vehicle.color, vehicle.year]
													.filter(Boolean)
													.join(' - ')}
											</p>
										{/if}
										{#if vehicle.notes}
											<p class="mt-1 text-xs text-muted-foreground italic">
												{vehicle.notes}
											</p>
										{/if}
									</div>
								{/each}
							</div>
						{:else if !showVehicleForm}
							<p class="mt-2 text-xs text-muted-foreground">Chua co xe nao.</p>
						{/if}
					</div>

					<!-- Memberships section -->
					<div class="mt-6 border-t border-border pt-4">
						<h3 class="text-sm font-semibold">The thanh vien ({memberships.length})</h3>
						{#if memberships.length > 0}
							<div class="mt-3 space-y-2">
								{#each memberships as membership (membership.id)}
									{@const statusBadge = membershipStatusBadge(membership.status)}
									<div class="rounded-md border border-border p-3">
										<div class="flex items-center justify-between">
											<span class="text-sm font-medium"
												>{membership.plan_name}</span
											>
											<span
												class="inline-flex rounded-full px-2 py-0.5 text-xs font-medium {statusBadge.bg} {statusBadge.text}"
											>
												{membership.status}
											</span>
										</div>
										<div class="mt-1 flex justify-between text-xs text-muted-foreground">
											<span>
												{membership.plan_type}
												{#if membership.total_uses}
													- {membership.used_count}/{membership.total_uses} luot
												{/if}
											</span>
											<span class="font-mono"
												>{formatVND(membership.price_paid)}</span
											>
										</div>
										<div class="mt-1 text-xs text-muted-foreground">
											{formatDate(membership.valid_from)}
											{#if membership.valid_to}
												~ {formatDate(membership.valid_to)}
											{/if}
										</div>
									</div>
								{/each}
							</div>
						{:else}
							<p class="mt-2 text-xs text-muted-foreground">
								Chua co the thanh vien.
							</p>
						{/if}
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>
