/**
 * Format VND amount: 150000 -> "150.000d"
 */
export function formatVND(amount: number): string {
	if (amount === 0) return '0d';
	const formatted = Math.abs(amount)
		.toString()
		.replace(/\B(?=(\d{3})+(?!\d))/g, '.');
	return amount < 0 ? `-${formatted}d` : `${formatted}d`;
}

/**
 * Format duration: 45 -> "45 phút"
 */
export function formatDuration(minutes: number): string {
	if (minutes < 60) return `${minutes} phút`;
	const hours = Math.floor(minutes / 60);
	const mins = minutes % 60;
	return mins > 0 ? `${hours}h ${mins}p` : `${hours}h`;
}

/**
 * Format relative time in Vietnamese
 */
export function formatRelativeTime(date: Date): string {
	const now = new Date();
	const diffMs = now.getTime() - date.getTime();
	const diffMins = Math.floor(diffMs / 60000);

	if (diffMins < 1) return 'Vừa xong';
	if (diffMins < 60) return `${diffMins} phút trước`;

	const diffHours = Math.floor(diffMins / 60);
	if (diffHours < 24) return `${diffHours} giờ trước`;

	const diffDays = Math.floor(diffHours / 24);
	return `${diffDays} ngày trước`;
}
