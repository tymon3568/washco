<script lang="ts">
	import { api, ApiError } from '$lib/api/client';
	import { toast } from '$lib/toast.svelte';
	import { untrack } from 'svelte';
	import type {
		TemplateResponse,
		NotificationResponse,
		CreateTemplateRequest,
		UpdateTemplateRequest,
		SendNotificationRequest
	} from '$lib/api/types';

	// ── State ──
	let activeTab = $state<'templates' | 'history'>('templates');
	let templates: TemplateResponse[] = $state([]);
	let notifications: NotificationResponse[] = $state([]);
	let loading = $state(false);

	// Template form
	let showTemplateForm = $state(false);
	let editingTemplateId: string | null = $state(null);
	let deleteConfirmId: string | null = $state(null);
	let templateType = $state('');
	let templateChannel = $state('sms');
	let templateSubject = $state('');
	let templateBody = $state('');
	let templateIsActive = $state(true);

	// Send form
	let showSendForm = $state(false);
	let sendTemplateType = $state('');
	let sendChannel = $state('sms');
	let sendRecipient = $state('');
	let sendPayload = $state('');

	const tabs = [
		{ key: 'templates' as const, label: 'Mẫu thông báo' },
		{ key: 'history' as const, label: 'Lịch sử gửi' }
	];

	const channelLabels: Record<string, { label: string; classes: string }> = {
		sms: { label: 'SMS', classes: 'bg-blue-100 text-blue-700' },
		email: { label: 'Email', classes: 'bg-amber-100 text-amber-700' },
		push: { label: 'Push', classes: 'bg-green-100 text-green-700' },
		zalo: { label: 'Zalo', classes: 'bg-blue-100 text-blue-700' }
	};

	const statusLabels: Record<string, { label: string; classes: string }> = {
		pending: { label: 'Chờ gửi', classes: 'bg-amber-100 text-amber-700' },
		sent: { label: 'Đã gửi', classes: 'bg-green-100 text-green-700' },
		failed: { label: 'Thất bại', classes: 'bg-red-100 text-red-700' }
	};

	function formatDateTime(dateStr: string): string {
		const d = new Date(dateStr);
		return d.toLocaleString('vi-VN', {
			day: '2-digit',
			month: '2-digit',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	// ── Data loading ──
	$effect(() => {
		untrack(() => {
			refreshTemplates();
			refreshNotifications();
		});
	});

	async function refreshTemplates() {
		try {
			templates = await api.get<TemplateResponse[]>('/notifications/templates');
		} catch {
			// API not available
		}
	}

	async function refreshNotifications() {
		try {
			notifications = await api.get<NotificationResponse[]>('/notifications');
		} catch {
			// API not available
		}
	}

	// ── Template CRUD ──
	function openAddTemplate() {
		editingTemplateId = null;
		templateType = '';
		templateChannel = 'sms';
		templateSubject = '';
		templateBody = '';
		templateIsActive = true;
		showTemplateForm = true;
	}

	function openEditTemplate(t: TemplateResponse) {
		editingTemplateId = t.id;
		templateType = t.template_type;
		templateChannel = t.channel;
		templateSubject = t.subject ?? '';
		templateBody = t.body_template;
		templateIsActive = t.is_active;
		showTemplateForm = true;
	}

	async function handleTemplateSubmit() {
		loading = true;
		try {
			if (editingTemplateId) {
				const body: UpdateTemplateRequest = {
					template_type: templateType,
					channel: templateChannel,
					subject: templateSubject || undefined,
					body_template: templateBody,
					is_active: templateIsActive
				};
				await api.put(`/notifications/templates/${editingTemplateId}`, body);
				toast.success('Đã cập nhật mẫu thông báo');
			} else {
				const body: CreateTemplateRequest = {
					template_type: templateType,
					channel: templateChannel,
					subject: templateSubject || undefined,
					body_template: templateBody
				};
				await api.post('/notifications/templates', body);
				toast.success('Đã thêm mẫu thông báo');
			}
			showTemplateForm = false;
			await refreshTemplates();
		} catch (e: unknown) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
		loading = false;
	}

	async function deleteTemplate(id: string) {
		try {
			await api.del(`/notifications/templates/${id}`);
			deleteConfirmId = null;
			toast.success('Đã xóa mẫu thông báo');
			await refreshTemplates();
		} catch (e: unknown) {
			toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
		}
	}

	// ── Send notification ──
	function openSendForm() {
		sendTemplateType = templates.length > 0 ? templates[0].template_type : '';
		sendChannel = 'sms';
		sendRecipient = '';
		sendPayload = '';
		showSendForm = true;
	}

	async function handleSendSubmit() {
		loading = true;
		try {
			const body: SendNotificationRequest = {
				recipient_phone: sendRecipient,
				template_type: sendTemplateType,
				channel: sendChannel,
				payload: sendPayload ? JSON.parse(sendPayload) : undefined
			};
			await api.post('/notifications/send', body);
			toast.success('Đã gửi thông báo');
			showSendForm = false;
			await refreshNotifications();
		} catch (e: unknown) {
			if (e instanceof SyntaxError) {
				toast.error('Payload JSON không hợp lệ');
			} else {
				toast.error(e instanceof ApiError ? e.message : 'Có lỗi xảy ra');
			}
		}
		loading = false;
	}
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Thông báo</h1>
			<p class="mt-1 text-sm text-muted-foreground">Quản lý mẫu thông báo và lịch sử gửi.</p>
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

	<!-- Tab 1: Templates -->
	{#if activeTab === 'templates'}
		<div class="mt-4 flex justify-end">
			<button
				onclick={openAddTemplate}
				class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
			>
				Thêm mẫu
			</button>
		</div>

		<!-- Template form -->
		{#if showTemplateForm}
			<div class="mt-4 rounded-lg border border-border bg-card p-4">
				<h3 class="text-sm font-medium">{editingTemplateId ? 'Sửa mẫu thông báo' : 'Thêm mẫu thông báo mới'}</h3>
				<form
					onsubmit={(e) => {
						e.preventDefault();
						handleTemplateSubmit();
					}}
					class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3"
				>
					<label class="block">
						<span class="block text-xs text-muted-foreground">Loại mẫu</span>
						<input
							bind:value={templateType}
							placeholder="VD: welcome, otp, booking_confirm"
							class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							required
						/>
					</label>
					<label class="block">
						<span class="block text-xs text-muted-foreground">Kênh</span>
						<select
							bind:value={templateChannel}
							class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
						>
							<option value="sms">SMS</option>
							<option value="email">Email</option>
							<option value="push">Push</option>
							<option value="zalo">Zalo</option>
						</select>
					</label>
					<label class="block">
						<span class="block text-xs text-muted-foreground">Chủ đề (tùy chọn)</span>
						<input
							bind:value={templateSubject}
							placeholder="Chủ đề email/thông báo"
							class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
						/>
					</label>
					<label class="block sm:col-span-2 lg:col-span-3">
						<span class="block text-xs text-muted-foreground">Nội dung mẫu</span>
						<textarea
							bind:value={templateBody}
							placeholder={'Xin chào {{name}}, mã OTP của bạn là {{otp}}'}
							rows={4}
							class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							required
						></textarea>
					</label>
					{#if editingTemplateId}
						<label class="flex items-center gap-2">
							<input
								type="checkbox"
								bind:checked={templateIsActive}
								class="accent-primary"
							/>
							<span class="text-sm">Kích hoạt</span>
						</label>
					{/if}
					<div class="flex items-end gap-2 sm:col-span-2 lg:col-span-3">
						<button
							type="submit"
							disabled={loading}
							class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
						>
							{editingTemplateId ? 'Cập nhật' : 'Thêm'}
						</button>
						<button
							type="button"
							onclick={() => (showTemplateForm = false)}
							class="rounded-md border border-border px-4 py-2 text-sm hover:bg-muted"
						>
							Hủy
						</button>
					</div>
				</form>
			</div>
		{/if}

		<!-- Templates table -->
		{#if templates.length === 0}
			<div class="mt-6 rounded-lg border border-border bg-card p-8 text-center text-sm text-muted-foreground">
				Chưa có mẫu thông báo nào. Bấm "Thêm mẫu" để bắt đầu.
			</div>
		{:else}
			<div class="mt-4 overflow-x-auto rounded-lg border border-border bg-card shadow-xs">
				<table class="min-w-full divide-y divide-border">
					<thead class="bg-muted/50">
						<tr>
							<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Loại mẫu</th>
							<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Kênh</th>
							<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Chủ đề</th>
							<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Trạng thái</th>
							<th class="px-4 py-3 text-right text-sm font-medium text-muted-foreground">Thao tác</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-border">
						{#each templates as tmpl (tmpl.id)}
							{@const ch = channelLabels[tmpl.channel] ?? { label: tmpl.channel, classes: 'bg-gray-100 text-gray-600' }}
							<tr>
								<td class="px-4 py-3 text-sm font-medium">{tmpl.template_type}</td>
								<td class="px-4 py-3 text-sm">
									<span class="inline-block rounded-full px-2 py-0.5 text-xs font-medium {ch.classes}">
										{ch.label}
									</span>
								</td>
								<td class="px-4 py-3 text-sm text-muted-foreground">{tmpl.subject ?? '—'}</td>
								<td class="px-4 py-3 text-sm">
									{#if tmpl.is_active}
										<span class="inline-block rounded-full px-2 py-0.5 text-xs font-medium bg-green-100 text-green-700">Hoạt động</span>
									{:else}
										<span class="inline-block rounded-full px-2 py-0.5 text-xs font-medium bg-gray-100 text-gray-600">Tắt</span>
									{/if}
								</td>
								<td class="px-4 py-3 text-right text-sm">
									<div class="flex items-center justify-end gap-2">
										<button
											onclick={() => openEditTemplate(tmpl)}
											class="text-xs text-primary hover:underline"
										>
											Sửa
										</button>
										{#if deleteConfirmId === tmpl.id}
											<span class="text-xs text-muted-foreground">Chắc chưa?</span>
											<button
												onclick={() => deleteTemplate(tmpl.id)}
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
												onclick={() => (deleteConfirmId = tmpl.id)}
												class="text-xs text-destructive hover:underline"
											>
												Xóa
											</button>
										{/if}
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	{/if}

	<!-- Tab 2: History -->
	{#if activeTab === 'history'}
		<div class="mt-4 flex justify-end">
			<button
				onclick={openSendForm}
				class="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
			>
				Gửi thông báo
			</button>
		</div>

		<!-- Send form -->
		{#if showSendForm}
			<div class="mt-4 rounded-lg border border-border bg-card p-4">
				<h3 class="text-sm font-medium">Gửi thông báo mới</h3>
				<form
					onsubmit={(e) => {
						e.preventDefault();
						handleSendSubmit();
					}}
					class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3"
				>
					<label class="block">
						<span class="block text-xs text-muted-foreground">Loại mẫu</span>
						<input
							bind:value={sendTemplateType}
							placeholder="VD: welcome, otp"
							class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							required
						/>
					</label>
					<label class="block">
						<span class="block text-xs text-muted-foreground">Kênh</span>
						<select
							bind:value={sendChannel}
							class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
						>
							<option value="sms">SMS</option>
							<option value="email">Email</option>
							<option value="push">Push</option>
							<option value="zalo">Zalo</option>
						</select>
					</label>
					<label class="block">
						<span class="block text-xs text-muted-foreground">Số điện thoại</span>
						<input
							bind:value={sendRecipient}
							placeholder="0901234567"
							class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
							required
						/>
					</label>
					<label class="block sm:col-span-2 lg:col-span-3">
						<span class="block text-xs text-muted-foreground">Payload JSON (tùy chọn)</span>
						<textarea
							bind:value={sendPayload}
							placeholder={'{"name": "Nguyen Van A", "otp": "123456"}'}
							rows={3}
							class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 font-mono text-sm"
						></textarea>
					</label>
					<div class="flex items-end gap-2 sm:col-span-2 lg:col-span-3">
						<button
							type="submit"
							disabled={loading}
							class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
						>
							Gửi
						</button>
						<button
							type="button"
							onclick={() => (showSendForm = false)}
							class="rounded-md border border-border px-4 py-2 text-sm hover:bg-muted"
						>
							Hủy
						</button>
					</div>
				</form>
			</div>
		{/if}

		<!-- Notifications table -->
		{#if notifications.length === 0}
			<div class="mt-6 rounded-lg border border-border bg-card p-8 text-center text-sm text-muted-foreground">
				Chưa có thông báo nào được gửi.
			</div>
		{:else}
			<div class="mt-4 overflow-x-auto rounded-lg border border-border bg-card shadow-xs">
				<table class="min-w-full divide-y divide-border">
					<thead class="bg-muted/50">
						<tr>
							<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Người nhận</th>
							<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Kênh</th>
							<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Loại mẫu</th>
							<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Trạng thái</th>
							<th class="px-4 py-3 text-left text-sm font-medium text-muted-foreground">Thời gian</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-border">
						{#each notifications as notif (notif.id)}
							{@const ch = channelLabels[notif.channel] ?? { label: notif.channel, classes: 'bg-gray-100 text-gray-600' }}
							{@const st = statusLabels[notif.status] ?? { label: notif.status, classes: 'bg-gray-100 text-gray-600' }}
							<tr>
								<td class="px-4 py-3 text-sm font-medium">{notif.recipient_phone}</td>
								<td class="px-4 py-3 text-sm">
									<span class="inline-block rounded-full px-2 py-0.5 text-xs font-medium {ch.classes}">
										{ch.label}
									</span>
								</td>
								<td class="px-4 py-3 text-sm text-muted-foreground">{notif.template_type}</td>
								<td class="px-4 py-3 text-sm">
									<span class="inline-block rounded-full px-2 py-0.5 text-xs font-medium {st.classes}">
										{st.label}
									</span>
								</td>
								<td class="px-4 py-3 text-sm text-muted-foreground">
									{formatDateTime(notif.sent_at ?? notif.created_at)}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	{/if}
</div>
