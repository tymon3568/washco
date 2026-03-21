<script lang="ts">
	import { api, ApiError } from '$lib/api/client';
	import type { PaymentResponse, DailyRevenueResponse } from '$lib/api/types';
	import { toast } from '$lib/toast.svelte';
	import { formatVND } from '$lib/utils/format';

	let payments: PaymentResponse[] = $state([]);
	let revenue: DailyRevenueResponse | null = $state(null);
	let locationId = $state('');
	let selectedDate = $state(new Date().toISOString().split('T')[0]);
	let completingId: string | null = $state(null);

	$effect(() => {
		loadData();
	});

	async function loadData() {
		try {
			const locations = await api.get<any[]>('/locations');
			if (locations.length > 0) {
				locationId = locations[0].id;
				await refreshPayments();
			}
		} catch {
			// API not available
		}
	}

	async function refreshPayments() {
		if (!locationId) return;
		try {
			const [p, r] = await Promise.all([
				api.get<PaymentResponse[]>(`/payments/locations/${locationId}?date=${selectedDate}`),
				api.get<DailyRevenueResponse>(`/payments/locations/${locationId}/revenue?date=${selectedDate}`)
			]);
			payments = p;
			revenue = r;
		} catch {
			// ignore
		}
	}

	async function completePayment(id: string) {
		completingId = id;
		try {
			await api.put(`/payments/${id}/complete`, {});
			toast.success('Da xac nhan thanh toan');
			await refreshPayments();
		} catch (e: unknown) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
		completingId = null;
	}

	function formatTime(dateStr: string): string {
		const d = new Date(dateStr);
		return d.toLocaleTimeString('vi-VN', { hour: '2-digit', minute: '2-digit' });
	}

	function statusBadgeClass(status: string): string {
		switch (status.toLowerCase()) {
			case 'completed':
				return 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400';
			case 'pending':
				return 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400';
			case 'refunded':
				return 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400';
			default:
				return 'bg-muted text-muted-foreground';
		}
	}

	function statusLabel(status: string): string {
		switch (status.toLowerCase()) {
			case 'completed':
				return 'Hoan thanh';
			case 'pending':
				return 'Cho xu ly';
			case 'refunded':
				return 'Hoan tien';
			default:
				return status;
		}
	}

	function methodBadgeClass(method: string): string {
		switch (method.toLowerCase()) {
			case 'cash':
				return 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400';
			case 'banktransfer':
				return 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400';
			case 'qr':
				return 'bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400';
			case 'ewallet':
				return 'bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400';
			default:
				return 'bg-muted text-muted-foreground';
		}
	}

	function methodLabel(method: string): string {
		switch (method.toLowerCase()) {
			case 'cash':
				return 'Tien mat';
			case 'banktransfer':
				return 'Chuyen khoan';
			case 'qr':
				return 'QR';
			case 'ewallet':
				return 'Vi dien tu';
			default:
				return method;
		}
	}
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Thanh toan</h1>
			<p class="mt-1 text-sm text-muted-foreground">Quan ly thanh toan va doanh thu.</p>
		</div>
		<input
			type="date"
			bind:value={selectedDate}
			onchange={refreshPayments}
			class="rounded-md border border-input bg-background px-3 py-2 text-sm"
		/>
	</div>

	<!-- Revenue KPI Cards -->
	<div class="mt-6 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-6">
		<div class="rounded-lg border border-border bg-card p-6 shadow-xs">
			<p class="text-sm text-muted-foreground">Doanh thu</p>
			<p class="mt-2 text-2xl font-bold font-mono">{formatVND(revenue?.total_revenue ?? 0)}</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-6 shadow-xs">
			<p class="text-sm text-muted-foreground">Hoan thanh</p>
			<p class="mt-2 text-2xl font-bold">{revenue?.completed_count ?? 0}</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-6 shadow-xs">
			<p class="text-sm text-muted-foreground">Tien mat</p>
			<p class="mt-2 text-2xl font-bold font-mono">{formatVND(revenue?.cash_amount ?? 0)}</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-6 shadow-xs">
			<p class="text-sm text-muted-foreground">Chuyen khoan</p>
			<p class="mt-2 text-2xl font-bold font-mono">{formatVND(revenue?.digital_amount ?? 0)}</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-6 shadow-xs">
			<p class="text-sm text-muted-foreground">TB/don</p>
			<p class="mt-2 text-2xl font-bold font-mono">{formatVND(revenue?.avg_per_job ?? 0)}</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-6 shadow-xs">
			<p class="text-sm text-muted-foreground">Cho xu ly</p>
			<p class="mt-2 text-2xl font-bold">{revenue?.pending_count ?? 0}</p>
		</div>
	</div>

	<!-- Payments Table -->
	<div class="mt-6 rounded-lg border border-border bg-card">
		<div class="border-b border-border p-4">
			<h2 class="text-lg font-medium">Danh sach thanh toan</h2>
		</div>
		<div class="overflow-x-auto">
			<table class="min-w-full divide-y divide-border">
				<thead>
					<tr class="bg-muted/50">
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Khach hang</th>
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Dich vu</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Gia goc</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Giam gia</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Thanh toan</th>
						<th class="px-4 py-3 text-center text-sm font-medium text-muted-foreground">Phuong thuc</th>
						<th class="px-4 py-3 text-center text-sm font-medium text-muted-foreground">Trang thai</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Thoi gian</th>
						<th class="px-4 py-3 text-center text-sm font-medium text-muted-foreground"></th>
					</tr>
				</thead>
				<tbody>
					{#if payments.length === 0}
						<tr>
							<td colspan="9" class="px-4 py-8 text-center text-sm text-muted-foreground">
								Chua co thanh toan nao cho ngay nay.
							</td>
						</tr>
					{:else}
						{#each payments as payment (payment.id)}
							<tr class="border-b border-border last:border-0 hover:bg-muted/30">
								<td class="px-4 py-3 text-sm">
									<div>{payment.customer_name}</div>
									{#if payment.customer_phone}
										<div class="text-xs text-muted-foreground">{payment.customer_phone}</div>
									{/if}
								</td>
								<td class="px-4 py-3 text-sm">{payment.service_name}</td>
								<td class="px-4 py-3 text-right text-sm font-mono">{formatVND(payment.base_price)}</td>
								<td class="px-4 py-3 text-right text-sm font-mono">
									{#if payment.discount_amount > 0}
										<span class="text-red-500">-{formatVND(payment.discount_amount)}</span>
									{:else}
										<span class="text-muted-foreground">--</span>
									{/if}
								</td>
								<td class="px-4 py-3 text-right text-sm font-mono font-semibold">{formatVND(payment.final_amount)}</td>
								<td class="px-4 py-3 text-center">
									<span class="inline-block rounded-full px-2 py-0.5 text-xs font-medium {methodBadgeClass(payment.payment_method)}">
										{methodLabel(payment.payment_method)}
									</span>
								</td>
								<td class="px-4 py-3 text-center">
									<span class="inline-block rounded-full px-2 py-0.5 text-xs font-medium {statusBadgeClass(payment.payment_status)}">
										{statusLabel(payment.payment_status)}
									</span>
								</td>
								<td class="px-4 py-3 text-right text-sm text-muted-foreground">
									{formatTime(payment.paid_at ?? payment.created_at)}
								</td>
								<td class="px-4 py-3 text-center">
									{#if payment.payment_status.toLowerCase() === 'pending'}
										<button
											onclick={() => completePayment(payment.id)}
											disabled={completingId === payment.id}
											class="rounded bg-primary px-3 py-1 text-xs font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
										>
											{completingId === payment.id ? '...' : 'Xac nhan'}
										</button>
									{/if}
								</td>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		</div>
	</div>
</div>
