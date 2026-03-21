const BASE_URL = '/api/v1';

interface RequestOptions {
	method?: string;
	body?: unknown;
	headers?: Record<string, string>;
}

interface ErrorResponse {
	error: string;
	code: number;
}

export class ApiError extends Error {
	code: number;
	constructor(message: string, code: number) {
		super(message);
		this.name = 'ApiError';
		this.code = code;
	}
}

let isRefreshing = false;
let refreshQueue: Array<{
	resolve: (token: string) => void;
	reject: (err: Error) => void;
}> = [];

function getToken(): string | null {
	return typeof localStorage !== 'undefined' ? localStorage.getItem('token') : null;
}

function getRefreshToken(): string | null {
	return typeof localStorage !== 'undefined' ? localStorage.getItem('refresh_token') : null;
}

function setTokens(access: string, refresh: string) {
	if (typeof localStorage !== 'undefined') {
		localStorage.setItem('token', access);
		localStorage.setItem('refresh_token', refresh);
	}
}

function clearTokens() {
	if (typeof localStorage !== 'undefined') {
		localStorage.removeItem('token');
		localStorage.removeItem('refresh_token');
	}
}

async function refreshAccessToken(): Promise<string> {
	const refreshToken = getRefreshToken();
	if (!refreshToken) {
		throw new ApiError('No refresh token', 401);
	}

	const res = await fetch(`${BASE_URL}/auth/refresh`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ refresh_token: refreshToken })
	});

	if (!res.ok) {
		clearTokens();
		throw new ApiError('Session expired', 401);
	}

	const data = await res.json();
	setTokens(data.access_token, data.refresh_token);
	return data.access_token;
}

async function handleTokenRefresh(): Promise<string> {
	if (isRefreshing) {
		return new Promise((resolve, reject) => {
			refreshQueue.push({ resolve, reject });
		});
	}

	isRefreshing = true;
	try {
		const newToken = await refreshAccessToken();
		refreshQueue.forEach((q) => q.resolve(newToken));
		return newToken;
	} catch (err) {
		refreshQueue.forEach((q) => q.reject(err as Error));
		throw err;
	} finally {
		refreshQueue = [];
		isRefreshing = false;
	}
}

async function request<T>(path: string, options: RequestOptions = {}, retry = true): Promise<T> {
	const { method = 'GET', body, headers = {} } = options;
	const token = getToken();

	const res = await fetch(`${BASE_URL}${path}`, {
		method,
		headers: {
			'Content-Type': 'application/json',
			...(token ? { Authorization: `Bearer ${token}` } : {}),
			...headers
		},
		body: body ? JSON.stringify(body) : undefined
	});

	if (!res.ok) {
		// Token expired - try refresh once
		if (res.status === 401 && retry && getRefreshToken()) {
			try {
				const newToken = await handleTokenRefresh();
				// Retry original request with new token
				return request<T>(path, {
					...options,
					headers: { ...headers, Authorization: `Bearer ${newToken}` }
				}, false);
			} catch {
				// Refresh also failed - clear auth
				clearTokens();
				if (typeof window !== 'undefined') {
					window.location.href = '/login';
				}
				throw new ApiError('Session expired', 401);
			}
		}

		const error: ErrorResponse = await res.json().catch(() => ({
			error: `HTTP ${res.status}`,
			code: res.status
		}));
		throw new ApiError(error.error || `HTTP ${res.status}`, res.status);
	}

	// Handle 204 No Content
	if (res.status === 204) {
		return undefined as T;
	}

	return res.json();
}

export const api = {
	get: <T>(path: string) => request<T>(path),
	post: <T>(path: string, body: unknown) => request<T>(path, { method: 'POST', body }),
	put: <T>(path: string, body: unknown) => request<T>(path, { method: 'PUT', body }),
	del: <T>(path: string) => request<T>(path, { method: 'DELETE' })
};
