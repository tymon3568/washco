const STORAGE_KEY = 'washco_favorites';

class FavoritesState {
	ids: string[] = $state([]);

	constructor() {
		if (typeof localStorage !== 'undefined') {
			try {
				const stored = localStorage.getItem(STORAGE_KEY);
				if (stored) this.ids = JSON.parse(stored);
			} catch {
				this.ids = [];
			}
		}
	}

	isFavorite(locationId: string): boolean {
		return this.ids.includes(locationId);
	}

	toggle(locationId: string) {
		if (this.isFavorite(locationId)) {
			this.ids = this.ids.filter((id) => id !== locationId);
		} else {
			this.ids = [...this.ids, locationId];
		}
		this.persist();
	}

	private persist() {
		if (typeof localStorage !== 'undefined') {
			localStorage.setItem(STORAGE_KEY, JSON.stringify(this.ids));
		}
	}
}

export const favorites = new FavoritesState();
