<script lang="ts">
	import { api } from '$lib/api/client';
	import type {
		PlatformMetricsResponse,
		AdminLocationResponse,
		AdminActionResponse,
		SubscriptionTierResponse
	} from '$lib/api/types';
	import { toast } from '$lib/toast.svelte';

	let activeTab: 'metrics' | 'locations' | 'audit' | 'tiers' = $state('metrics');
	let metrics: PlatformMetricsResponse | null = $state(null);
	let locations: AdminLocationResponse[] = $state([]);
	let actions: AdminActionResponse[] = $state([]);
	let tiers: SubscriptionTierResponse[] = $state([]);

	$effect(() => {
		loadMetrics();
	});

	$effect(() => {
		if (activeTab === 'locations' && locations.length === 0) loadLocations();
		if (activeTab === 'audit' && actions.length === 0) loadActions();
		if (activeTab === 'tiers' && tiers.length === 0) loadTiers();
	});

	async function loadMetrics() {
		try {
			metrics = await api.get<PlatformMetricsResponse>('/admin/metrics');
		} catch {
			// not admin or API unavailable
		}
	}

	async function loadLocations() {
		try {
			locations = await api.get<AdminLocationResponse[]>('/admin/locations');
		} catch {
			// ignore
		}
	}

	async function loadActions() {
		try {
			actions = await api.get<AdminActionResponse[]>('/admin/actions');
		} catch {
			// ignore
		}
	}

	async function loadTiers() {
		try {
			tiers = await api.get<SubscriptionTierResponse[]>('/admin/tiers');
		} catch {
			// ignore
		}
	}

	async function approveLocation(id: string) {
		try {
			await api.put(`/admin/locations/${id}/approve`, {});
			toast.success('Đã duyệt cơ sở');
			await loadLocations();
		} catch {
			toast.error('Thao tác thất bại');
		}
	}

	async function suspendLocation(id: string) {
		const reason = prompt('Lý do tạm ngưng:');
		if (!reason) return;
		try {
			await api.put(`/admin/locations/${id}/suspend`, { reason });
			toast.success('Đã tạm ngưng cơ sở');
			await loadLocations();
		} catch {
			toast.error('Thao tác thất bại');
		}
	}

	const statusLabels: Record<string, string> = {
		active: 'Hoạt động',
		pending: 'Chờ duyệt',
		suspended: 'Tạm ngưng'
	};

	const statusColors: Record<string, string> = {
		active: 'bg-green-100 text-green-700',
		pending: 'bg-yellow-100 text-yellow-700',
		suspended: 'bg-red-100 text-red-700'
	};
</script>

