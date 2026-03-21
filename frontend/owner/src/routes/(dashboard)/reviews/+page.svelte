<script lang="ts">
	import { api } from '$lib/api/client';

	interface Location {
		id: string;
		name: string;
	}

	interface Review {
		id: string;
		location_id: string;
		customer_name: string;
		rating: number;
		comment: string | null;
		reply: string | null;
		replied_at: string | null;
		created_at: string;
	}

	interface ReviewSummary {
		average_rating: number;
		total_count: number;
		distribution: number[];
	}

	let locations: Location[] = $state([]);
	let locationId = $state('');
	let reviews: Review[] = $state([]);
	let summary = $state<ReviewSummary | null>(null);
	let loading = $state(false);
	let offset = $state(0);
	let hasMore = $state(true);

	let replyingTo = $state('');
	let replyText = $state('');
	let submittingReply = $state(false);

	const LIMIT = 20;

	let maxDistribution = $derived(
		summary !== null ? Math.max(...summary.distribution, 1) : 1
	);

	$effect(() => {
		loadLocations();
	});

	async function loadLocations() {
		try {
			locations = await api.get<Location[]>('/locations');
			if (locations.length > 0) {
				locationId = locations[0].id;
				await loadReviews(true);
			}
		} catch {
			// API not available
		}
	}

	async function onLocationChange() {
		await loadReviews(true);
	}

	async function loadReviews(reset = false) {
		if (!locationId) return;
		loading = true;

		if (reset) {
			offset = 0;
			reviews = [];
		}

		try {
			const [reviewList, reviewSummary] = await Promise.all([
				api.get<Review[]>(`/reviews/locations/${locationId}?limit=${LIMIT}&offset=${offset}`),
				reset
					? api.get<ReviewSummary>(`/reviews/locations/${locationId}/summary`)
					: Promise.resolve(summary)
			]);

			if (reset) {
				reviews = reviewList;
			} else {
				reviews = [...reviews, ...reviewList];
			}

			summary = reviewSummary;
			hasMore = reviewList.length === LIMIT;
		} catch {
			// ignore
		} finally {
			loading = false;
		}
	}

	async function loadMore() {
		offset += LIMIT;
		await loadReviews(false);
	}

	function startReply(reviewId: string) {
		replyingTo = reviewId;
		replyText = '';
	}

	function cancelReply() {
		replyingTo = '';
		replyText = '';
	}

	async function submitReply(reviewId: string) {
		if (!replyText.trim()) return;
		submittingReply = true;

		try {
			await api.put(`/reviews/${reviewId}/reply`, { reply: replyText.trim() });
			const idx = reviews.findIndex((r) => r.id === reviewId);
			if (idx !== -1) {
				reviews[idx].reply = replyText.trim();
				reviews[idx].replied_at = new Date().toISOString();
			}
			replyingTo = '';
			replyText = '';
		} catch {
			// ignore
		} finally {
			submittingReply = false;
		}
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('vi-VN', {
			day: '2-digit',
			month: '2-digit',
			year: 'numeric'
		});
	}

	function renderStars(rating: number): { filled: string; empty: string } {
		return {
			filled: '★'.repeat(rating),
			empty: '☆'.repeat(5 - rating)
		};
	}
</script>

