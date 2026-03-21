import { goto } from '$app/navigation';
import { api, ApiError, setTokens as persistTokens, clearTokens } from '$lib/api/client';
import type { TokenResponse, UserResponse } from '$lib/api/types';

class AuthState {
	user: UserResponse | null = $state(null);
	token: string | null = $state(null);
	isLoading: boolean = $state(true);
	error: string | null = $state(null);
	isAuthenticated: boolean = $derived(this.token !== null && this.user !== null);

	constructor() {
		if (typeof localStorage !== 'undefined') {
			this.token = localStorage.getItem('driver_token');
		}
	}

	async init() {
		if (!this.token) {
			this.isLoading = false;
			return;
		}
		try {
			this.user = await api.get<UserResponse>('/auth/me');
			this.error = null;
		} catch (err) {
			if (err instanceof ApiError && err.code === 401) {
				this.logout();
			} else {
				this.error = 'Loi ket noi. Vui long thu lai.';
				this.logout();
			}
		}
		this.isLoading = false;
	}

	async requestOtp(phone: string) {
		this.error = null;
		try {
			await api.post('/auth/otp/request', { phone });
		} catch (err) {
			this.error = err instanceof ApiError ? err.message : 'Gui OTP that bai';
			throw err;
		}
	}

	async verifyOtp(phone: string, code: string) {
		this.error = null;
		try {
			const res = await api.post<TokenResponse>('/auth/otp/verify', { phone, code });
			this.setTokens(res);
			await this.init();
		} catch (err) {
			if (err instanceof ApiError && err.code === 404) {
				throw err;
			}
			this.error = err instanceof ApiError ? err.message : 'Xac thuc that bai';
			throw err;
		}
	}

	async register(phone: string, name: string) {
		this.error = null;
		try {
			await api.post<UserResponse>('/auth/register', {
				phone,
				owner_name: name,
				business_name: name
			});
		} catch (err) {
			this.error = err instanceof ApiError ? err.message : 'Dang ky that bai';
			throw err;
		}
	}

	logout() {
		this.token = null;
		this.user = null;
		this.error = null;
		clearTokens();
		goto('/login');
	}

	private setTokens(tokens: TokenResponse) {
		this.token = tokens.access_token;
		persistTokens(tokens.access_token, tokens.refresh_token);
	}
}

export const auth = new AuthState();
