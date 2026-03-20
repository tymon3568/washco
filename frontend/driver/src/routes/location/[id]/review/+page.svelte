<script lang="ts">
	import { page } from '$app/state';
	import { api } from '$lib/api/client';

	let locationId = $derived(page.params.id!);
	let name = $state('');
	let phone = $state('');
	let rating = $state(5);
	let comment = $state('');
	let submitting = $state(false);
	let submitted = $state(false);
	let error = $state('');

	async function submit() {
		submitting = true;
		error = '';
		try {
			await api.post('/reviews', {
				location_id: locationId,
				customer_name: name,
				customer_phone: phone || null,
				rating,
				comment: comment || null
			});
			submitted = true;
		} catch (e: any) {
			error = e.message || 'Gui danh gia that bai';
		}
		submitting = false;
	}
</script>

<div>
	<a href="/location/{locationId}" class="text-sm text-primary">&larr; Quay lai</a>

	{#if submitted}
		<div class="mt-8 rounded-xl bg-card p-6 text-center">
			<p class="text-2xl">⭐</p>
			<p class="mt-2 font-semibold">Cam on ban!</p>
			<p class="mt-1 text-sm text-muted-foreground">Danh gia cua ban da duoc ghi nhan.</p>
			<a href="/location/{locationId}" class="mt-4 inline-block text-sm text-primary">Quay lai tiem</a>
		</div>
	{:else}
		<div class="mt-4 rounded-xl bg-card p-4">
			<h2 class="text-lg font-semibold">Danh gia dich vu</h2>

			{#if error}
				<p class="mt-2 text-xs text-red-400">{error}</p>
			{/if}

			<div class="mt-4 space-y-4">
				<div>
					<p class="text-sm text-muted-foreground">Diem danh gia</p>
					<div class="mt-1 flex gap-2">
						{#each [1, 2, 3, 4, 5] as star}
							<button
								onclick={() => { rating = star; }}
								class="text-2xl {star <= rating ? 'text-yellow-400' : 'text-muted-foreground/30'}"
							>
								★
							</button>
						{/each}
					</div>
				</div>

				<input
					bind:value={name}
					placeholder="Ho ten"
					class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm"
				/>

				<input
					bind:value={phone}
					placeholder="So dien thoai (tuy chon)"
					type="tel"
					class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm"
				/>

				<textarea
					bind:value={comment}
					placeholder="Nhan xet cua ban..."
					rows="3"
					class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm"
				></textarea>

				<button
					onclick={submit}
					disabled={submitting || !name}
					class="w-full rounded-lg bg-primary px-4 py-2.5 text-sm font-medium text-primary-foreground disabled:opacity-50"
				>
					{submitting ? 'Dang gui...' : 'Gui danh gia'}
				</button>
			</div>
		</div>
	{/if}
</div>
