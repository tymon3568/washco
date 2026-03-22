<script lang="ts">
	import { page } from '$app/state';
	import { api } from '$lib/api/client';
	import { auth } from '$lib/auth.svelte';
	import { toast } from '$lib/toast.svelte';
	import StarRating from '$lib/components/StarRating.svelte';

	let locationId = $derived(page.params.id!);
	let name = $state(auth.user?.name ?? '');
	let phone = $state(auth.user?.phone ?? '');
	let rating = $state(5);
	let comment = $state('');
	let submitting = $state(false);
	let submitted = $state(false);

	async function submit() {
		if (!name.trim()) {
			toast.error('Vui long nhap ho ten');
			return;
		}
		submitting = true;
		try {
			await api.post(`/reviews/public`, {
				location_id: locationId,
				customer_name: name,
				customer_phone: phone || null,
				rating,
				comment: comment || null
			});
			submitted = true;
			toast.success('Cam on ban da danh gia!');
		} catch (e: any) {
			toast.error(e.message || 'Gui danh gia that bai');
		}
		submitting = false;
	}
</script>

<div class="pb-4">
	<a href="/location/{locationId}" class="inline-flex min-h-11 items-center gap-1 text-sm text-primary">
		<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
			<path d="M19 12H5M12 19l-7-7 7-7" />
		</svg>
		Quay lai
	</a>

	{#if submitted}
		<div class="mt-6 rounded-2xl bg-card p-8 text-center shadow-xs">
			<div class="mx-auto flex h-16 w-16 items-center justify-center rounded-full bg-success/10">
				<svg class="h-8 w-8 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path d="M20 6 9 17l-5-5" />
				</svg>
			</div>
			<p class="mt-4 text-lg font-semibold">Cam on ban!</p>
			<p class="mt-1 text-sm text-muted-foreground">Danh gia cua ban da duoc ghi nhan.</p>
			<div class="mt-4">
				<StarRating value={rating} readonly />
			</div>
			<a href="/location/{locationId}" class="mt-6 inline-block rounded-xl bg-primary px-6 py-2.5 text-sm font-medium text-primary-foreground">
				Quay lai tiem
			</a>
		</div>
	{:else}
		<div class="mt-4 rounded-2xl bg-card p-5 shadow-xs">
			<h2 class="text-lg font-semibold">Danh gia dich vu</h2>
			<p class="mt-1 text-sm text-muted-foreground">Y kien cua ban giup tiem cai thien dich vu</p>

			<div class="mt-5 space-y-4">
				<div class="text-center">
					<p class="text-sm font-medium text-muted-foreground">Diem danh gia</p>
					<div class="mt-2 flex justify-center">
						<StarRating bind:value={rating} size="lg" />
					</div>
					<p class="mt-1 text-xs text-muted-foreground">
						{#if rating === 5}Tuyet voi!
						{:else if rating === 4}Rat tot
						{:else if rating === 3}Binh thuong
						{:else if rating === 2}Can cai thien
						{:else}Rat te
						{/if}
					</p>
				</div>

				<div>
					<label for="rv-name" class="text-sm font-medium">Ho ten</label>
					<input
						id="rv-name"
						bind:value={name}
						autocomplete="name"
						placeholder="Nguyen Van A"
						class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm focus:border-primary focus:outline-none"
					/>
				</div>

				<div>
					<label for="rv-phone" class="text-sm font-medium">So dien thoai <span class="text-muted-foreground">(tuy chon)</span></label>
					<input
						id="rv-phone"
						bind:value={phone}
						type="tel"
						autocomplete="tel"
						placeholder="0912 345 678"
						class="mt-1 min-h-12 w-full rounded-xl border border-border bg-background px-4 text-sm focus:border-primary focus:outline-none"
					/>
				</div>

				<div>
					<label for="rv-comment" class="text-sm font-medium">Nhan xet</label>
					<textarea
						id="rv-comment"
						bind:value={comment}
						placeholder="Chia se trai nghiem cua ban..."
						rows={4}
						class="mt-1 w-full rounded-xl border border-border bg-background px-4 py-3 text-sm focus:border-primary focus:outline-none"
					></textarea>
				</div>

				<button
					onclick={submit}
					disabled={submitting || !name.trim()}
					class="min-h-12 w-full rounded-xl bg-primary text-sm font-semibold text-primary-foreground disabled:opacity-50"
				>
					{submitting ? 'Dang gui...' : 'Gui danh gia'}
				</button>
			</div>
		</div>
	{/if}
</div>
