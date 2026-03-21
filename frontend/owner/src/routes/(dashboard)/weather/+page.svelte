<script lang="ts">
	import { api } from '$lib/api/client';
	import { locationState } from '$lib/location.svelte';
	import type {
		WeatherTriggerResponse,
		CreateWeatherTriggerRequest,
		WeatherDataResponse
	} from '$lib/api/types';
	import { toast } from '$lib/toast.svelte';

	let triggers: WeatherTriggerResponse[] = $state([]);
	let weatherData: WeatherDataResponse | null = $state(null);
	let showForm = $state(false);
	let isSubmitting = $state(false);

	// Form state
	let formPromotionId = $state('');
	let formCondition = $state('rain');
	let formAutoActivate = $state(false);

	// Promotions for the dropdown
	let promotions: { id: string; name: string; code: string }[] = $state([]);

	let locId = $derived(locationState.current?.id);
	let city = $derived(locationState.current?.city ?? '');

	const conditionLabels: Record<string, string> = {
		rain: 'Mưa',
		heavy_rain: 'Mưa lớn',
		sunny_hot: 'Nắng nóng',
		cloudy: 'Nhiều mây'
	};

	$effect(() => {
		if (locId) {
			loadData();
		}
	});

	async function loadData() {
		try {
			const [t, p] = await Promise.all([
				api.get<WeatherTriggerResponse[]>(`/weather/triggers?location_id=${locId}`),
				api.get<{ id: string; name: string; code: string }[]>('/promotions')
			]);
			triggers = t;
			promotions = p;
		} catch {
			// ignore
		}

		if (city) {
			try {
				weatherData = await api.get<WeatherDataResponse>(`/weather/data/${encodeURIComponent(city)}`);
			} catch {
				weatherData = null;
			}
		}
	}

	async function createTrigger() {
		if (!locId || !formPromotionId) return;
		isSubmitting = true;
		try {
			const body: CreateWeatherTriggerRequest = {
				promotion_id: formPromotionId,
				location_id: locId,
				trigger_condition: formCondition,
				auto_activate: formAutoActivate
			};
			await api.post<WeatherTriggerResponse>('/weather/triggers', body);
			toast.success('Tạo trigger thời tiết thành công');
			showForm = false;
			formPromotionId = '';
			formCondition = 'rain';
			formAutoActivate = false;
			await loadData();
		} catch {
			toast.error('Không thể tạo trigger');
		}
		isSubmitting = false;
	}

	async function toggleActive(trigger: WeatherTriggerResponse) {
		try {
			await api.put(`/weather/triggers/${trigger.id}`, {
				is_active: !trigger.is_active
			});
			await loadData();
		} catch {
			toast.error('Cập nhật thất bại');
		}
	}

	async function deleteTrigger(id: string) {
		try {
			await api.del(`/weather/triggers/${id}`);
			toast.success('Đã xóa trigger');
			await loadData();
		} catch {
			toast.error('Không thể xóa');
		}
	}

	function promoName(id: string): string {
		return promotions.find((p) => p.id === id)?.name ?? id.slice(0, 8);
	}
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Thời tiết & Khuyến mãi</h1>
			<p class="mt-1 text-sm text-muted-foreground">
				Tự động kích hoạt khuyến mãi theo thời tiết.
			</p>
		</div>
		<button
			onclick={() => (showForm = !showForm)}
			class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
		>
			{showForm ? 'Hủy' : '+ Thêm trigger'}
		</button>
	</div>

	<!-- Current weather -->
	{#if weatherData}
		<div class="mt-4 rounded-lg border border-border bg-card p-4">
			<p class="text-sm text-muted-foreground">Thời tiết hiện tại — {weatherData.city}</p>
			<div class="mt-2 flex items-center gap-4">
				<span class="text-lg font-bold capitalize">{weatherData.condition}</span>
				{#if weatherData.temperature_c != null}
					<span class="text-muted-foreground">{weatherData.temperature_c}°C</span>
				{/if}
				{#if weatherData.humidity != null}
					<span class="text-muted-foreground">Độ ẩm: {weatherData.humidity}%</span>
				{/if}
			</div>
		</div>
	{/if}

	<!-- Create form -->
	{#if showForm}
		<div class="mt-4 rounded-lg border border-border bg-card p-4">
			<h2 class="text-lg font-medium">Tạo trigger mới</h2>
			<div class="mt-4 grid grid-cols-1 gap-4 sm:grid-cols-2">
				<div>
					<label for="promo" class="block text-sm font-medium text-muted-foreground"
						>Khuyến mãi</label
					>
					<select
						id="promo"
						bind:value={formPromotionId}
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
					>
						<option value="">Chọn khuyến mãi</option>
						{#each promotions as p}
							<option value={p.id}>{p.name} ({p.code})</option>
						{/each}
					</select>
				</div>
				<div>
					<label for="condition" class="block text-sm font-medium text-muted-foreground"
						>Điều kiện</label
					>
					<select
						id="condition"
						bind:value={formCondition}
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
					>
						{#each Object.entries(conditionLabels) as [value, label]}
							<option {value}>{label}</option>
						{/each}
					</select>
				</div>
				<div class="flex items-center gap-2">
					<input
						id="auto"
						type="checkbox"
						bind:checked={formAutoActivate}
						class="h-4 w-4 rounded border-input"
					/>
					<label for="auto" class="text-sm">Tự động kích hoạt</label>
				</div>
			</div>
			<div class="mt-4">
				<button
					onclick={createTrigger}
					disabled={isSubmitting || !formPromotionId}
					class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
				>
					{isSubmitting ? 'Đang tạo...' : 'Tạo trigger'}
				</button>
			</div>
		</div>
	{/if}

	<!-- Triggers list -->
	<div class="mt-6 rounded-lg border border-border bg-card">
		<div class="border-b border-border p-4">
			<h2 class="text-lg font-medium">Danh sách trigger</h2>
		</div>
		<table class="w-full">
			<thead>
				<tr class="border-b border-border bg-muted/50">
					<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
						>Khuyến mãi</th
					>
					<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
						>Điều kiện</th
					>
					<th class="px-4 py-3 text-center text-sm font-medium text-muted-foreground"
						>Tự động</th
					>
					<th class="px-4 py-3 text-center text-sm font-medium text-muted-foreground"
						>Trạng thái</th
					>
					<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
						>Lần kích hoạt</th
					>
					<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"></th>
				</tr>
			</thead>
			<tbody>
				{#if triggers.length === 0}
					<tr>
						<td colspan="6" class="px-4 py-8 text-center text-sm text-muted-foreground">
							Chưa có trigger nào.
						</td>
					</tr>
				{:else}
					{#each triggers as trigger}
						<tr class="border-b border-border last:border-0">
							<td class="px-4 py-3 text-sm">{promoName(trigger.promotion_id)}</td>
							<td class="px-4 py-3 text-sm"
								>{conditionLabels[trigger.trigger_condition] ?? trigger.trigger_condition}</td
							>
							<td class="px-4 py-3 text-center text-sm">
								{#if trigger.auto_activate}
									<span class="text-green-600">Có</span>
								{:else}
									<span class="text-muted-foreground">Không</span>
								{/if}
							</td>
							<td class="px-4 py-3 text-center">
								<button
									onclick={() => toggleActive(trigger)}
									class="rounded px-2 py-0.5 text-xs font-medium {trigger.is_active
										? 'bg-green-100 text-green-700'
										: 'bg-gray-100 text-gray-500'}"
								>
									{trigger.is_active ? 'Hoạt động' : 'Tắt'}
								</button>
							</td>
							<td class="px-4 py-3 text-sm text-muted-foreground">
								{trigger.last_triggered
									? new Date(trigger.last_triggered).toLocaleString('vi-VN')
									: '—'}
							</td>
							<td class="px-4 py-3 text-right">
								<button
									onclick={() => deleteTrigger(trigger.id)}
									class="text-sm text-red-500 hover:text-red-700"
								>
									Xóa
								</button>
							</td>
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
	</div>
</div>
