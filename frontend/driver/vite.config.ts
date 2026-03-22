import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	server: {
		host: '0.0.0.0',
		port: 5174,
		allowedHosts: ['app.washco.local', 'localhost'],
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
