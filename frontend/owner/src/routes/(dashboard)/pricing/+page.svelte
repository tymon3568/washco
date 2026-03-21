<script lang="ts">
	import { api, ApiError } from '$lib/api/client';
	import { toast } from '$lib/toast.svelte';
	import { formatVND } from '$lib/utils/format';
	import { untrack } from 'svelte';
	import type {
		PricingRuleResponse,
		ServiceResponse,
		PriceCalculationResponse
	} from '$lib/api/types';

	let rules: PricingRuleResponse[] = $state([]);
	let services: ServiceResponse[] = $state([]);
	let locationId = $state('');
	let locations: { id: string; name: string }[] = $state([]);
	let loading = $state(false);
	let showForm = $state(false);
	let editingId: string | null = $state(null);
	let deleteConfirmId: string | null = $state(null);

	// Form fields
	let ruleName = $state('');
	let ruleType = $state('surge');
	let multiplier = $state(1.0);
	let fixedAdjustment = $state(0);
	let priority = $state(0);
	let isActive = $state(true);
	let validFrom = $state('');
	let validTo = $state('');
	let conditionsJson = $state('{}');
	let serviceId = $state('');

	// Price calculator
	let showCalculator = $state(false);
	let calcServiceId = $state('');
	let calcBasePrice = $state(0);
	let calcResult: PriceCalculationResponse | null = $state(null);
	let calculating = $state(false);

	const ruleTypeLabels: Record<string, { label: string; classes: string }> = {
		surge: { label: 'Surge', classes: 'bg-red-100 text-red-700' },
		time_based: { label: 'Theo giờ', classes: 'bg-blue-100 text-blue-700' },
		day_of_week: { label: 'Theo ngày', classes: 'bg-amber-100 text-amber-700' },
		demand: { label: 'Theo nhu cầu', classes: 'bg-green-100 text-green-700' }
	};

	const conditionPlaceholders: Record<string, string> = {
		surge: '{"min_queue": 5, "max_queue": 20}',
		time_based: '{"start_hour": 17, "end_hour": 21}',
		day_of_week: '{"days": ["saturday", "sunday"]}',
		demand: '{"threshold": 10}'
	};

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return '--';
		const d = new Date(dateStr);
		return d.toLocaleDateString('vi-VN', { day: '2-digit', month: '2-digit', year: 'numeric' });
	}

	$effect(() => {
		untrack(() => loadData());
	});

	async function loadData() {
		try {
			locations = await api.get<{ id: string; name: string }[]>('/locations');
			if (locations.length > 0) {
				locationId = locations[0].id;
				await Promise.all([refreshRules(), refreshServices()]);
			}
		} catch {
			// API not available
		}
	}

	async function refreshRules() {
		if (!locationId) return;
		try {
			rules = await api.get<PricingRuleResponse[]>(
				`/pricing/locations/${locationId}/rules`
			);
		} catch {
			rules = [];
		}
	}

	async function refreshServices() {
		if (!locationId) return;
		try {
			services = await api.get<ServiceResponse[]>(
				`/catalog/locations/${locationId}/services`
			);
		} catch {
			services = [];
		}
	}

	async function handleLocationChange() {
		await Promise.all([refreshRules(), refreshServices()]);
		calcResult = null;
	}

	function openAddForm() {
		editingId = null;
		ruleName = '';
		ruleType = 'surge';
		multiplier = 1.0;
		fixedAdjustment = 0;
		priority = 0;
		isActive = true;
		validFrom = '';
		validTo = '';
		conditionsJson = '{}';
		serviceId = '';
		showForm = true;
	}

	function openEditForm(rule: PricingRuleResponse) {
		editingId = rule.id;
		ruleName = rule.name;
		ruleType = rule.rule_type;
		multiplier = rule.multiplier;
		fixedAdjustment = rule.fixed_adjustment;
		priority = rule.priority;
		isActive = rule.is_active;
		validFrom = rule.valid_from ? rule.valid_from.slice(0, 10) : '';
		validTo = rule.valid_to ? rule.valid_to.slice(0, 10) : '';
		conditionsJson = JSON.stringify(rule.conditions, null, 2);
		serviceId = rule.service_id ?? '';
		showForm = true;
	}

	async function handleSubmit() {
		loading = true;
		try {
			let conditions: Record<string, unknown>;
			try {
				conditions = JSON.parse(conditionsJson);
			} catch {
				toast.error('Điều kiện JSON không hợp lệ');
				loading = false;
				return;
			}

			if (editingId) {
				await api.put(`/pricing/rules/${editingId}`, {
					name: ruleName,
					rule_type: ruleType,
					multiplier,
					fixed_adjustment: fixedAdjustment,
					conditions,
					priority,
					is_active: isActive,
					service_id: serviceId || undefined,
					valid_from: validFrom || undefined,
					valid_to: validTo || undefined
				});
				toast.success('Đã cập nhật quy tắc');
			} else {
				await api.post('/pricing/rules', {
					location_id: locationId,
					name: ruleName,
					rule_type: ruleType,
					multiplier,
					fixed_adjustment: fixedAdjustment,
					conditions,
					priority,
					is_active: isActive,
					service_id: serviceId || undefined,
					valid_from: validFrom || undefined,
					valid_to: validTo || undefined
				});
				toast.success('Đã thêm quy tắc mới');
			}
			showForm = false;
			await refreshRules();
		} catch (e: unknown) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
		loading = false;
	}

	async function deleteRule(id: string) {
		try {
			await api.del(`/pricing/rules/${id}`);
			deleteConfirmId = null;
			toast.success('Đã xóa quy tắc');
			await refreshRules();
		} catch (e: unknown) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
	}

	async function calculatePrice() {
		calculating = true;
		try {
			calcResult = await api.post<PriceCalculationResponse>('/pricing/calculate', {
				location_id: locationId,
				service_id: calcServiceId || undefined,
				base_price: calcBasePrice
			});
		} catch (e: unknown) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
			calcResult = null;
		}
		calculating = false;
	}