<div>
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold">Đánh giá</h1>
			<p class="mt-1 text-sm text-muted-foreground">Quản lý đánh giá của khách hàng.</p>
		</div>
		{#if locations.length > 1}
			<select
				bind:value={locationId}
				onchange={onLocationChange}
				class="rounded-md border border-input bg-background px-3 py-2 text-sm"
			>
				{#each locations as loc (loc.id)}
					<option value={loc.id}>{loc.name}</option>
				{/each}
			</select>
		{/if}
	</div>

	<!-- Summary Card -->
	{#if summary}
		<div class="mt-6 rounded-lg border border-border bg-card p-6">
			<div class="flex flex-col gap-6 sm:flex-row sm:items-start">
				<!-- Average Rating -->
				<div class="flex flex-col items-center sm:items-start">
					<p class="text-4xl font-bold">{summary.average_rating.toFixed(1)}</p>
					<p class="mt-1 text-lg text-yellow-400">
						{renderStars(Math.round(summary.average_rating)).filled}<span class="text-muted-foreground">{renderStars(Math.round(summary.average_rating)).empty}</span>
					</p>
					<p class="mt-1 text-sm text-muted-foreground">{summary.total_count} đánh giá</p>
				</div>

				<!-- Distribution -->
				<div class="flex-1 space-y-1.5">
					{#each [5, 4, 3, 2, 1] as star (star)}
						{@const count = summary.distribution[star - 1] ?? 0}
						<div class="flex items-center gap-2">
							<span class="w-8 text-right text-sm text-muted-foreground">{star} <span class="text-yellow-400">★</span></span>
							<div class="h-3 flex-1 overflow-hidden rounded-full bg-muted">
								<div
									class="h-full rounded-full bg-yellow-400 transition-all"
									style="width: {(count / maxDistribution) * 100}%"
								></div>
							</div>
							<span class="w-8 text-right text-sm text-muted-foreground">{count}</span>
						</div>
					{/each}
				</div>
			</div>
		</div>
	{/if}

	<!-- Reviews List -->
	<div class="mt-6 space-y-4">
		{#if reviews.length === 0 && !loading}
			<div class="rounded-lg border border-border bg-card p-8 text-center text-sm text-muted-foreground">
				Chưa có đánh giá nào.
			</div>
		{/if}

		{#each reviews as review (review.id)}
			<div class="rounded-lg border border-border bg-card p-4">
				<!-- Review Header -->
				<div class="flex items-start justify-between">
					<div>
						<p class="font-medium">{review.customer_name}</p>
						<p class="mt-0.5 text-yellow-400">
							{renderStars(review.rating).filled}<span class="text-muted-foreground">{renderStars(review.rating).empty}</span>
						</p>
					</div>
					<span class="text-xs text-muted-foreground">{formatDate(review.created_at)}</span>
				</div>

				<!-- Comment -->
				{#if review.comment}
					<p class="mt-3 text-sm">{review.comment}</p>
				{/if}

				<!-- Existing Reply -->
				{#if review.reply}
					<div class="mt-3 rounded-md border border-border bg-muted/50 p-3">
						<p class="text-xs font-medium text-muted-foreground">Phản hồi của bạn</p>
						<p class="mt-1 text-sm">{review.reply}</p>
						{#if review.replied_at}
							<p class="mt-1 text-xs text-muted-foreground">{formatDate(review.replied_at)}</p>
						{/if}
					</div>
				{:else if replyingTo === review.id}
					<!-- Inline Reply Form -->
					<div class="mt-3 space-y-2">
						<textarea
							bind:value={replyText}
							placeholder="Nhập phản hồi..."
							rows="3"
							class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
						></textarea>
						<div class="flex gap-2">
							<button
								onclick={() => submitReply(review.id)}
								disabled={submittingReply || !replyText.trim()}
								class="rounded-md bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
							>
								{submittingReply ? 'Đang gửi...' : 'Gửi'}
							</button>
							<button
								onclick={cancelReply}
								class="rounded-md border border-input px-3 py-1.5 text-sm text-muted-foreground hover:bg-muted"
							>
								Hủy
							</button>
						</div>
					</div>
				{:else}
					<!-- Reply Button -->
					<button
						onclick={() => startReply(review.id)}
						class="mt-3 text-sm font-medium text-primary hover:text-primary/80"
					>
						Trả lời
					</button>
				{/if}
			</div>
		{/each}
	</div>

	<!-- Load More -->
	{#if hasMore && reviews.length > 0}
		<div class="mt-6 flex justify-center">
			<button
				onclick={loadMore}
				disabled={loading}
				class="rounded-md border border-input px-4 py-2 text-sm font-medium text-foreground hover:bg-muted disabled:opacity-50"
			>
				{loading ? 'Đang tải...' : 'Tải thêm'}
			</button>
		</div>
	{/if}

	{#if loading && reviews.length === 0}
		<div class="mt-6 flex justify-center">
			<p class="text-sm text-muted-foreground">Đang tải...</p>
		</div>
	{/if}
</div>
