<script lang="ts">
	import { api } from '$lib/api/client';
	import { locationState } from '$lib/location.svelte';
	import type {
		DailySummaryResponse,
		ServiceMetricResponse,
		TrendDataPointResponse,
		PeriodSummaryResponse,
		LocationComparisonResponse
	} from '$lib/api/types';
	import { formatVND } from '$lib/utils/format';

	let activeTab: 'daily' | 'trend' | 'compare' = $state('daily');

	// Daily tab
	let summary: DailySummaryResponse | null = $state(null);
	let serviceMetrics: ServiceMetricResponse[] = $state([]);
	let selectedDate = $state(new Date().toISOString().split('T')[0]);

	// Trend tab
	let trendData: TrendDataPointResponse[] = $state([]);
	let periodSummary: PeriodSummaryResponse | null = $state(null);
	let trendFrom = $state(
		new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString().split('T')[0]
	);
	let trendTo = $state(new Date().toISOString().split('T')[0]);

	// Compare tab
	let comparisons: LocationComparisonResponse[] = $state([]);

	let locId = $derived(locationState.current?.id);

	$effect(() => {
		if (locId && activeTab === 'daily') {
			refreshDaily();
		}
	});

	$effect(() => {
		if (locId && activeTab === 'trend') {
			refreshTrend();
		}
	});

	$effect(() => {
		if (activeTab === 'compare' && locationState.locations.length > 0) {
			refreshCompare();
		}
	});

	async function refreshDaily() {
		if (!locId) return;
		try {
			const [s, m] = await Promise.all([
				api.get<DailySummaryResponse>(
					`/analytics/locations/${locId}/daily?date=${selectedDate}`
				),
				api.get<ServiceMetricResponse[]>(
					`/analytics/locations/${locId}/services?date=${selectedDate}`
				)
			]);
			summary = s;
			serviceMetrics = m;
		} catch {
			// ignore
		}
	}

	async function refreshTrend() {
		if (!locId) return;
		try {
			const [t, p] = await Promise.all([
				api.get<TrendDataPointResponse[]>(
					`/analytics/locations/${locId}/trend?from=${trendFrom}&to=${trendTo}`
				),
				api.get<PeriodSummaryResponse>(
					`/analytics/locations/${locId}/period?from=${trendFrom}&to=${trendTo}`
				)
			]);
			trendData = t;
			periodSummary = p;
		} catch {
			// ignore
		}
	}

	async function refreshCompare() {
		const ids = locationState.locations.map((l) => l.id).join(',');
		if (!ids) return;
		try {
			comparisons = await api.get<LocationComparisonResponse[]>(
				`/analytics/compare?location_ids=${ids}&from=${trendFrom}&to=${trendTo}`
			);
		} catch {
			// ignore
		}
	}

	function maxRevenue(data: TrendDataPointResponse[]): number {
		const max = Math.max(...data.map((d) => d.revenue), 1);
		return max;
	}
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Báo cáo</h1>
			<p class="mt-1 text-sm text-muted-foreground">Theo dõi hiệu suất hoạt động.</p>
		</div>
	</div>

	<!-- Tabs -->
	<div class="mt-4 flex gap-1 rounded-lg bg-muted p-1">
		<button
			onclick={() => (activeTab = 'daily')}
			class="flex-1 rounded-md px-3 py-1.5 text-sm font-medium transition-colors {activeTab === 'daily'
				? 'bg-background text-foreground shadow-sm'
				: 'text-muted-foreground hover:text-foreground'}"
		>
			Theo ngày
		</button>
		<button
			onclick={() => (activeTab = 'trend')}
			class="flex-1 rounded-md px-3 py-1.5 text-sm font-medium transition-colors {activeTab === 'trend'
				? 'bg-background text-foreground shadow-sm'
				: 'text-muted-foreground hover:text-foreground'}"
		>
			Xu hướng
		</button>
		{#if locationState.locations.length > 1}
			<button
				onclick={() => (activeTab = 'compare')}
				class="flex-1 rounded-md px-3 py-1.5 text-sm font-medium transition-colors {activeTab === 'compare'
					? 'bg-background text-foreground shadow-sm'
					: 'text-muted-foreground hover:text-foreground'}"
			>
				So sánh
			</button>
		{/if}
	</div>

	<!-- Daily Tab -->
	{#if activeTab === 'daily'}
		<div class="mt-4 flex justify-end">
			<input
				type="date"
				bind:value={selectedDate}
				onchange={refreshDaily}
				class="rounded-md border border-input bg-background px-3 py-2 text-sm"
			/>
		</div>

		<div class="mt-4 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
			<div class="rounded-lg border border-border bg-card p-6">
				<p class="text-sm text-muted-foreground">Doanh thu</p>
				<p class="mt-2 font-mono text-2xl font-bold">{formatVND(summary?.total_revenue ?? 0)}</p>
			</div>
			<div class="rounded-lg border border-border bg-card p-6">
				<p class="text-sm text-muted-foreground">Hoàn thành</p>
				<p class="mt-2 text-2xl font-bold">{summary?.completed_jobs ?? 0}</p>
			</div>
			<div class="rounded-lg border border-border bg-card p-6">
				<p class="text-sm text-muted-foreground">TG chờ TB</p>
				<p class="mt-2 text-2xl font-bold">
					{(summary?.average_wait_minutes ?? 0).toFixed(1)} phút
				</p>
			</div>
			<div class="rounded-lg border border-border bg-card p-6">
				<p class="text-sm text-muted-foreground">Hủy</p>
				<p class="mt-2 text-2xl font-bold">{summary?.cancellations ?? 0}</p>
			</div>
		</div>

		<div class="mt-6 rounded-lg border border-border bg-card">
			<div class="border-b border-border p-4">
				<h2 class="text-lg font-medium">Dịch vụ</h2>
			</div>
			<table class="w-full">
				<thead>
					<tr class="border-b border-border bg-muted/50">
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
							>Dịch vụ</th
						>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
							>Số lượng</th
						>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
							>Doanh thu</th
						>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">TG TB</th>
					</tr>
				</thead>
				<tbody>
					{#if serviceMetrics.length === 0}
						<tr>
							<td colspan="4" class="px-4 py-8 text-center text-sm text-muted-foreground">
								Chưa có dữ liệu cho ngày này.
							</td>
						</tr>
					{:else}
						{#each serviceMetrics as metric}
							<tr class="border-b border-border last:border-0">
								<td class="px-4 py-3 text-sm">{metric.service_name}</td>
								<td class="px-4 py-3 text-right text-sm">{metric.count}</td>
								<td class="px-4 py-3 text-right font-mono text-sm"
									>{formatVND(metric.revenue)}</td
								>
								<td class="px-4 py-3 text-right text-sm text-muted-foreground"
									>{metric.average_duration_minutes} phút</td
								>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		</div>
	{/if}

	<!-- Trend Tab -->
	{#if activeTab === 'trend'}
		<div class="mt-4 flex items-center gap-2">
			<label class="text-sm text-muted-foreground">Từ</label>
			<input
				type="date"
				bind:value={trendFrom}
				onchange={refreshTrend}
				class="rounded-md border border-input bg-background px-3 py-2 text-sm"
			/>
			<label class="text-sm text-muted-foreground">đến</label>
			<input
				type="date"
				bind:value={trendTo}
				onchange={refreshTrend}
				class="rounded-md border border-input bg-background px-3 py-2 text-sm"
			/>
		</div>

		{#if periodSummary}
			<div class="mt-4 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
				<div class="rounded-lg border border-border bg-card p-6">
					<p class="text-sm text-muted-foreground">Tổng doanh thu</p>
					<p class="mt-2 font-mono text-2xl font-bold">
						{formatVND(periodSummary.total_revenue)}
					</p>
				</div>
				<div class="rounded-lg border border-border bg-card p-6">
					<p class="text-sm text-muted-foreground">Tổng hoàn thành</p>
					<p class="mt-2 text-2xl font-bold">{periodSummary.total_completed}</p>
				</div>
				<div class="rounded-lg border border-border bg-card p-6">
					<p class="text-sm text-muted-foreground">TG chờ TB</p>
					<p class="mt-2 text-2xl font-bold">
						{periodSummary.average_wait_minutes.toFixed(1)} phút
					</p>
				</div>
				<div class="rounded-lg border border-border bg-card p-6">
					<p class="text-sm text-muted-foreground">Ngày bận nhất</p>
					<p class="mt-2 text-2xl font-bold">{periodSummary.busiest_day ?? '—'}</p>
				</div>
			</div>
		{/if}

		<!-- Revenue bar chart -->
		<div class="mt-6 rounded-lg border border-border bg-card p-4">
			<h2 class="text-lg font-medium">Doanh thu theo ngày</h2>
			{#if trendData.length === 0}
				<p class="py-8 text-center text-sm text-muted-foreground">Chưa có dữ liệu.</p>
			{:else}
				<div class="mt-4 flex items-end gap-0.5 overflow-x-auto" style="height: 200px;">
					{#each trendData as point}
						{@const pct = (point.revenue / maxRevenue(trendData)) * 100}
						<div class="group relative flex min-w-[8px] flex-1 flex-col items-center justify-end">
							<div
								class="w-full rounded-t bg-primary/80 transition-colors group-hover:bg-primary"
								style="height: {Math.max(pct, 1)}%"
							></div>
							<div
								class="pointer-events-none absolute bottom-full mb-1 hidden rounded bg-foreground px-2 py-1 text-xs text-background group-hover:block"
							>
								{point.date}: {formatVND(point.revenue)}
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>

		<!-- Trend table -->
		<div class="mt-6 rounded-lg border border-border bg-card">
			<div class="border-b border-border p-4">
				<h2 class="text-lg font-medium">Chi tiết xu hướng</h2>
			</div>
			<div class="overflow-x-auto">
				<table class="w-full">
					<thead>
						<tr class="border-b border-border bg-muted/50">
							<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
								>Ngày</th
							>
							<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
								>Doanh thu</th
							>
							<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
								>Hoàn thành</th
							>
							<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
								>Walk-in</th
							>
							<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
								>Hủy</th
							>
							<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
								>TG chờ</th
							>
						</tr>
					</thead>
					<tbody>
						{#each trendData as point}
							<tr class="border-b border-border last:border-0">
								<td class="px-4 py-3 text-sm">{point.date}</td>
								<td class="px-4 py-3 text-right font-mono text-sm"
									>{formatVND(point.revenue)}</td
								>
								<td class="px-4 py-3 text-right text-sm">{point.completed_jobs}</td>
								<td class="px-4 py-3 text-right text-sm">{point.walk_ins}</td>
								<td class="px-4 py-3 text-right text-sm">{point.cancellations}</td>
								<td class="px-4 py-3 text-right text-sm text-muted-foreground"
									>{point.average_wait_minutes.toFixed(1)} phút</td
								>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	{/if}

	<!-- Compare Tab -->
	{#if activeTab === 'compare'}
		<div class="mt-4 flex items-center gap-2">
			<label class="text-sm text-muted-foreground">Từ</label>
			<input
				type="date"
				bind:value={trendFrom}
				onchange={refreshCompare}
				class="rounded-md border border-input bg-background px-3 py-2 text-sm"
			/>
			<label class="text-sm text-muted-foreground">đến</label>
			<input
				type="date"
				bind:value={trendTo}
				onchange={refreshCompare}
				class="rounded-md border border-input bg-background px-3 py-2 text-sm"
			/>
		</div>

		<div class="mt-6 rounded-lg border border-border bg-card">
			<div class="border-b border-border p-4">
				<h2 class="text-lg font-medium">So sánh cơ sở</h2>
			</div>
			<table class="w-full">
				<thead>
					<tr class="border-b border-border bg-muted/50">
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
							>Cơ sở</th
						>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
							>Doanh thu</th
						>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
							>Hoàn thành</th
						>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
							>TG chờ TB</th
						>
					</tr>
				</thead>
				<tbody>
					{#if comparisons.length === 0}
						<tr>
							<td colspan="4" class="px-4 py-8 text-center text-sm text-muted-foreground">
								Cần ít nhất 2 cơ sở để so sánh.
							</td>
						</tr>
					{:else}
						{#each comparisons as comp}
							<tr class="border-b border-border last:border-0">
								<td class="px-4 py-3 text-sm font-medium">{comp.location_name}</td>
								<td class="px-4 py-3 text-right font-mono text-sm"
									>{formatVND(comp.total_revenue)}</td
								>
								<td class="px-4 py-3 text-right text-sm">{comp.total_completed}</td>
								<td class="px-4 py-3 text-right text-sm text-muted-foreground"
									>{comp.average_wait_minutes.toFixed(1)} phút</td
								>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		</div>
	{/if}
</div>
