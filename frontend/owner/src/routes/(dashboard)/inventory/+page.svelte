<script lang="ts">
	import { api, ApiError } from '$lib/api/client';
	import type {
		MaterialResponse,
		LowStockAlertResponse,
		MaterialVarianceResponse
	} from '$lib/api/types';
	import { toast } from '$lib/toast.svelte';
	import { formatVND } from '$lib/utils/format';

	type Tab = 'materials' | 'alerts' | 'variance';

	let activeTab: Tab = $state('materials');
	let locationId = $state('');
	let loading = $state(false);

	// Materials tab
	let materials: MaterialResponse[] = $state([]);
	let showAddForm = $state(false);
	let addName = $state('');
	let addCategory = $state('Chemicals');
	let addUnit = $state('');
	let addUnitCost = $state(0);
	let addCurrentStock = $state(0);
	let addMinStock = $state(0);

	// Inline transaction form
	let txMaterialId: string | null = $state(null);
	let txType: 'purchase' | 'usage' = $state('purchase');
	let txQuantity = $state(0);
	let txUnitCost: number | undefined = $state(undefined);
	let txNotes = $state('');

	// Low stock alerts tab
	let alerts: LowStockAlertResponse[] = $state([]);

	// Variance tab
	let variance: MaterialVarianceResponse[] = $state([]);
	let varianceFrom = $state('');
	let varianceTo = $state('');

	const categories = ['Chemicals', 'Consumables', 'Equipment', 'Other'];

	const categoryColors: Record<string, string> = {
		Chemicals: 'bg-blue-100 text-blue-800',
		Consumables: 'bg-green-100 text-green-800',
		Equipment: 'bg-amber-100 text-amber-800',
		Other: 'bg-gray-100 text-gray-800'
	};

	const tabs: { key: Tab; label: string }[] = [
		{ key: 'materials', label: 'Vật tư' },
		{ key: 'alerts', label: 'Cảnh báo' },
		{ key: 'variance', label: 'Độ lệch' }
	];

	$effect(() => {
		loadLocation();
	});

	async function loadLocation() {
		try {
			const locations = await api.get<any[]>('/locations');
			if (locations.length > 0) {
				locationId = locations[0].id;
				await refreshMaterials();
			}
		} catch {
			// API not available
		}
	}

	async function refreshMaterials() {
		if (!locationId) return;
		try {
			materials = await api.get<MaterialResponse[]>(
				`/inventory/locations/${locationId}/materials`
			);
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Không thể tải danh sách vật tư');
		}
	}

	async function loadAlerts() {
		if (!locationId) return;
		try {
			alerts = await api.get<LowStockAlertResponse[]>(
				`/inventory/locations/${locationId}/low-stock`
			);
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Không thể tải cảnh báo');
		}
	}

	async function loadVariance() {
		if (!locationId || !varianceFrom || !varianceTo) return;
		try {
			variance = await api.get<MaterialVarianceResponse[]>(
				`/inventory/locations/${locationId}/variance?from=${varianceFrom}&to=${varianceTo}`
			);
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Không thể tải báo cáo độ lệch');
		}
	}

	function switchTab(tab: Tab) {
		activeTab = tab;
		if (tab === 'alerts') loadAlerts();
		if (tab === 'variance' && varianceFrom && varianceTo) loadVariance();
	}

	// Add material
	function resetAddForm() {
		addName = '';
		addCategory = 'Chemicals';
		addUnit = '';
		addUnitCost = 0;
		addCurrentStock = 0;
		addMinStock = 0;
	}

	async function handleAddMaterial() {
		loading = true;
		try {
			await api.post(`/inventory/locations/${locationId}/materials`, {
				name: addName,
				category: addCategory,
				unit: addUnit,
				unit_cost: addUnitCost,
				current_stock: addCurrentStock,
				min_stock: addMinStock
			});
			toast.success('Thêm vật tư thành công');
			showAddForm = false;
			resetAddForm();
			await refreshMaterials();
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
		loading = false;
	}

	// Inline transaction
	function openTransaction(materialId: string, type: 'purchase' | 'usage') {
		txMaterialId = materialId;
		txType = type;
		txQuantity = 0;
		txUnitCost = undefined;
		txNotes = '';
	}

	function closeTransaction() {
		txMaterialId = null;
	}

	async function handleTransaction() {
		if (!txMaterialId || txQuantity <= 0) return;
		loading = true;
		try {
			await api.post('/inventory/transactions', {
				material_id: txMaterialId,
				transaction_type: txType,
				quantity: txQuantity,
				unit_cost: txUnitCost || undefined,
				notes: txNotes || undefined
			});
			toast.success(txType === 'purchase' ? 'Nhập kho thành công' : 'Xuất kho thành công');
			closeTransaction();
			await refreshMaterials();
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
		loading = false;
	}

	async function toggleActive(mat: MaterialResponse) {
		try {
			await api.put(`/inventory/materials/${mat.id}`, { is_active: !mat.is_active });
			await refreshMaterials();
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
	}

	function varianceColor(v: number): string {
		if (v > 0) return 'text-red-600 font-semibold';
		if (v < 0) return 'text-green-600 font-semibold';
		return 'text-muted-foreground';
	}
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Kho vật tư</h1>
			<p class="mt-1 text-sm text-muted-foreground">Quản lý vật tư, tồn kho và báo cáo độ lệch.</p>
		</div>
		{#if activeTab === 'materials'}
			<button
				onclick={() => { showAddForm = !showAddForm; if (!showAddForm) resetAddForm(); }}
				class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
			>
				{showAddForm ? 'Đóng' : 'Thêm vật tư'}
			</button>
		{/if}
	</div>

	<!-- Tab navigation -->
	<div class="mt-4 flex gap-1 border-b border-border">
		{#each tabs as tab (tab.key)}
			<button
				onclick={() => switchTab(tab.key)}
				class="px-4 py-2 text-sm font-medium transition-colors {activeTab === tab.key
					? 'border-b-2 border-primary text-primary'
					: 'text-muted-foreground hover:text-foreground'}"
			>
				{tab.label}
			</button>
		{/each}
	</div>

	<!-- Materials tab -->
	{#if activeTab === 'materials'}
		<!-- Add material form -->
		{#if showAddForm}
			<div class="mt-4 rounded-lg border border-border bg-card p-4">
				<h3 class="text-sm font-medium">Thêm vật tư mới</h3>
				<form
					onsubmit={(e) => { e.preventDefault(); handleAddMaterial(); }}
					class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3"
				>
					<input
						bind:value={addName}
						placeholder="Tên vật tư"
						class="rounded-md border border-input bg-background px-3 py-2 text-sm"
						required
					/>
					<select
						bind:value={addCategory}
						class="rounded-md border border-input bg-background px-3 py-2 text-sm"
					>
						{#each categories as cat (cat)}
							<option value={cat}>{cat}</option>
						{/each}
					</select>
					<input
						bind:value={addUnit}
						placeholder="Đơn vị (lít, kg, cái...)"
						class="rounded-md border border-input bg-background px-3 py-2 text-sm"
						required
					/>
					<div>
						<label class="block text-xs text-muted-foreground" for="add-unit-cost">Đơn giá (VND)</label>
						<input
							id="add-unit-cost"
							bind:value={addUnitCost}
							type="number"
							min="0"
							step="1000"
							class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							required
						/>
					</div>
					<div>
						<label class="block text-xs text-muted-foreground" for="add-current-stock">Tồn kho ban đầu</label>
						<input
							id="add-current-stock"
							bind:value={addCurrentStock}
							type="number"
							min="0"
							class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							required
						/>
					</div>
					<div>
						<label class="block text-xs text-muted-foreground" for="add-min-stock">Tồn tối thiểu</label>
						<input
							id="add-min-stock"
							bind:value={addMinStock}
							type="number"
							min="0"
							class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							required
						/>
					</div>
					<div class="flex items-end gap-2">
						<button
							type="submit"
							disabled={loading}
							class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
						>
							Thêm
						</button>
						<button
							type="button"
							onclick={() => { showAddForm = false; resetAddForm(); }}
							class="rounded-md border border-border px-4 py-2 text-sm hover:bg-muted"
						>
							Hủy
						</button>
					</div>
				</form>
			</div>
		{/if}

		<!-- Materials table -->
		<div class="mt-6 rounded-lg border border-border">
			<table class="w-full">
				<thead>
					<tr class="border-b border-border bg-muted/50">
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Tên</th>
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Danh mục</th>
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Đơn vị</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Đơn giá</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Tồn kho</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Tồn tối thiểu</th>
						<th class="px-4 py-3 text-center text-sm font-medium text-muted-foreground">Trạng thái</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"></th>
					</tr>
				</thead>
				<tbody>
					{#if materials.length === 0}
						<tr>
							<td colspan="8" class="px-4 py-8 text-center text-sm text-muted-foreground">
								Chưa có vật tư nào. Bấm "Thêm vật tư" để bắt đầu.
							</td>
						</tr>
					{:else}
						{#each materials as mat (mat.id)}
							<tr
								class="border-b border-border last:border-0 {mat.current_stock <= mat.min_stock
									? 'bg-red-50 dark:bg-red-950/20'
									: 'hover:bg-muted/30'}"
							>
								<td class="px-4 py-3 text-sm font-medium {mat.current_stock <= mat.min_stock ? 'text-red-700 dark:text-red-400' : ''}">{mat.name}</td>
								<td class="px-4 py-3 text-sm">
									<span class="inline-flex rounded-full px-2 py-0.5 text-xs font-medium {categoryColors[mat.category] ?? categoryColors['Other']}">
										{mat.category}
									</span>
								</td>
								<td class="px-4 py-3 text-sm text-muted-foreground">{mat.unit}</td>
								<td class="px-4 py-3 text-right text-sm font-mono">{formatVND(mat.unit_cost)}</td>
								<td class="px-4 py-3 text-right text-sm font-mono {mat.current_stock <= mat.min_stock ? 'text-red-700 dark:text-red-400 font-semibold' : ''}">
									{mat.current_stock}
								</td>
								<td class="px-4 py-3 text-right text-sm text-muted-foreground">{mat.min_stock}</td>
								<td class="px-4 py-3 text-center">
									<button
										onclick={() => toggleActive(mat)}
										class="inline-flex rounded-full px-2 py-0.5 text-xs font-medium {mat.is_active
											? 'bg-green-100 text-green-800'
											: 'bg-gray-100 text-gray-600'}"
									>
										{mat.is_active ? 'Hoạt động' : 'Ngưng'}
									</button>
								</td>
								<td class="px-4 py-3 text-right">
									<button
										onclick={() => openTransaction(mat.id, 'purchase')}
										class="text-xs text-blue-600 hover:underline"
									>
										Nhập kho
									</button>
									<button
										onclick={() => openTransaction(mat.id, 'usage')}
										class="ml-2 text-xs text-amber-600 hover:underline"
									>
										Xuất kho
									</button>
								</td>
							</tr>
							<!-- Inline transaction form -->
							{#if txMaterialId === mat.id}
								<tr class="border-b border-border bg-muted/20">
									<td colspan="8" class="px-4 py-3">
										<form
											onsubmit={(e) => { e.preventDefault(); handleTransaction(); }}
											class="flex flex-wrap items-end gap-3"
										>
											<span class="text-sm font-medium">
												{txType === 'purchase' ? 'Nhập kho' : 'Xuất kho'}: {mat.name}
											</span>
											<div>
												<label class="block text-xs text-muted-foreground" for="tx-quantity-{mat.id}">Số lượng</label>
												<input
													id="tx-quantity-{mat.id}"
													bind:value={txQuantity}
													type="number"
													min="1"
													class="mt-1 w-24 rounded-md border border-input bg-background px-2 py-1 text-sm"
													required
												/>
											</div>
											{#if txType === 'purchase'}
												<div>
													<label class="block text-xs text-muted-foreground" for="tx-unit-cost-{mat.id}">Đơn giá (VND, tùy chọn)</label>
													<input
														id="tx-unit-cost-{mat.id}"
														bind:value={txUnitCost}
														type="number"
														min="0"
														step="1000"
														class="mt-1 w-28 rounded-md border border-input bg-background px-2 py-1 text-sm"
													/>
												</div>
											{/if}
											<div>
												<label class="block text-xs text-muted-foreground" for="tx-notes-{mat.id}">Ghi chú</label>
												<input
													id="tx-notes-{mat.id}"
													bind:value={txNotes}
													placeholder="Tùy chọn"
													class="mt-1 w-40 rounded-md border border-input bg-background px-2 py-1 text-sm"
												/>
											</div>
											<button
												type="submit"
												disabled={loading}
												class="rounded-md bg-primary px-3 py-1.5 text-sm text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
											>
												Xác nhận
											</button>
											<button
												type="button"
												onclick={closeTransaction}
												class="rounded-md border border-border px-3 py-1.5 text-sm hover:bg-muted"
											>
												Hủy
											</button>
										</form>
									</td>
								</tr>
							{/if}
						{/each}
					{/if}
				</tbody>
			</table>
		</div>

	<!-- Low stock alerts tab -->
	{:else if activeTab === 'alerts'}
		<div class="mt-6">
			{#if alerts.length === 0}
				<div class="rounded-lg border border-border p-8 text-center text-sm text-muted-foreground">
					Không có cảnh báo tồn kho thấp.
				</div>
			{:else}
				<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
					{#each alerts as alert (alert.material_id)}
						<div class="rounded-lg border border-red-200 bg-red-50 p-4 dark:border-red-900 dark:bg-red-950/30">
							<div class="flex items-start justify-between">
								<h3 class="text-sm font-semibold text-red-800 dark:text-red-300">{alert.name}</h3>
								<span class="rounded-full bg-red-200 px-2 py-0.5 text-xs font-medium text-red-800 dark:bg-red-900 dark:text-red-300">
									Thấp
								</span>
							</div>
							<div class="mt-3 space-y-1">
								<div class="flex justify-between text-sm">
									<span class="text-red-700 dark:text-red-400">Tồn kho hiện tại</span>
									<span class="font-mono font-semibold text-red-800 dark:text-red-300">{alert.current_stock} {alert.unit}</span>
								</div>
								<div class="flex justify-between text-sm">
									<span class="text-red-700 dark:text-red-400">Tồn tối thiểu</span>
									<span class="font-mono text-red-700 dark:text-red-400">{alert.min_stock} {alert.unit}</span>
								</div>
							</div>
							<div class="mt-3">
								<div class="h-2 w-full overflow-hidden rounded-full bg-red-200 dark:bg-red-900">
									<div
										class="h-full rounded-full bg-red-500"
										style="width: {Math.min((alert.current_stock / alert.min_stock) * 100, 100)}%"
									></div>
								</div>
								<p class="mt-1 text-xs text-red-600 dark:text-red-400">
									Còn {Math.round((alert.current_stock / alert.min_stock) * 100)}% mức tối thiểu
								</p>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>

	<!-- Variance tab -->
	{:else if activeTab === 'variance'}
		<div class="mt-4 flex flex-wrap items-end gap-3">
			<div>
				<label class="block text-xs text-muted-foreground" for="variance-from">Từ ngày</label>
				<input
					id="variance-from"
					bind:value={varianceFrom}
					type="date"
					class="mt-1 rounded-md border border-input bg-background px-3 py-2 text-sm"
				/>
			</div>
			<div>
				<label class="block text-xs text-muted-foreground" for="variance-to">Đến ngày</label>
				<input
					id="variance-to"
					bind:value={varianceTo}
					type="date"
					class="mt-1 rounded-md border border-input bg-background px-3 py-2 text-sm"
				/>
			</div>
			<button
				onclick={loadVariance}
				disabled={!varianceFrom || !varianceTo}
				class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
			>
				Xem báo cáo
			</button>
		</div>

		<div class="mt-6 rounded-lg border border-border">
			<table class="w-full">
				<thead>
					<tr class="border-b border-border bg-muted/50">
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Vật tư</th>
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Đơn vị</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Số lượt</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Dự kiến</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Thực tế</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Độ lệch</th>
					</tr>
				</thead>
				<tbody>
					{#if variance.length === 0}
						<tr>
							<td colspan="6" class="px-4 py-8 text-center text-sm text-muted-foreground">
								{varianceFrom && varianceTo
									? 'Không có dữ liệu độ lệch trong khoảng thời gian này.'
									: 'Chọn khoảng thời gian và bấm "Xem báo cáo".'}
							</td>
						</tr>
					{:else}
						{#each variance as v (v.material_id)}
							<tr class="border-b border-border last:border-0 hover:bg-muted/30">
								<td class="px-4 py-3 text-sm font-medium">{v.material_name}</td>
								<td class="px-4 py-3 text-sm text-muted-foreground">{v.unit}</td>
								<td class="px-4 py-3 text-right text-sm font-mono">{v.job_count}</td>
								<td class="px-4 py-3 text-right text-sm font-mono">{v.expected_usage}</td>
								<td class="px-4 py-3 text-right text-sm font-mono">{v.actual_usage}</td>
								<td class="px-4 py-3 text-right text-sm font-mono {varianceColor(v.variance)}">
									{v.variance > 0 ? '+' : ''}{v.variance}
								</td>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		</div>
	{/if}
</div>
