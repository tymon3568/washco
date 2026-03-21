<script lang="ts">
	import { auth } from '$lib/auth.svelte';
	import { goto } from '$app/navigation';

	let step: 'phone' | 'otp' | 'register' = $state('phone');
	let phone = $state('');
	let otpCode = $state('');
	let businessName = $state('');
	let ownerName = $state('');
	let error = $state('');
	let loading = $state(false);

	function validatePhone(p: string): boolean {
		return /^0[0-9]{9}$/.test(p.replace(/\s/g, ''));
	}

	async function handleRequestOtp() {
		error = '';
		if (!validatePhone(phone)) {
			error = 'So dien thoai khong hop le (VD: 0901234567)';
			return;
		}
		loading = true;
		try {
			await auth.requestOtp(phone.replace(/\s/g, ''));
			step = 'otp';
		} catch (e: any) {
			if (e.message?.includes('not found')) {
				step = 'register';
			} else {
				error = e.message;
			}
		}
		loading = false;
	}

	async function handleVerifyOtp() {
		error = '';
		loading = true;
		try {
			await auth.verifyOtp(phone, otpCode);
			goto('/');
		} catch (e: any) {
			error = e.message;
		}
		loading = false;
	}

	async function handleRegister() {
		error = '';
		if (!validatePhone(phone)) {
			error = 'So dien thoai khong hop le (VD: 0901234567)';
			return;
		}
		if (!ownerName.trim()) {
			error = 'Vui long nhap ho ten';
			return;
		}
		if (!businessName.trim()) {
			error = 'Vui long nhap ten doanh nghiep';
			return;
		}
		loading = true;
		try {
			await auth.register(phone.replace(/\s/g, ''), businessName.trim(), ownerName.trim());
			await auth.requestOtp(phone.replace(/\s/g, ''));
			step = 'otp';
		} catch (e: any) {
			error = e.message;
		}
		loading = false;
	}
</script>

<div class="flex min-h-screen items-center justify-center bg-background">
	<div class="w-full max-w-sm space-y-6 px-4">
		<div class="text-center">
			<h1 class="text-2xl font-bold text-primary">WashCo</h1>
			<p class="mt-2 text-sm text-muted-foreground">Dang nhap quan ly tiem rua xe</p>
		</div>

		{#if error}
			<div class="rounded-md bg-destructive/10 p-3 text-sm text-destructive">{error}</div>
		{/if}

		{#if step === 'phone'}
			<form onsubmit={(e) => { e.preventDefault(); handleRequestOtp(); }} class="space-y-4">
				<div>
					<label for="phone" class="block text-sm font-medium">So dien thoai</label>
					<input
						id="phone"
						type="tel"
						bind:value={phone}
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-ring focus:outline-none"
						placeholder="0901234567"
						required
					/>
				</div>
				<button
					type="submit"
					disabled={loading || !phone}
					class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
				>
					{loading ? 'Dang xu ly...' : 'Tiep tuc'}
				</button>
			</form>
			<p class="text-center text-xs text-muted-foreground">
				Chua co tai khoan?
				<button onclick={() => (step = 'register')} class="text-primary hover:underline">Dang ky</button>
			</p>
		{:else if step === 'otp'}
			<form onsubmit={(e) => { e.preventDefault(); handleVerifyOtp(); }} class="space-y-4">
				<p class="text-sm text-muted-foreground">Ma OTP da gui den <strong>{phone}</strong></p>
				<div>
					<label for="otp" class="block text-sm font-medium">Ma OTP</label>
					<input
						id="otp"
						type="text"
						bind:value={otpCode}
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-center text-lg tracking-widest font-mono focus:ring-2 focus:ring-ring focus:outline-none"
						placeholder="000000"
						maxlength="6"
						required
					/>
				</div>
				<button
					type="submit"
					disabled={loading || otpCode.length !== 6}
					class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
				>
					{loading ? 'Dang xac thuc...' : 'Xac thuc'}
				</button>
				<button onclick={() => (step = 'phone')} class="w-full text-sm text-muted-foreground hover:text-foreground">
					Quay lai
				</button>
			</form>
		{:else}
			<form onsubmit={(e) => { e.preventDefault(); handleRegister(); }} class="space-y-4">
				<div>
					<label for="reg-phone" class="block text-sm font-medium">So dien thoai</label>
					<input
						id="reg-phone"
						type="tel"
						bind:value={phone}
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-ring focus:outline-none"
						required
					/>
				</div>
				<div>
					<label for="owner-name" class="block text-sm font-medium">Ho ten chu tiem</label>
					<input
						id="owner-name"
						type="text"
						bind:value={ownerName}
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-ring focus:outline-none"
						required
					/>
				</div>
				<div>
					<label for="biz-name" class="block text-sm font-medium">Ten doanh nghiep</label>
					<input
						id="biz-name"
						type="text"
						bind:value={businessName}
						class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-ring focus:outline-none"
						required
					/>
				</div>
				<button
					type="submit"
					disabled={loading}
					class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
				>
					{loading ? 'Dang dang ky...' : 'Dang ky'}
				</button>
				<button onclick={() => (step = 'phone')} class="w-full text-sm text-muted-foreground hover:text-foreground">
					Da co tai khoan? Dang nhap
				</button>
			</form>
		{/if}
	</div>
</div>
