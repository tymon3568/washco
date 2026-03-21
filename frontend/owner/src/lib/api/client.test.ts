import { describe, it, expect } from 'vitest';
import { ApiError } from './client';

describe('ApiError', () => {
	it('creates error with message and code', () => {
		const err = new ApiError('Not found', 404);
		expect(err.message).toBe('Not found');
		expect(err.code).toBe(404);
		expect(err.name).toBe('ApiError');
	});

	it('is an instance of Error', () => {
		const err = new ApiError('Server error', 500);
		expect(err instanceof Error).toBe(true);
		expect(err instanceof ApiError).toBe(true);
	});

	it('works with try/catch pattern', () => {
		try {
			throw new ApiError('Có lỗi xảy ra', 422);
		} catch (e: unknown) {
			expect(e instanceof ApiError).toBe(true);
			if (e instanceof ApiError) {
				expect(e.message).toBe('Có lỗi xảy ra');
				expect(e.code).toBe(422);
			}
		}
	});
});
