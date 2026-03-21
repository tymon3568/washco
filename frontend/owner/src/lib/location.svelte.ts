import { api } from '$lib/api/client';
import type { LocationResponse } from '$lib/api/types';

class LocationState {
	locations: LocationResponse[] = $state([]);
	selectedId: string | null = $state(null);
	isLoading: boolean = $state(false);
	current: LocationResponse | null = $derived(
		this.locations.find((l) => l.id === this.selectedId) ?? this.locations[0] ?? null
	);

	async load() {
		this.isLoading = true;
		try {
			this.locations = await api.get<LocationResponse[]>('/locations');
			// Restore from localStorage or pick first
			const saved =
				typeof localStorage !== 'undefined' ? localStorage.getItem('selected_location') : null;
			if (saved && this.locations.some((l) => l.id === saved)) {
				this.selectedId = saved;
			} else if (this.locations.length > 0) {
				this.selectedId = this.locations[0].id;
			}
		} catch {
			this.locations = [];
		}
		this.isLoading = false;
	}

	select(id: string) {
		this.selectedId = id;
		if (typeof localStorage !== 'undefined') {
			localStorage.setItem('selected_location', id);
		}
	}
}

export const locationState = new LocationState();
