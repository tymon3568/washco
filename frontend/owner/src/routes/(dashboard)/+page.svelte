<script lang="ts">
	import { api } from '$lib/api/client';
	import { formatVND } from '$lib/utils/format';
	import type { DailySummaryResponse, QueueStateResponse } from '$lib/api/types';

	let summary: DailySummaryResponse | null = $state(null);
	let queue: QueueStateResponse | null = $state(null);
	let locationId = $state('');

	$effect(() => {
		loadData();
	});

	async function loadData() {
		try {
			// Get first location for the owner
			const locations = await api.get<any[]>('/locations');
			if (locations.length > 0) {
				locationId = locations[0].id;
				const [s, q] = await Promise.all([
					api.get<DailySummaryResponse>(`/analytics/locations/${locationId}/daily`),
					api.get<QueueStateResponse>(`/queue/locations/${locationId}`)
				]);
				summary = s;
				queue = q;
			}
		} catch {
			// API not available yet, show defaults
		}
	}
</script>

<div>
	<h1 class="text-2xl font-semibold">Dashboard</h1>
	<p class="mt-1 text-sm text-muted-foreground">Tổng quan hoạt động hôm nay.</p>

	<div class="mt-6 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
		<div class="rounded-lg border border-border bg-card p-6 shadow-xs">
			<p class="text-sm text-muted-foreground">Doanh thu hôm nay</p>
			<p class="mt-2 text-2xl font-bold font-mono">
				{formatVND(summary?.total_revenue ?? 0)}
			</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-6 shadow-xs">
			<p class="text-sm text-muted-foreground">Hoàn thành</p>
			<p class="mt-2 text-2xl font-bold">{summary?.completed_jobs ?? 0}</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-6 shadow-xs">
			<p class="text-sm text-muted-foreground">Đang chờ</p>
			<p class="mt-2 text-2xl font-bold">{queue?.waiting.length ?? 0}</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-6 shadow-xs">
			<p class="text-sm text-muted-foreground">TG chờ TB</p>
			<p class="mt-2 text-2xl font-bold">{summary?.average_wait_minutes ?? 0} phút</p>
		</div>
	</div>

	{#if queue && (queue.waiting.length > 0 || queue.in_progress.length > 0)}
		<div class="mt-6 rounded-lg border border-border bg-card p-6">
			<h2 class="text-lg font-medium">Hàng đợi hiện tại</h2>
			<div class="mt-4 space-y-2">
				{#each queue.in_progress as entry}
					<div class="flex items-center justify-between rounded-md bg-primary/5 px-4 py-2">
						<div>
							<span class="font-medium">#{entry.queue_number}</span>
							<span class="ml-2 text-sm">{entry.customer_name}</span>
							<span class="ml-2 text-xs text-muted-foreground">{entry.service_name}</span>
						</div>
						<span class="rounded-full bg-primary/10 px-2 py-0.5 text-xs font-medium text-primary">Đang rửa</span>
					</div>
				{/each}
				{#each queue.waiting as entry}
					<div class="flex items-center justify-between rounded-md px-4 py-2">
						<div>
							<span class="font-medium">#{entry.queue_number}</span>
							<span class="ml-2 text-sm">{entry.customer_name}</span>
							<span class="ml-2 text-xs text-muted-foreground">{entry.service_name}</span>
						</div>
						<span class="text-xs text-muted-foreground">~{entry.estimated_wait_minutes} phút</span>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>
