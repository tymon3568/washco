<script lang="ts">
	import { goto } from '$app/navigation';
	import { auth } from '$lib/auth.svelte';
	import { ApiError } from '$lib/api/client';
	import { toast } from '$lib/toast.svelte';

	let step = $state<'phone' | 'otp' | 'register'>('phone');
	let phone = $state('');
	let otp = $state('');
	let name = $state('');
	let loading = $state(false);
	let error = $state('');

	async function handleRequestOtp() {
		if (!phone.match(/^0[0-9]{9}$/)) {
			error = 'So dien thoai khong hop le (10 so, bat dau bang 0)';
			return;
		}
		loading = true;
		error = '';
		try {
			await auth.requestOtp(phone);
			step = 'otp';
			toast.info('Ma OTP da duoc gui');
		} catch {
			error = auth.error ?? 'Gui OTP that bai';
		}
		loading = false;
	}

	async function handleVerifyOtp() {
		if (otp.length !== 6) {
			error = 'Ma OTP phai co 6 so';
			return;
		}
		loading = true;
		error = '';
		try {
			await auth.verifyOtp(phone, otp);
			toast.success('Dang nhap thanh cong!');
			goto('/');
		} catch (err) {
			if (err instanceof ApiError && err.code === 404) {
				step = 'register';
				error = '';
			} else {
				error = auth.error ?? 'Xac thuc that bai';
			}
		}
		loading = false;
	}

	async function handleRegister() {
		if (!name.trim()) {
			error = 'Vui long nhap ho ten';
			return;
		}
		loading = true;
		error = '';
		try {
			await auth.register(phone, name.trim());
			toast.success('Dang ky thanh cong! Vui long nhap OTP.');
			await auth.requestOtp(phone);
			step = 'otp';
			otp = '';
		} catch {
			error = auth.error ?? 'Dang ky that bai';
		}
		loading = false;
	}
</script>

<div class="flex min-h-screen flex-col items-center justify-center px-4">
	<div class="w-full max-w-sm">
		<!-- Logo -->
		<div class="text-center">
			<h1 class="text-3xl font-bold text-primary">WashCo</h1>
			<p class="mt-1 text-sm text-muted-foreground">Tim va dat lich rua xe</p>
		</div>

		<div class="mt-8 rounded-2xl bg-card p-6 shadow-sm">
			{#if step === 'phone'}
				<h2 class="text-lg font-semibold">Dang nhap</h2>
				<p class="mt-1 text-sm text-muted-foreground">Nhap so dien thoai de tiep tuc</p>

				{#if error}
					<p class="mt-3 rounded-lg bg-destructive/10 px-3 py-2 text-xs text-destructive">{error}</p>
				{/if}

				<form onsubmit={(e) => { e.preventDefault(); handleRequestOtp(); }} class="mt-4 space-y-4">
					<div>
						<label for="phone" class="text-sm font-medium">So dien thoai</label>
						<input
							id="phone"
							bind:value={phone}
							type="tel"
							inputmode="tel"
							autocomplete="tel"
							placeholder="0912 345 678"
							class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-base focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
						/>
					</div>
					<button
						type="submit"
						disabled={loading || !phone.trim()}
						class="min-h-12 w-full rounded-xl bg-primary text-sm font-semibold text-primary-foreground disabled:opacity-50"
					>
						{loading ? 'Dang gui...' : 'Gui ma OTP'}
					</button>
				</form>

				<p class="mt-4 text-center text-xs text-muted-foreground">
					Tiep tuc la ban dong y voi dieu khoan su dung cua WashCo
				</p>

			{:else if step === 'otp'}
				<div class="flex items-center gap-2">
					<button onclick={() => { step = 'phone'; error = ''; }} class="text-muted-foreground" aria-label="Quay lai">
						<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
							<path d="M19 12H5M12 19l-7-7 7-7" />
						</svg>
					</button>
					<h2 class="text-lg font-semibold">Nhap ma OTP</h2>
				</div>
				<p class="mt-1 text-sm text-muted-foreground">Ma da gui den {phone}</p>

				{#if error}
					<p class="mt-3 rounded-lg bg-destructive/10 px-3 py-2 text-xs text-destructive">{error}</p>
				{/if}

				<form onsubmit={(e) => { e.preventDefault(); handleVerifyOtp(); }} class="mt-4 space-y-4">
					<input
						bind:value={otp}
						type="text"
						inputmode="numeric"
						autocomplete="one-time-code"
						maxlength={6}
						placeholder="000000"
						class="min-h-14 w-full rounded-xl border border-border bg-background text-center font-mono text-2xl tracking-[0.5em] focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
					/>
					<button
						type="submit"
						disabled={loading || otp.length !== 6}
						class="min-h-12 w-full rounded-xl bg-primary text-sm font-semibold text-primary-foreground disabled:opacity-50"
					>
						{loading ? 'Dang xac thuc...' : 'Xac nhan'}
					</button>
				</form>

				<button
					onclick={handleRequestOtp}
					disabled={loading}
					class="mt-3 w-full text-center text-sm text-primary disabled:opacity-50"
				>
					Gui lai ma OTP
				</button>

			{:else if step === 'register'}
				<div class="flex items-center gap-2">
					<button onclick={() => { step = 'phone'; error = ''; }} class="text-muted-foreground" aria-label="Quay lai">
						<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
							<path d="M19 12H5M12 19l-7-7 7-7" />
						</svg>
					</button>
					<h2 class="text-lg font-semibold">Tao tai khoan</h2>
				</div>
				<p class="mt-1 text-sm text-muted-foreground">Nhap thong tin de hoan tat dang ky</p>

				{#if error}
					<p class="mt-3 rounded-lg bg-destructive/10 px-3 py-2 text-xs text-destructive">{error}</p>
				{/if}

				<form onsubmit={(e) => { e.preventDefault(); handleRegister(); }} class="mt-4 space-y-4">
					<div>
						<label for="name" class="text-sm font-medium">Ho va ten</label>
						<input
							id="name"
							bind:value={name}
							type="text"
							autocomplete="name"
							placeholder="Nguyen Van A"
							class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
						/>
					</div>

					<div class="rounded-lg bg-muted p-3">
						<p class="text-xs text-muted-foreground">So dien thoai: <span class="font-medium text-foreground">{phone}</span></p>
					</div>

					<button
						type="submit"
						disabled={loading || !name.trim()}
						class="min-h-12 w-full rounded-xl bg-primary text-sm font-semibold text-primary-foreground disabled:opacity-50"
					>
						{loading ? 'Dang dang ky...' : 'Dang ky'}
					</button>
				</form>
			{/if}
		</div>

		<!-- Skip login -->
		<div class="mt-4 text-center">
			<a href="/" class="text-sm text-muted-foreground underline">Bo qua, xem truoc</a>
		</div>
	</div>
</div>
