<script lang="ts">
	import { api, ApiError } from '$lib/api/client';
	import type {
		StaffResponse,
		ShiftResponse,
		CommissionRuleResponse
	} from '$lib/api/types';
	import { toast } from '$lib/toast.svelte';
	import { formatVND } from '$lib/utils/format';

	type Tab = 'staff' | 'shifts' | 'commission';

	let activeTab: Tab = $state('staff');
	let locationId = $state('');
	let loading = $state(false);

	// Staff state
	let staffList: StaffResponse[] = $state([]);
	let showStaffForm = $state(false);
	let staffUserId = $state('');
	let staffDisplayName = $state('');
	let staffSkillLevel = $state('Junior');
	let staffHourlyRate = $state(0);

	// Shifts state
	let shifts: ShiftResponse[] = $state([]);
	let shiftDate = $state(new Date().toISOString().slice(0, 10));
	let showShiftForm = $state(false);
	let shiftStaffId = $state('');
	let shiftStartTime = $state('08:00');
	let shiftEndTime = $state('17:00');

	// Commission state
	let commissionRules: CommissionRuleResponse[] = $state([]);
	let showCommissionForm = $state(false);
	let ruleName = $state('');
	let ruleServiceId = $state('');
	let ruleSkillLevel = $state('');
	let ruleRoleInJob = $state('primary');
	let ruleCommissionType = $state('Fixed');
	let ruleCommissionValue = $state(0);

	const skillLevelColors: Record<string, string> = {
		Junior: 'bg-blue-100 text-blue-700',
		Mid: 'bg-green-100 text-green-700',
		Senior: 'bg-amber-100 text-amber-700',
		Lead: 'bg-purple-100 text-purple-700'
	};

	const shiftStatusColors: Record<string, { class: string; label: string }> = {
		Scheduled: { class: 'bg-blue-100 text-blue-700', label: 'Da len lich' },
		CheckedIn: { class: 'bg-green-100 text-green-700', label: 'Da check-in' },
		Completed: { class: 'bg-gray-100 text-gray-600', label: 'Hoan thanh' },
		Absent: { class: 'bg-red-100 text-red-700', label: 'Vang mat' }
	};

	const tabs: { key: Tab; label: string }[] = [
		{ key: 'staff', label: 'Nhan vien' },
		{ key: 'shifts', label: 'Ca lam viec' },
		{ key: 'commission', label: 'Hoa hong' }
	];

	$effect(() => {
		loadData();
	});

	$effect(() => {
		if (locationId && activeTab === 'shifts') {
			loadShifts();
		}
	});

	async function loadData() {
		try {
			const locations = await api.get<any[]>('/locations');
			if (locations.length > 0) {
				locationId = locations[0].id;
				await refreshStaff();
				await loadCommissionRules();
				await loadShifts();
			}
		} catch {
			// API not available
		}
	}

	// --- Staff ---

	async function refreshStaff() {
		if (!locationId) return;
		staffList = await api.get<StaffResponse[]>(`/staff/locations/${locationId}/staff`);
	}

	function openStaffForm() {
		staffUserId = '';
		staffDisplayName = '';
		staffSkillLevel = 'Junior';
		staffHourlyRate = 0;
		showStaffForm = true;
	}

	async function handleAddStaff() {
		loading = true;
		try {
			await api.post(`/staff/locations/${locationId}/staff`, {
				user_id: staffUserId,
				display_name: staffDisplayName,
				skill_level: staffSkillLevel,
				hourly_rate: staffHourlyRate
			});
			showStaffForm = false;
			toast.success('Da them nhan vien');
			await refreshStaff();
		} catch (e: unknown) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
		loading = false;
	}

	async function toggleStaffActive(staff: StaffResponse) {
		try {
			await api.put(`/staff/staff/${staff.id}`, { is_active: !staff.is_active });
			await refreshStaff();
		} catch (e: unknown) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
	}

	function getStaffName(staffId: string): string {
		const found = staffList.find((s) => s.id === staffId);
		return found?.display_name ?? staffId.slice(0, 8);
	}

	// --- Shifts ---

	async function loadShifts() {
		if (!locationId) return;
		shifts = await api.get<ShiftResponse[]>(
			`/staff/locations/${locationId}/shifts?date=${shiftDate}`
		);
	}

	function openShiftForm() {
		shiftStaffId = staffList.length > 0 ? staffList[0].id : '';
		shiftStartTime = '08:00';
		shiftEndTime = '17:00';
		showShiftForm = true;
	}

	async function handleAddShift() {
		loading = true;
		try {
			await api.post(`/staff/locations/${locationId}/shifts`, {
				staff_id: shiftStaffId,
				shift_date: shiftDate,
				start_time: shiftStartTime,
				end_time: shiftEndTime
			});
			showShiftForm = false;
			toast.success('Da them ca lam viec');
			await loadShifts();
		} catch (e: unknown) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
		loading = false;
	}

	// --- Commission ---

	async function loadCommissionRules() {
		if (!locationId) return;
		commissionRules = await api.get<CommissionRuleResponse[]>(
			`/staff/locations/${locationId}/commission-rules`
		);
	}

	function openCommissionForm() {
		ruleName = '';
		ruleServiceId = '';
		ruleSkillLevel = '';
		ruleRoleInJob = 'primary';
		ruleCommissionType = 'Fixed';
		ruleCommissionValue = 0;
		showCommissionForm = true;
	}

	async function handleAddCommissionRule() {
		loading = true;
		try {
			await api.post(`/staff/locations/${locationId}/commission-rules`, {
				name: ruleName,
				service_id: ruleServiceId || undefined,
				skill_level: ruleSkillLevel || undefined,
				role_in_job: ruleRoleInJob,
				commission_type: ruleCommissionType,
				commission_value: ruleCommissionValue
			});
			showCommissionForm = false;
			toast.success('Da them quy tac hoa hong');
			await loadCommissionRules();
		} catch (e: unknown) {
			toast.error(e instanceof ApiError ? e.message : 'Co loi xay ra');
		}
		loading = false;
	}

	function formatCommissionValue(type: string, value: number): string {
		return type === 'Percentage' ? `${value}%` : formatVND(value);
	}
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Nhan vien & Hoa hong</h1>
			<p class="mt-1 text-sm text-muted-foreground">Quan ly nhan vien, ca lam viec va hoa hong.</p>
		</div>
	</div>

	<!-- Tab navigation -->
	<div class="mt-6 flex gap-1 rounded-lg bg-muted p-1">
		{#each tabs as tab (tab.key)}
			<button
				onclick={() => (activeTab = tab.key)}
				class="flex-1 rounded-md px-4 py-2 text-sm font-medium transition-colors {activeTab === tab.key
					? 'bg-background text-foreground shadow-sm'
					: 'text-muted-foreground hover:text-foreground'}"
			>
				{tab.label}
			</button>
		{/each}
	</div>

	<!-- Staff Tab -->
	{#if activeTab === 'staff'}
		<div class="mt-4 flex justify-end">
			<button
				onclick={openStaffForm}
				class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
			>
				Them nhan vien
			</button>
		</div>

		{#if showStaffForm}
			<div class="mt-4 rounded-lg border border-border bg-card p-4">
				<h3 class="text-sm font-medium">Them nhan vien moi</h3>
				<form
					onsubmit={(e) => {
						e.preventDefault();
						handleAddStaff();
					}}
					class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-5"
				>
					<input
						bind:value={staffUserId}
						placeholder="User ID"
						class="rounded-md border border-input bg-background px-3 py-2 text-sm"
						required
					/>
					<input
						bind:value={staffDisplayName}
						placeholder="Ten hien thi"
						class="rounded-md border border-input bg-background px-3 py-2 text-sm"
						required
					/>
					<select
						bind:value={staffSkillLevel}
						class="rounded-md border border-input bg-background px-3 py-2 text-sm"
					>
						<option value="Junior">Junior</option>
						<option value="Mid">Mid</option>
						<option value="Senior">Senior</option>
						<option value="Lead">Lead</option>
					</select>
					<div>
						<label for="staff-hourly-rate" class="block text-xs text-muted-foreground">Luong/gio (VND)</label>
						<input
							id="staff-hourly-rate"
							bind:value={staffHourlyRate}
							type="number"
							min="0"
							step="1000"
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
							Them
						</button>
						<button
							type="button"
							onclick={() => (showStaffForm = false)}
							class="rounded-md border border-border px-4 py-2 text-sm hover:bg-muted"
						>
							Huy
						</button>
					</div>
				</form>
			</div>
		{/if}

		<div class="mt-6 rounded-lg border border-border">
			<table class="w-full">
				<thead>
					<tr class="border-b border-border bg-muted/50">
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Ten</th>
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Cap do</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"
							>Luong/gio</th
						>
						<th class="px-4 py-3 text-center text-sm font-medium text-muted-foreground"
							>Trang thai</th
						>
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
							>Ngay vao lam</th
						>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground"></th>
					</tr>
				</thead>
				<tbody>
					{#if staffList.length === 0}
						<tr>
							<td colspan="6" class="px-4 py-8 text-center text-sm text-muted-foreground">
								Chua co nhan vien nao. Bam "Them nhan vien" de bat dau.
							</td>
						</tr>
					{:else}
						{#each staffList as staff (staff.id)}
							<tr class="border-b border-border last:border-0 hover:bg-muted/30">
								<td class="px-4 py-3 text-sm font-medium">{staff.display_name}</td>
								<td class="px-4 py-3 text-sm">
									<span
										class="inline-block rounded-full px-2 py-0.5 text-xs font-medium {skillLevelColors[staff.skill_level] ?? 'bg-gray-100 text-gray-700'}"
									>
										{staff.skill_level}
									</span>
								</td>
								<td class="px-4 py-3 text-right text-sm font-mono"
									>{formatVND(staff.hourly_rate)}</td
								>
								<td class="px-4 py-3 text-center text-sm">
									{#if staff.is_active}
										<span
											class="inline-block rounded-full bg-green-100 px-2 py-0.5 text-xs font-medium text-green-700"
										>
											Hoat dong
										</span>
									{:else}
										<span
											class="inline-block rounded-full bg-gray-100 px-2 py-0.5 text-xs font-medium text-gray-500"
										>
											Nghi viec
										</span>
									{/if}
								</td>
								<td class="px-4 py-3 text-sm text-muted-foreground">{staff.joined_date}</td>
								<td class="px-4 py-3 text-right">
									<button
										onclick={() => toggleStaffActive(staff)}
										class="text-xs text-primary hover:underline"
									>
										{staff.is_active ? 'Vo hieu hoa' : 'Kich hoat'}
									</button>
								</td>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		</div>
	{/if}

	<!-- Shifts Tab -->
	{#if activeTab === 'shifts'}
		<div class="mt-4 flex items-center justify-between gap-4">
			<div class="flex items-center gap-2">
				<label for="shift-date" class="text-sm text-muted-foreground">Ngay:</label>
				<input
					id="shift-date"
					type="date"
					bind:value={shiftDate}
					onchange={() => loadShifts()}
					class="rounded-md border border-input bg-background px-3 py-2 text-sm"
				/>
			</div>
			<button
				onclick={openShiftForm}
				class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
			>
				Them ca
			</button>
		</div>

		{#if showShiftForm}
			<div class="mt-4 rounded-lg border border-border bg-card p-4">
				<h3 class="text-sm font-medium">Them ca lam viec</h3>
				<form
					onsubmit={(e) => {
						e.preventDefault();
						handleAddShift();
					}}
					class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-5"
				>
					<select
						bind:value={shiftStaffId}
						class="rounded-md border border-input bg-background px-3 py-2 text-sm"
						required
					>
						{#each staffList.filter((s) => s.is_active) as staff (staff.id)}
							<option value={staff.id}>{staff.display_name}</option>
						{/each}
					</select>
					<div>
						<label for="shift-start" class="block text-xs text-muted-foreground">Gio bat dau</label>
						<input
							id="shift-start"
							bind:value={shiftStartTime}
							type="time"
							class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							required
						/>
					</div>
					<div>
						<label for="shift-end" class="block text-xs text-muted-foreground">Gio ket thuc</label>
						<input
							id="shift-end"
							bind:value={shiftEndTime}
							type="time"
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
							Them
						</button>
						<button
							type="button"
							onclick={() => (showShiftForm = false)}
							class="rounded-md border border-border px-4 py-2 text-sm hover:bg-muted"
						>
							Huy
						</button>
					</div>
				</form>
			</div>
		{/if}

		<div class="mt-6 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
			{#if shifts.length === 0}
				<div class="col-span-full py-8 text-center text-sm text-muted-foreground">
					Khong co ca lam viec nao cho ngay nay.
				</div>
			{:else}
				{#each shifts as shift (shift.id)}
					{@const statusInfo = shiftStatusColors[shift.status] ?? {
						class: 'bg-gray-100 text-gray-600',
						label: shift.status
					}}
					<div class="rounded-lg border border-border bg-card p-4">
						<div class="flex items-center justify-between">
							<span class="text-sm font-medium">{getStaffName(shift.staff_id)}</span>
							<span class="inline-block rounded-full px-2 py-0.5 text-xs font-medium {statusInfo.class}">
								{statusInfo.label}
							</span>
						</div>
						<p class="mt-2 text-sm text-muted-foreground">
							{shift.start_time.slice(0, 5)} - {shift.end_time.slice(0, 5)}
						</p>
						<p class="mt-1 text-xs text-muted-foreground">{shift.shift_date}</p>
					</div>
				{/each}
			{/if}
		</div>
	{/if}

	<!-- Commission Tab -->
	{#if activeTab === 'commission'}
		<div class="mt-4 flex justify-end">
			<button
				onclick={openCommissionForm}
				class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
			>
				Them quy tac
			</button>
		</div>

		{#if showCommissionForm}
			<div class="mt-4 rounded-lg border border-border bg-card p-4">
				<h3 class="text-sm font-medium">Them quy tac hoa hong</h3>
				<form
					onsubmit={(e) => {
						e.preventDefault();
						handleAddCommissionRule();
					}}
					class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3"
				>
					<input
						bind:value={ruleName}
						placeholder="Ten quy tac"
						class="rounded-md border border-input bg-background px-3 py-2 text-sm"
						required
					/>
					<select
						bind:value={ruleRoleInJob}
						class="rounded-md border border-input bg-background px-3 py-2 text-sm"
					>
						<option value="primary">Chinh (Primary)</option>
						<option value="assistant">Phu (Assistant)</option>
					</select>
					<select
						bind:value={ruleCommissionType}
						class="rounded-md border border-input bg-background px-3 py-2 text-sm"
					>
						<option value="Fixed">Co dinh (Fixed)</option>
						<option value="Percentage">Phan tram (%)</option>
					</select>
					<div>
						<label for="commission-value" class="block text-xs text-muted-foreground">
							Gia tri {ruleCommissionType === 'Percentage' ? '(%)' : '(VND)'}
						</label>
						<input
							id="commission-value"
							bind:value={ruleCommissionValue}
							type="number"
							min="0"
							step={ruleCommissionType === 'Percentage' ? '0.1' : '1000'}
							class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							required
						/>
					</div>
					<input
						bind:value={ruleServiceId}
						placeholder="Service ID (tuy chon)"
						class="rounded-md border border-input bg-background px-3 py-2 text-sm"
					/>
					<select
						bind:value={ruleSkillLevel}
						class="rounded-md border border-input bg-background px-3 py-2 text-sm"
					>
						<option value="">Tat ca cap do</option>
						<option value="Junior">Junior</option>
						<option value="Mid">Mid</option>
						<option value="Senior">Senior</option>
						<option value="Lead">Lead</option>
					</select>
					<div class="flex items-end gap-2 sm:col-span-2 lg:col-span-3">
						<button
							type="submit"
							disabled={loading}
							class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
						>
							Them
						</button>
						<button
							type="button"
							onclick={() => (showCommissionForm = false)}
							class="rounded-md border border-border px-4 py-2 text-sm hover:bg-muted"
						>
							Huy
						</button>
					</div>
				</form>
			</div>
		{/if}

		<div class="mt-6 rounded-lg border border-border">
			<table class="w-full">
				<thead>
					<tr class="border-b border-border bg-muted/50">
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground"
							>Ten quy tac</th
						>
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Vai tro</th>
						<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Loai</th>
						<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Gia tri</th>
						<th class="px-4 py-3 text-center text-sm font-medium text-muted-foreground"
							>Trang thai</th
						>
					</tr>
				</thead>
				<tbody>
					{#if commissionRules.length === 0}
						<tr>
							<td colspan="5" class="px-4 py-8 text-center text-sm text-muted-foreground">
								Chua co quy tac hoa hong nao. Bam "Them quy tac" de bat dau.
							</td>
						</tr>
					{:else}
						{#each commissionRules as rule (rule.id)}
							<tr class="border-b border-border last:border-0 hover:bg-muted/30">
								<td class="px-4 py-3 text-sm font-medium">
									{rule.name}
									{#if rule.skill_level}
										<span
											class="ml-1 inline-block rounded-full px-1.5 py-0.5 text-xs {skillLevelColors[rule.skill_level] ?? 'bg-gray-100 text-gray-700'}"
										>
											{rule.skill_level}
										</span>
									{/if}
								</td>
								<td class="px-4 py-3 text-sm text-muted-foreground">
									{rule.role_in_job === 'primary' ? 'Chinh' : 'Phu'}
								</td>
								<td class="px-4 py-3 text-sm text-muted-foreground">
									{rule.commission_type === 'Fixed' ? 'Co dinh' : 'Phan tram'}
								</td>
								<td class="px-4 py-3 text-right text-sm font-mono">
									{formatCommissionValue(rule.commission_type, rule.commission_value)}
								</td>
								<td class="px-4 py-3 text-center text-sm">
									{#if rule.is_active}
										<span
											class="inline-block rounded-full bg-green-100 px-2 py-0.5 text-xs font-medium text-green-700"
										>
											Hoat dong
										</span>
									{:else}
										<span
											class="inline-block rounded-full bg-gray-100 px-2 py-0.5 text-xs font-medium text-gray-500"
										>
											Tat
										</span>
									{/if}
								</td>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		</div>
	{/if}
</div>
