<script lang="ts">
	import { api } from '$lib/api/client';
	import type { DailySummaryResponse, ServiceMetricResponse } from '$lib/api/types';
	import { formatVND } from '$lib/utils/format';

	let summary: DailySummaryResponse | null = $state(null);
	let serviceMetrics: ServiceMetricResponse[] = $state([]);
	let locationId = $state('');
	let selectedDate = $state(new Date().toISOString().split('T')[0]);

	$effect(() => {
		loadData();
	});

	async function loadData() {
		try {
			const locations = await api.get<any[]>('/locations');
			if (locations.length > 0) {
				locationId = locations[0].id;
				await refreshAnalytics();
			}
		} catch {
			// API not available
		}
	}

	async function refreshAnalytics() {
		if (!locationId) return;
		try {
			const [s, m] = await Promise.all([
				api.get<DailySummaryResponse>(`/analytics/locations/${locationId}/daily?date=${selectedDate}`),
				api.get<ServiceMetricResponse[]>(`/analytics/locations/${locationId}/services?date=${selectedDate}`)
			]);
			summary = s;
			serviceMetrics = m;
		} catch {
			// ignore
		}
	}
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Bao cao</h1>
			<p class="mt-1 text-sm text-muted-foreground">Theo doi hieu suat hoat dong.</p>
		</div>
		<input
			type="date"
			bind:value={selectedDate}
			onchange={refreshAnalytics}
			class="rounded-md border border-input bg-background px-3 py-2 text-sm"
		/>
	</div>

	<!-- KPI Cards -->
	<div class="mt-6 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
		<div class="rounded-lg border border-border bg-card p-6">
			<p class="text-sm text-muted-foreground">Doanh thu</p>
			<p class="mt-2 text-2xl font-bold font-mono">{formatVND(summary?.total_revenue ?? 0)}</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-6">
			<p class="text-sm text-muted-foreground">Hoan thanh</p>
			<p class="mt-2 text-2xl font-bold">{summary?.completed_jobs ?? 0}</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-6">
			<p class="text-sm text-muted-foreground">TG cho TB</p>
			<p class="mt-2 text-2xl font-bold">{summary?.average_wait_minutes ?? 0} phut</p>
		</div>
		<div class="rounded-lg border border-border bg-card p-6">
			<p class="text-sm text-muted-foreground">Huy</p>
			<p class="mt-2 text-2xl font-bold">{summary?.cancellations ?? 0}</p>
		</div>
	</div>

	<!-- Service breakdown -->
	<div class="mt-6 rounded-lg border border-border bg-card">
		<div class="border-b border-border p-4">
			<h2 class="text-lg font-medium">Dich vu</h2>
		</div>
		<table class="w-full">
			<thead>
				<tr class="border-b border-border bg-muted/50">
					<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Dich vu</th>
					<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">So luong</th>
					<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Doanh thu</th>
					<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">TG TB</th>
				</tr>
			</thead>
			<tbody>
				{#if serviceMetrics.length === 0}
					<tr>
						<td colspan="4" class="px-4 py-8 text-center text-sm text-muted-foreground">
							Chua co du lieu cho ngay nay.
						</td>
					</tr>
				{:else}
					{#each serviceMetrics as metric}
						<tr class="border-b border-border last:border-0">
							<td class="px-4 py-3 text-sm">{metric.service_name}</td>
							<td class="px-4 py-3 text-right text-sm">{metric.count}</td>
							<td class="px-4 py-3 text-right text-sm font-mono">{formatVND(metric.revenue)}</td>
							<td class="px-4 py-3 text-right text-sm text-muted-foreground">{metric.average_duration_minutes} phut</td>
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
	</div>
</div>