<div>
	<h1 class="text-2xl font-semibold">Quản trị hệ thống</h1>
	<p class="mt-1 text-sm text-muted-foreground">Quản lý nền tảng WashCo.</p>

	<!-- Tabs -->
	<div class="mt-4 flex gap-1 rounded-lg bg-muted p-1">
		{#each [
			{ key: 'metrics', label: 'Tổng quan' },
			{ key: 'locations', label: 'Cơ sở' },
			{ key: 'audit', label: 'Nhật ký' },
			{ key: 'tiers', label: 'Gói dịch vụ' }
		] as tab}
			<button
				onclick={() => (activeTab = tab.key as typeof activeTab)}
				class="flex-1 rounded-md px-3 py-1.5 text-sm font-medium transition-colors {activeTab === tab.key
					? 'bg-background text-foreground shadow-sm'
					: 'text-muted-foreground hover:text-foreground'}"
			>
				{tab.label}
			</button>
		{/each}
	</div>

	<!-- Metrics -->
	{#if activeTab === 'metrics'}
		{#if metrics}
			<div class="mt-6 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
				<div class="rounded-lg border border-border bg-card p-6">
					<p class="text-sm text-muted-foreground">Tổng doanh nghiệp</p>
					<p class="mt-2 text-2xl font-bold">{metrics.total_tenants}</p>
				</div>
				<div class="rounded-lg border border-border bg-card p-6">
					<p class="text-sm text-muted-foreground">Cơ sở hoạt động</p>
					<p class="mt-2 text-2xl font-bold">
						{metrics.active_locations} / {metrics.total_locations}
					</p>
				</div>
				<div class="rounded-lg border border-border bg-card p-6">
					<p class="text-sm text-muted-foreground">Cơ sở tạm ngưng</p>
					<p class="mt-2 text-2xl font-bold">{metrics.suspended_locations}</p>
				</div>
				<div class="rounded-lg border border-border bg-card p-6">
					<p class="text-sm text-muted-foreground">Tổng người dùng</p>
					<p class="mt-2 text-2xl font-bold">{metrics.total_users}</p>
				</div>
				<div class="rounded-lg border border-border bg-card p-6">
					<p class="text-sm text-muted-foreground">Hàng đợi hôm nay</p>
					<p class="mt-2 text-2xl font-bold">{metrics.total_queue_entries_today}</p>
				</div>
			</div>
		{:else}
			<p class="mt-6 text-center text-sm text-muted-foreground">
				Không có quyền truy cập hoặc API chưa sẵn sàng.
			</p>
		{/if}
	{/if}

	<!-- Locations -->
	{#if activeTab === 'locations'}
		<div class="mt-6 rounded-lg border border-border bg-card">
			<table class="w-full">
				<thead>
					<tr class="border-b border-border bg-muted/50">
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
							>Cơ sở</th
						>
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
							>Doanh nghiệp</th
						>
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
							>Thành phố</th
						>
						<th class="px-4 py-3 text-center text-sm font-medium text-muted-foreground"
							>Trạng thái</th
						>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"></th>
					</tr>
				</thead>
				<tbody>
					{#if locations.length === 0}
						<tr>
							<td colspan="5" class="px-4 py-8 text-center text-sm text-muted-foreground">
								Chưa có cơ sở nào.
							</td>
						</tr>
					{:else}
						{#each locations as loc}
							<tr class="border-b border-border last:border-0">
								<td class="px-4 py-3 text-sm font-medium">{loc.name}</td>
								<td class="px-4 py-3 text-sm">{loc.tenant_name}</td>
								<td class="px-4 py-3 text-sm text-muted-foreground">{loc.city}</td>
								<td class="px-4 py-3 text-center">
									<span
										class="rounded px-2 py-0.5 text-xs font-medium {statusColors[loc.status] ?? 'bg-gray-100 text-gray-600'}"
									>
										{statusLabels[loc.status] ?? loc.status}
									</span>
								</td>
								<td class="px-4 py-3 text-right">
									{#if loc.status === 'pending' || loc.status === 'suspended'}
										<button
											onclick={() => approveLocation(loc.id)}
											class="mr-2 text-sm text-green-600 hover:text-green-800"
										>
											Duyệt
										</button>
									{/if}
									{#if loc.status === 'active'}
										<button
											onclick={() => suspendLocation(loc.id)}
											class="text-sm text-red-500 hover:text-red-700"
										>
											Tạm ngưng
										</button>
									{/if}
								</td>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		</div>
	{/if}

	<!-- Audit Log -->
	{#if activeTab === 'audit'}
		<div class="mt-6 rounded-lg border border-border bg-card">
			<table class="w-full">
				<thead>
					<tr class="border-b border-border bg-muted/50">
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
							>Thời gian</th
						>
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
							>Hành động</th
						>
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
							>Đối tượng</th
						>
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Lý do</th
						>
					</tr>
				</thead>
				<tbody>
					{#if actions.length === 0}
						<tr>
							<td colspan="4" class="px-4 py-8 text-center text-sm text-muted-foreground">
								Chưa có nhật ký.
							</td>
						</tr>
					{:else}
						{#each actions as action}
							<tr class="border-b border-border last:border-0">
								<td class="px-4 py-3 text-sm text-muted-foreground">
									{new Date(action.created_at).toLocaleString('vi-VN')}
								</td>
								<td class="px-4 py-3 text-sm">{action.action_type}</td>
								<td class="px-4 py-3 text-sm"
									>{action.target_type}: {action.target_id.slice(0, 8)}...</td
								>
								<td class="px-4 py-3 text-sm text-muted-foreground"
									>{action.reason ?? '—'}</td
								>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		</div>
	{/if}

	<!-- Tiers -->
	{#if activeTab === 'tiers'}
		<div class="mt-6 grid grid-cols-1 gap-4 sm:grid-cols-3">
			{#each tiers as tier}
				<div
					class="rounded-lg border border-border bg-card p-6 {tier.name === 'pro'
						? 'ring-2 ring-primary'
						: ''}"
				>
					<h3 class="text-lg font-bold">{tier.display_name}</h3>
					<p class="mt-1 text-sm text-muted-foreground">
						{tier.max_locations} cơ sở · {tier.max_staff} nhân viên
					</p>
					<ul class="mt-4 space-y-1">
						{#each tier.features as feature}
							<li class="flex items-center gap-2 text-sm">
								<svg
									class="h-4 w-4 text-green-500"
									fill="none"
									viewBox="0 0 24 24"
									stroke="currentColor"
									stroke-width="2"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										d="M5 13l4 4L19 7"
									/>
								</svg>
								{feature}
							</li>
						{/each}
					</ul>
				</div>
			{/each}
		</div>
	{/if}
</div>
