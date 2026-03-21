<script lang="ts">
	import { auth } from '$lib/auth.svelte';
	import { favorites } from '$lib/favorites.svelte';
	import { formatPhone } from '$lib/utils/format';

	let showInstallHint = $state(false);
	let deferredPrompt: any = $state(null);

	$effect(() => {
		if (typeof window !== 'undefined') {
			window.addEventListener('beforeinstallprompt', (e: any) => {
				e.preventDefault();
				deferredPrompt = e;
				showInstallHint = true;
			});
		}
	});

	async function installApp() {
		if (!deferredPrompt) return;
		deferredPrompt.prompt();
		const { outcome } = await deferredPrompt.userChoice;
		if (outcome === 'accepted') {
			showInstallHint = false;
		}
		deferredPrompt = null;
	}

	const menuItems = [
		{ icon: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z', label: 'Lich su dat lich', href: '/history' },
		{ icon: 'M20.84 4.61a5.5 5.5 0 00-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 00-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 000-7.78z', label: 'Yeu thich', href: '/favorites', badge: favorites.ids.length > 0 ? `${favorites.ids.length}` : undefined }
	];
</script>

<div class="pb-4">
	<h1 class="text-xl font-semibold">Tai khoan</h1>

	<!-- User info -->
	<div class="mt-4 rounded-2xl bg-card p-5 shadow-xs">
		{#if auth.isAuthenticated}
			<div class="flex items-center gap-4">
				<div class="flex h-14 w-14 items-center justify-center rounded-full bg-primary/10 text-xl font-bold text-primary">
					{(auth.user?.name ?? auth.user?.phone ?? '?').charAt(0).toUpperCase()}
				</div>
				<div>
					<p class="font-semibold">{auth.user?.name ?? 'Tai xe'}</p>
					<p class="text-sm text-muted-foreground">{formatPhone(auth.user?.phone ?? '')}</p>
				</div>
			</div>
		{:else}
			<div class="text-center">
				<div class="mx-auto flex h-14 w-14 items-center justify-center rounded-full bg-muted">
					<svg class="h-7 w-7 text-muted-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2M12 11a4 4 0 100-8 4 4 0 000 8z" />
					</svg>
				</div>
				<p class="mt-3 font-medium">Dang nhap de su dung day du tinh nang</p>
				<a href="/login" class="mt-3 inline-block rounded-xl bg-primary px-6 py-2.5 text-sm font-semibold text-primary-foreground">
					Dang nhap
				</a>
			</div>
		{/if}
	</div>

	<!-- Install PWA -->
	{#if showInstallHint}
		<div class="mt-3 rounded-2xl border border-primary/20 bg-primary/5 p-4">
			<div class="flex items-start gap-3">
				<div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-primary/10">
					<svg class="h-5 w-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
						<path d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
					</svg>
				</div>
				<div class="flex-1">
					<p class="text-sm font-medium">Cai dat WashCo</p>
					<p class="mt-0.5 text-xs text-muted-foreground">Them vao man hinh chinh de truy cap nhanh hon</p>
				</div>
				<button
					onclick={installApp}
					class="shrink-0 rounded-lg bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground"
				>
					Cai dat
				</button>
			</div>
		</div>
	{/if}

	<!-- Menu -->
	<div class="mt-3 rounded-2xl bg-card shadow-xs">
		{#each menuItems as item (item.href)}
			<a href={item.href} class="flex items-center gap-3 border-b border-border px-5 py-4 last:border-0">
				<svg class="h-5 w-5 text-muted-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path d={item.icon} />
				</svg>
				<span class="flex-1 text-sm font-medium">{item.label}</span>
				{#if item.badge}
					<span class="rounded-full bg-primary/10 px-2 py-0.5 text-xs font-medium text-primary">{item.badge}</span>
				{/if}
				<svg class="h-4 w-4 text-muted-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path d="m9 18 6-6-6-6" />
				</svg>
			</a>
		{/each}
	</div>

	<!-- App info -->
	<div class="mt-3 rounded-2xl bg-card p-5 shadow-xs">
		<h3 class="text-sm font-medium">Thong tin ung dung</h3>
		<div class="mt-2 space-y-2 text-xs text-muted-foreground">
			<div class="flex justify-between">
				<span>Phien ban</span>
				<span>1.0.0</span>
			</div>
			<div class="flex justify-between">
				<span>Ho tro</span>
				<span class="text-primary">support@washco.vn</span>
			</div>
		</div>
	</div>

	<!-- Logout -->
	{#if auth.isAuthenticated}
		<button
			onclick={() => auth.logout()}
			class="mt-3 w-full rounded-2xl border border-destructive/20 bg-card py-3 text-sm font-medium text-destructive shadow-xs"
		>
			Dang xuat
		</button>
	{/if}
</div>
