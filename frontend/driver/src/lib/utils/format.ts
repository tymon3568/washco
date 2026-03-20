export function formatVND(amount: number): string {
	return amount.toString().replace(/\B(?=(\d{3})+(?!\d))/g, '.') + 'd';
}

export function formatDistance(meters: number): string {
	if (meters < 1000) return `${Math.round(meters)}m`;
	return `${(meters / 1000).toFixed(1)}km`;
}
