import { goto } from '$app/navigation';
import { api } from '$lib/api/client';
import type { TokenResponse, UserResponse } from '$lib/api/types';

class AuthState {
	user: UserResponse | null = $state(null);
	token: string | null = $state(null);
	isLoading: boolean = $state(true);
	isAuthenticated: boolean = $derived(this.token !== null && this.user !== null);

	constructor() {
		if (typeof localStorage !== 'undefined') {
			this.token = localStorage.getItem('token');
		}
	}

	async init() {
		if (!this.token) {
			this.isLoading = false;
			return;
		}
		try {
			this.user = await api.get<UserResponse>('/auth/me');
		} catch {
			this.logout();
		}
		this.isLoading = false;
	}

	async register(phone: string, businessName: string, ownerName: string) {
		await api.post<UserResponse>('/auth/register', {
			phone,
			business_name: businessName,
			owner_name: ownerName
		});
	}

	async requestOtp(phone: string) {
		await api.post('/auth/otp/request', { phone });
	}

	async verifyOtp(phone: string, code: string) {
		const res = await api.post<TokenResponse>('/auth/otp/verify', { phone, code });
		this.setTokens(res);
		await this.init();
	}

	logout() {
		this.token = null;
		this.user = null;
		if (typeof localStorage !== 'undefined') {
			localStorage.removeItem('token');
			localStorage.removeItem('refresh_token');
		}
		goto('/login');
	}

	private setTokens(tokens: TokenResponse) {
		this.token = tokens.access_token;
		if (typeof localStorage !== 'undefined') {
			localStorage.setItem('token', tokens.access_token);
			localStorage.setItem('refresh_token', tokens.refresh_token);
		}
	}
}

export const auth = new AuthState();
