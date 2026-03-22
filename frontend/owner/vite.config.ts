import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	test: {
		include: ['src/**/*.test.ts']
	},
	server: {
		host: '0.0.0.0',
		allowedHosts: ['owner.washco.local', 'localhost'],
		proxy: {
			'/api/v1/queue/ws': {
				target: 'ws://localhost:8080',
				ws: true
			},
			'/api': {
				target: 'http://localhost:8080',
				changeOrigin: true
			}
		}
	}
});
