import { describe, it, expect } from 'vitest';
import { formatVND, formatDuration, formatRelativeTime } from './format';

describe('formatVND', () => {
	it('formats zero', () => {
		expect(formatVND(0)).toBe('0d');
	});

	it('formats small amount', () => {
		expect(formatVND(5000)).toBe('5.000d');
	});

	it('formats typical price', () => {
		expect(formatVND(150000)).toBe('150.000d');
	});

	it('formats large amount', () => {
		expect(formatVND(1500000)).toBe('1.500.000d');
	});

	it('formats negative amount', () => {
		expect(formatVND(-50000)).toBe('-50.000d');
	});

	it('formats amount without thousands', () => {
		expect(formatVND(500)).toBe('500d');
	});
});

describe('formatDuration', () => {
	it('formats minutes only', () => {
		expect(formatDuration(45)).toBe('45 phút');
	});

	it('formats exact hours', () => {
		expect(formatDuration(60)).toBe('1h');
		expect(formatDuration(120)).toBe('2h');
	});

	it('formats hours and minutes', () => {
		expect(formatDuration(90)).toBe('1h 30p');
		expect(formatDuration(75)).toBe('1h 15p');
	});

	it('formats zero', () => {
		expect(formatDuration(0)).toBe('0 phút');
	});
});

describe('formatRelativeTime', () => {
	it('formats just now', () => {
		expect(formatRelativeTime(new Date())).toBe('Vừa xong');
	});

	it('formats minutes ago', () => {
		const fiveMinAgo = new Date(Date.now() - 5 * 60 * 1000);
		expect(formatRelativeTime(fiveMinAgo)).toBe('5 phút trước');
	});

	it('formats hours ago', () => {
		const twoHoursAgo = new Date(Date.now() - 2 * 60 * 60 * 1000);
		expect(formatRelativeTime(twoHoursAgo)).toBe('2 giờ trước');
	});

	it('formats days ago', () => {
		const threeDaysAgo = new Date(Date.now() - 3 * 24 * 60 * 60 * 1000);
		expect(formatRelativeTime(threeDaysAgo)).toBe('3 ngày trước');
	});
});
