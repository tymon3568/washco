import { goto } from '$app/navigation';
import { api, ApiError } from '$lib/api/client';
import type { TokenResponse, UserResponse } from '$lib/api/types';

class AuthState {
	user: UserResponse | null = $state(null);
	token: string | null = $state(null);
	isLoading: boolean = $state(true);
	error: string | null = $state(null);
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
			this.error = null;
		} catch (err) {
			if (err instanceof ApiError && err.code === 401) {
				// Token refresh is handled by the api client automatically
				// If we still get 401, the session is truly expired
				this.logout();
			} else {
				this.error = 'Lỗi kết nối. Vui lòng thử lại.';
				this.logout();
			}
		}
		this.isLoading = false;
	}

	async register(phone: string, businessName: string, ownerName: string) {
		this.error = null;
		try {
			await api.post<UserResponse>('/auth/register', {
				phone,
				business_name: businessName,
				owner_name: ownerName
			});
		} catch (err) {
			this.error = err instanceof ApiError ? err.message : 'Đăng ký thất bại';
			throw err;
		}
	}

	async requestOtp(phone: string) {
		this.error = null;
		try {
			await api.post('/auth/otp/request', { phone });
		} catch (err) {
			this.error = err instanceof ApiError ? err.message : 'Gửi OTP thất bại';
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
			this.error = err instanceof ApiError ? err.message : 'Xác thực thất bại';
			throw err;
		}
	}

	logout() {
		this.token = null;
		this.user = null;
		this.error = null;
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
