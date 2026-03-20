const BASE_URL = '/api/v1';

interface RequestOptions {
	method?: string;
	body?: unknown;
}

async function request<T>(path: string, options: RequestOptions = {}): Promise<T> {
	const { method = 'GET', body } = options;

	const res = await fetch(`${BASE_URL}${path}`, {
		method,
		headers: { 'Content-Type': 'application/json' },
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
	put: <T>(path: string, body: unknown) => request<T>(path, { method: 'PUT', body })
};
