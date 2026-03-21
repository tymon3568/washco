export function formatVND(amount: number): string {
	if (amount === 0) return 'Mien phi';
	return amount.toString().replace(/\B(?=(\d{3})+(?!\d))/g, '.') + 'd';
}

export function formatDistance(meters: number): string {
	if (meters < 1000) return `${Math.round(meters)}m`;
	return `${(meters / 1000).toFixed(1)}km`;
}

export function formatDuration(minutes: number): string {
	if (minutes < 60) return `${minutes} phut`;
	const h = Math.floor(minutes / 60);
	const m = minutes % 60;
	return m > 0 ? `${h}h ${m}p` : `${h}h`;
}

export function formatDate(dateStr: string): string {
	const d = new Date(dateStr);
	return d.toLocaleDateString('vi-VN', { day: '2-digit', month: '2-digit', year: 'numeric' });
}

export function formatTimeSlot(slot: string): string {
	return slot.slice(0, 5);
}

export function formatRelativeTime(dateStr: string): string {
	const now = Date.now();
	const then = new Date(dateStr).getTime();
	const diff = now - then;

	const minutes = Math.floor(diff / 60000);
	if (minutes < 1) return 'Vua xong';
	if (minutes < 60) return `${minutes} phut truoc`;

	const hours = Math.floor(minutes / 60);
	if (hours < 24) return `${hours} gio truoc`;

	const days = Math.floor(hours / 24);
	if (days < 7) return `${days} ngay truoc`;

	return formatDate(dateStr);
}

export function formatPhone(phone: string): string {
	if (phone.length === 10) {
		return `${phone.slice(0, 4)} ${phone.slice(4, 7)} ${phone.slice(7)}`;
	}
	return phone;
}