</script>

<div>
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Định giá</h1>
			<p class="mt-1 text-sm text-muted-foreground">Quản lý quy tắc định giá động</p>
		</div>
		<button
			onclick={openAddForm}
			class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
		>
			Thêm quy tắc mới
		</button>
	</div>

	<!-- Location selector -->
	{#if locations.length > 1}
		<div class="mt-4">
			<select
				bind:value={locationId}
				onchange={handleLocationChange}
				class="rounded-md border border-input bg-background px-3 py-2 text-sm"
			>
				{#each locations as loc (loc.id)}
					<option value={loc.id}>{loc.name}</option>
				{/each}
			</select>
		</div>
	{/if}

	<!-- Add/Edit form -->
	{#if showForm}
		<div class="mt-4 rounded-lg border border-border bg-card p-4">
			<h3 class="text-sm font-medium">{editingId ? 'Sửa quy tắc' : 'Thêm quy tắc mới'}</h3>
			<form
				onsubmit={(e) => {
					e.preventDefault();
					handleSubmit();
				}}
				class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3"
			>
				<label class="block">
					<span class="block text-xs text-muted-foreground">Tên quy tắc</span>
					<input
						bind:value={ruleName}
						placeholder="VD: Giờ cao điểm"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
						required
					/>
				</label>

				<label class="block">
					<span class="block text-xs text-muted-foreground">Loại</span>
					<select
						bind:value={ruleType}
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
					>
						<option value="surge">Surge</option>
						<option value="time_based">Theo giờ</option>
						<option value="day_of_week">Theo ngày</option>
						<option value="demand">Theo nhu cầu</option>
					</select>
				</label>

				<label class="block">
					<span class="block text-xs text-muted-foreground">Dịch vụ (tùy chọn)</span>
					<select
						bind:value={serviceId}
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
					>
						<option value="">Tất cả dịch vụ</option>
						{#each services as svc (svc.id)}
							<option value={svc.id}>{svc.name}</option>
						{/each}
					</select>
				</label>

				<label class="block">
					<span class="block text-xs text-muted-foreground">Hệ số nhân</span>
					<input
						bind:value={multiplier}
						type="number"
						min="0"
						step="0.01"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
						required
					/>
				</label>

				<label class="block">
					<span class="block text-xs text-muted-foreground">Điều chỉnh cố định (VND)</span>
					<input
						bind:value={fixedAdjustment}
						type="number"
						step="1000"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
					/>
				</label>

				<label class="block">
					<span class="block text-xs text-muted-foreground">Độ ưu tiên</span>
					<input
						bind:value={priority}
						type="number"
						min="0"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
					/>
				</label>

				<label class="block">
					<span class="block text-xs text-muted-foreground">Ngày bắt đầu (tùy chọn)</span>
					<input
						bind:value={validFrom}
						type="date"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
					/>
				</label>

				<label class="block">
					<span class="block text-xs text-muted-foreground">Ngày kết thúc (tùy chọn)</span>
					<input
						bind:value={validTo}
						type="date"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
					/>
				</label>

				<label class="flex items-center gap-2 self-end">
					<input
						bind:checked={isActive}
						type="checkbox"
						class="h-4 w-4 accent-primary"
					/>
					<span class="text-sm">Kích hoạt</span>
				</label>

				<label class="block sm:col-span-2 lg:col-span-3">
					<span class="block text-xs text-muted-foreground">Điều kiện (JSON)</span>
					<textarea
						bind:value={conditionsJson}
						placeholder={conditionPlaceholders[ruleType] ?? '{}'}
						rows="3"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 font-mono text-sm"
					></textarea>
				</label>

				<div class="flex items-end gap-2 sm:col-span-2 lg:col-span-3">
					<button
						type="submit"
						disabled={loading}
						class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
					>
						{loading ? 'Đang xử lý...' : editingId ? 'Cập nhật' : 'Thêm'}
					</button>
					<button
						type="button"
						onclick={() => (showForm = false)}
						class="rounded-md border border-border px-4 py-2 text-sm hover:bg-muted"
					>
						Hủy
					</button>
				</div>
			</form>
		</div>
	{/if}

	<!-- Rules table -->
	<div class="mt-6 rounded-lg border border-border">
		<table class="w-full">
			<thead>
				<tr class="border-b border-border bg-muted/50">
					<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Tên</th>
					<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Loại</th>
					<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Hệ số nhân</th>
					<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Điều chỉnh</th>
					<th class="px-4 py-3 text-center text-sm font-medium text-muted-foreground">Ưu tiên</th>
					<th class="px-4 py-3 text-center text-sm font-medium text-muted-foreground">Trạng thái</th>
					<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Thời gian</th>
					<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Thao tác</th>
				</tr>
			</thead>
			<tbody>
				{#if rules.length === 0}
					<tr>
						<td colspan="8" class="px-4 py-8 text-center text-sm text-muted-foreground">
							Chưa có quy tắc định giá nào. Bấm "Thêm quy tắc mới" để bắt đầu.
						</td>
					</tr>
				{:else}
					{#each rules as rule (rule.id)}
						{@const typeInfo = ruleTypeLabels[rule.rule_type] ?? { label: rule.rule_type, classes: 'bg-gray-100 text-gray-600' }}
						<tr class="border-b border-border last:border-0 hover:bg-muted/30">
							<td class="px-4 py-3 text-sm font-medium">{rule.name}</td>
							<td class="px-4 py-3 text-sm">
								<span class="inline-block rounded-full px-2 py-0.5 text-xs font-medium {typeInfo.classes}">
									{typeInfo.label}
								</span>
							</td>
							<td class="px-4 py-3 text-right font-mono text-sm">x{rule.multiplier.toFixed(2)}</td>
							<td class="px-4 py-3 text-right font-mono text-sm">
								{#if rule.fixed_adjustment !== 0}
									{rule.fixed_adjustment > 0 ? '+' : ''}{formatVND(rule.fixed_adjustment)}
								{:else}
									--
								{/if}
							</td>
							<td class="px-4 py-3 text-center text-sm">{rule.priority}</td>
							<td class="px-4 py-3 text-center text-sm">
								{#if rule.is_active}
									<span class="inline-block rounded-full bg-green-100 px-2 py-0.5 text-xs font-medium text-green-700">
										Hoạt động
									</span>
								{:else}
									<span class="inline-block rounded-full bg-gray-100 px-2 py-0.5 text-xs font-medium text-gray-600">
										Tắt
									</span>
								{/if}
							</td>
							<td class="px-4 py-3 text-xs text-muted-foreground">
								{formatDate(rule.valid_from)} - {formatDate(rule.valid_to)}
							</td>
							<td class="px-4 py-3 text-right">
								<button
									onclick={() => openEditForm(rule)}
									class="text-xs text-primary hover:underline"
								>
									Sửa
								</button>
								{#if deleteConfirmId === rule.id}
									<span class="ml-2 text-xs text-muted-foreground">Chắc chưa?</span>
									<button
										onclick={() => deleteRule(rule.id)}
										class="ml-1 text-xs font-medium text-destructive hover:underline"
									>
										Xóa
									</button>
									<button
										onclick={() => (deleteConfirmId = null)}
										class="ml-1 text-xs text-muted-foreground hover:underline"
									>
										Hủy
									</button>
								{:else}
									<button
										onclick={() => (deleteConfirmId = rule.id)}
										class="ml-2 text-xs text-destructive hover:underline"
									>
										Xóa
									</button>
								{/if}
							</td>
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
	</div>

	<!-- Price calculator -->
	<div class="mt-6 rounded-lg border border-border bg-card p-6 shadow-xs">
		<div class="flex items-center justify-between">
			<div>
				<h2 class="text-lg font-semibold">Công cụ tính giá</h2>
				<p class="mt-0.5 text-xs text-muted-foreground">Tính giá cuối cùng sau khi áp dụng quy tắc</p>
			</div>
			<button
				onclick={() => { showCalculator = !showCalculator; calcResult = null; }}
				class="rounded-md border border-border px-3 py-1.5 text-sm hover:bg-muted"
			>
				{showCalculator ? 'Ẩn' : 'Mở'}
			</button>
		</div>

		{#if showCalculator}
			<div class="mt-4 grid grid-cols-1 gap-3 sm:grid-cols-3">
				<label class="block">
					<span class="block text-xs text-muted-foreground">Dịch vụ (tùy chọn)</span>
					<select
						bind:value={calcServiceId}
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
					>
						<option value="">Không chọn</option>
						{#each services as svc (svc.id)}
							<option value={svc.id}>{svc.name} - {formatVND(svc.base_price)}</option>
						{/each}
					</select>
				</label>

				<label class="block">
					<span class="block text-xs text-muted-foreground">Giá gốc (VND)</span>
					<input
						bind:value={calcBasePrice}
						type="number"
						min="0"
						step="1000"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
					/>
				</label>

				<div class="flex items-end">
					<button
						onclick={calculatePrice}
						disabled={calculating || calcBasePrice <= 0}
						class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
					>
						{calculating ? 'Đang tính...' : 'Tính giá'}
					</button>
				</div>
			</div>

			{#if calcResult}
				<div class="mt-4 rounded-md border border-border bg-muted/30 p-4">
					<div class="grid grid-cols-2 gap-4">
						<div>
							<span class="text-xs text-muted-foreground">Giá gốc</span>
							<p class="font-mono text-lg">{formatVND(calcResult.base_price)}</p>
						</div>
						<div>
							<span class="text-xs text-muted-foreground">Giá cuối</span>
							<p class="font-mono text-lg font-semibold text-primary">{formatVND(calcResult.final_price)}</p>
						</div>
					</div>
					{#if calcResult.applied_rules.length > 0}
						<div class="mt-3 border-t border-border pt-3">
							<span class="text-xs font-medium text-muted-foreground">Quy tắc đã áp dụng:</span>
							<ul class="mt-1 space-y-1">
								{#each calcResult.applied_rules as applied (applied.rule_id)}
									<li class="flex items-center justify-between text-sm">
										<span>{applied.rule_name}</span>
										<span class="font-mono text-xs {applied.adjustment >= 0 ? 'text-red-600' : 'text-green-600'}">
											{applied.adjustment >= 0 ? '+' : ''}{formatVND(applied.adjustment)}
										</span>
									</li>
								{/each}
							</ul>
						</div>
					{:else}
						<p class="mt-3 text-xs text-muted-foreground">Không có quy tắc nào được áp dụng.</p>
					{/if}
				</div>
			{/if}
		{/if}
	</div>
</div>
