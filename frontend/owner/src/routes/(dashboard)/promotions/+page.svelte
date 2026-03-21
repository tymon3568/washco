<script lang="ts">
	import { api, ApiError } from '$lib/api/client';
	import { toast } from '$lib/toast.svelte';
	import { formatVND } from '$lib/utils/format';
	import { untrack } from 'svelte';

	interface Promotion {
		id: string;
		code: string;
		name: string;
		description: string | null;
		discount_type: string;
		discount_value: number;
		min_order: number;
		max_uses: number | null;
		used_count: number;
		valid_from: string;
		valid_to: string;
		location_ids: string[];
		is_active: boolean;
		created_at: string;
	}

	let promotions: Promotion[] = $state([]);
	let loading = $state(false);
	let showForm = $state(false);
	let editingId: string | null = $state(null);
	let deleteConfirmId: string | null = $state(null);

	// Form fields
	let code = $state('');
	let name = $state('');
	let description = $state('');
	let discountType = $state<'percentage' | 'fixed'>('percentage');
	let discountValue = $state(0);
	let minOrder = $state(0);
	let maxUses = $state<number | string>('');
	let validFrom = $state('');
	let validTo = $state('');

	function getStatus(promo: Promotion): 'active' | 'expired' | 'used_up' {
		const now = new Date();
		const to = new Date(promo.valid_to);
		if (to < now) return 'expired';
		if (promo.max_uses !== null && promo.used_count >= promo.max_uses) return 'used_up';
		if (promo.is_active) return 'active';
		return 'expired';
	}

	const statusLabels: Record<string, { label: string; classes: string }> = {
		active: { label: 'Đang hoạt động', classes: 'bg-green-500/15 text-green-400' },
		expired: { label: 'Hết hạn', classes: 'bg-muted text-muted-foreground' },
		used_up: { label: 'Hết lượt', classes: 'bg-red-500/15 text-red-400' }
	};

	function formatDiscount(promo: Promotion): string {
		if (promo.discount_type === 'percentage') return `${promo.discount_value}%`;
		return formatVND(promo.discount_value);
	}

	function formatDate(dateStr: string): string {
		const d = new Date(dateStr);
		return d.toLocaleDateString('vi-VN', { day: '2-digit', month: '2-digit', year: 'numeric' });
	}

	$effect(() => {
		untrack(() => refreshPromotions());
	});

	async function refreshPromotions() {
		try {
			promotions = await api.get<Promotion[]>('/promotions');
		} catch {
			// API not available
		}
	}

	function openAddForm() {
		editingId = null;
		code = '';
		name = '';
		description = '';
		discountType = 'percentage';
		discountValue = 0;
		minOrder = 0;
		maxUses = '';
		validFrom = '';
		validTo = '';
		showForm = true;
	}

	function openEditForm(promo: Promotion) {
		editingId = promo.id;
		code = promo.code;
		name = promo.name;
		description = promo.description ?? '';
		discountType = promo.discount_type as 'percentage' | 'fixed';
		discountValue = promo.discount_value;
		minOrder = promo.min_order;
		maxUses = promo.max_uses ?? '';
		validFrom = promo.valid_from.slice(0, 10);
		validTo = promo.valid_to.slice(0, 10);
		showForm = true;
	}

	async function handleSubmit() {
		loading = true;
		try {
			const body = {
				code: code.toUpperCase(),
				name,
				description: description || null,
				discount_type: discountType,
				discount_value: discountValue,
				min_order: minOrder,
				max_uses: maxUses === '' ? null : Number(maxUses),
				valid_from: validFrom,
				valid_to: validTo
			};
			if (editingId) {
				await api.put(`/promotions/${editingId}`, body);
			} else {
				await api.post('/promotions', body);
			}
			showForm = false;
			await refreshPromotions();
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
		loading = false;
	}

	async function toggleActive(promo: Promotion) {
		try {
			await api.put(`/promotions/${promo.id}`, { is_active: !promo.is_active });
			await refreshPromotions();
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
	}

	async function deletePromotion(id: string) {
		try {
			await api.del(`/promotions/${id}`);
			deleteConfirmId = null;
			await refreshPromotions();
		} catch (e: any) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
	}
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Khuyến mãi</h1>
			<p class="mt-1 text-sm text-muted-foreground">Quản lý các chương trình khuyến mãi và mã giảm giá.</p>
		</div>
		<button
			onclick={openAddForm}
			class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
		>
			Thêm khuyến mãi
		</button>
	</div>

	<!-- Add/Edit form -->
	{#if showForm}
		<div class="mt-4 rounded-lg border border-border bg-card p-4">
			<h3 class="text-sm font-medium">{editingId ? 'Sửa khuyến mãi' : 'Thêm khuyến mãi mới'}</h3>
			<form
				onsubmit={(e) => {
					e.preventDefault();
					handleSubmit();
				}}
				class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3"
			>
				<label class="block">
					<span class="block text-xs text-muted-foreground">Mã khuyến mãi</span>
					<input
						bind:value={code}
						placeholder="VD: GIAM20"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm font-mono uppercase"
						required
					/>
				</label>
				<label class="block">
					<span class="block text-xs text-muted-foreground">Tên</span>
					<input
						bind:value={name}
						placeholder="Tên khuyến mãi"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
						required
					/>
				</label>
				<label class="block">
					<span class="block text-xs text-muted-foreground">Mô tả (tùy chọn)</span>
					<input
						bind:value={description}
						placeholder="Mô tả ngắn"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
					/>
				</label>

				<!-- Discount type -->
				<fieldset>
					<legend class="block text-xs text-muted-foreground">Loại giảm giá</legend>
					<div class="mt-1 flex gap-4">
						<label class="flex items-center gap-1.5 text-sm">
							<input
								type="radio"
								name="discount_type"
								value="percentage"
								checked={discountType === 'percentage'}
								onchange={() => (discountType = 'percentage')}
								class="accent-primary"
							/>
							Phần trăm
						</label>
						<label class="flex items-center gap-1.5 text-sm">
							<input
								type="radio"
								name="discount_type"
								value="fixed"
								checked={discountType === 'fixed'}
								onchange={() => (discountType = 'fixed')}
								class="accent-primary"
							/>
							Cố định
						</label>
					</div>
				</fieldset>

				<label class="block">
					<span class="block text-xs text-muted-foreground">
						Giá trị giảm {discountType === 'percentage' ? '(%)' : '(VND)'}
					</span>
					<input
						bind:value={discountValue}
						type="number"
						min="0"
						max={discountType === 'percentage' ? 100 : undefined}
						step={discountType === 'percentage' ? 1 : 1000}
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
						required
					/>
				</label>

				<label class="block">
					<span class="block text-xs text-muted-foreground">Đơn tối thiểu (VND)</span>
					<input
						bind:value={minOrder}
						type="number"
						min="0"
						step="1000"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
					/>
				</label>

				<label class="block">
					<span class="block text-xs text-muted-foreground">Số lượt dùng tối đa</span>
					<input
						bind:value={maxUses}
						type="number"
						min="1"
						placeholder="Không giới hạn"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
					/>
				</label>

				<label class="block">
					<span class="block text-xs text-muted-foreground">Ngày bắt đầu</span>
					<input
						bind:value={validFrom}
						type="date"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
						required
					/>
				</label>

				<label class="block">
					<span class="block text-xs text-muted-foreground">Ngày kết thúc</span>
					<input
						bind:value={validTo}
						type="date"
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
						required
					/>
				</label>

				<div class="flex items-end gap-2 sm:col-span-2 lg:col-span-3">
					<button
						type="submit"
						disabled={loading}
						class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
					>
						{editingId ? 'Cập nhật' : 'Thêm'}
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

	<!-- Promotions list -->
	{#if promotions.length === 0}
		<div class="mt-6 rounded-lg border border-border bg-card p-8 text-center text-sm text-muted-foreground">
			Chưa có khuyến mãi nào. Bấm "Thêm khuyến mãi" để bắt đầu.
		</div>
	{:else}
		<div class="mt-6 grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
			{#each promotions as promo (promo.id)}
				{@const status = getStatus(promo)}
				{@const statusInfo = statusLabels[status]}
				<div class="rounded-lg border border-border bg-card p-4">
					<div class="flex items-start justify-between">
						<div class="min-w-0">
							<h3 class="truncate text-sm font-medium">{promo.name}</h3>
							<span class="mt-1 inline-block rounded bg-muted px-2 py-0.5 font-mono text-xs text-foreground">
								{promo.code}
							</span>
						</div>
						<span class="ml-2 shrink-0 rounded-full px-2 py-0.5 text-xs font-medium {statusInfo.classes}">
							{statusInfo.label}
						</span>
					</div>

					{#if promo.description}
						<p class="mt-2 text-xs text-muted-foreground">{promo.description}</p>
					{/if}

					<div class="mt-3 space-y-1.5">
						<div class="flex items-center justify-between text-sm">
							<span class="text-muted-foreground">Giảm giá</span>
							<span class="font-medium text-primary">{formatDiscount(promo)}</span>
						</div>
						{#if promo.min_order > 0}
							<div class="flex items-center justify-between text-sm">
								<span class="text-muted-foreground">Đơn tối thiểu</span>
								<span>{formatVND(promo.min_order)}</span>
							</div>
						{/if}
						<div class="flex items-center justify-between text-sm">
							<span class="text-muted-foreground">Thời gian</span>
							<span class="text-xs">{formatDate(promo.valid_from)} - {formatDate(promo.valid_to)}</span>
						</div>
						<div class="flex items-center justify-between text-sm">
							<span class="text-muted-foreground">Đã dùng</span>
							<span>
								{promo.used_count}{promo.max_uses !== null ? ` / ${promo.max_uses}` : ''} lượt
							</span>
						</div>
					</div>

					<!-- Actions -->
					<div class="mt-4 flex items-center justify-between border-t border-border pt-3">
						<button
							onclick={() => toggleActive(promo)}
							class="flex items-center gap-1.5 text-xs"
						>
							<span
								class="inline-block h-3 w-6 rounded-full transition-colors {promo.is_active
									? 'bg-green-500'
									: 'bg-muted-foreground/30'}"
							>
								<span
									class="mt-0.5 block h-2 w-2 rounded-full bg-white transition-transform {promo.is_active
										? 'translate-x-3.5'
										: 'translate-x-0.5'}"
								></span>
							</span>
							<span class="text-muted-foreground">{promo.is_active ? 'Bật' : 'Tắt'}</span>
						</button>
						<div class="flex gap-2">
							<button
								onclick={() => openEditForm(promo)}
								class="text-xs text-primary hover:underline"
							>
								Sửa
							</button>
							{#if deleteConfirmId === promo.id}
								<span class="text-xs text-muted-foreground">Chắc chưa?</span>
								<button
									onclick={() => deletePromotion(promo.id)}
									class="text-xs font-medium text-destructive hover:underline"
								>
									Xóa
								</button>
								<button
									onclick={() => (deleteConfirmId = null)}
									class="text-xs text-muted-foreground hover:underline"
								>
									Hủy
								</button>
							{:else}
								<button
									onclick={() => (deleteConfirmId = promo.id)}
									class="text-xs text-destructive hover:underline"
								>
									Xóa
								</button>
							{/if}
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
