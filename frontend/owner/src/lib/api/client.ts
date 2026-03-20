const BASE_URL = '/api/v1';

interface RequestOptions {
	method?: string;
	body?: unknown;
	headers?: Record<string, string>;
}

async function request<T>(path: string, options: RequestOptions = {}): Promise<T> {
	const { method = 'GET', body, headers = {} } = options;

	const token = typeof localStorage !== 'undefined' ? localStorage.getItem('token') : null;

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
		const error = await res.json().catch(() => ({ error: 'Request failed' }));
		throw new Error(error.error || `HTTP ${res.status}`);
	}

	return res.json();
}

export const api = {
	get: <T>(path: string) => request<T>(path),
	post: <T>(path: string, body: unknown) => request<T>(path, { method: 'POST', body }),
	put: <T>(path: string, body: unknown) => request<T>(path, { method: 'PUT', body }),
	del: <T>(path: string) => request<T>(path, { method: 'DELETE' })
};
