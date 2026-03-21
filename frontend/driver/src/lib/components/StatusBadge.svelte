<script lang="ts">
	let { status }: { status: string } = $props();

	const config: Record<string, { label: string; class: string }> = {
		pending: { label: 'Cho xac nhan', class: 'bg-yellow-100 text-yellow-700' },
		confirmed: { label: 'Da xac nhan', class: 'bg-blue-100 text-blue-700' },
		waiting: { label: 'Dang cho', class: 'bg-orange-100 text-orange-700' },
		in_progress: { label: 'Dang rua', class: 'bg-primary/10 text-primary' },
		completed: { label: 'Hoan thanh', class: 'bg-green-100 text-green-700' },
		cancelled: { label: 'Da huy', class: 'bg-red-100 text-red-700' },
		no_show: { label: 'Khong den', class: 'bg-gray-100 text-gray-600' }
	};

	let resolved = $derived(config[status] ?? { label: status, class: 'bg-muted text-muted-foreground' });
</script>

<span class="inline-flex items-center rounded-full px-2.5 py-1 text-xs font-medium {resolved.class}">
	{resolved.label}
</span>
