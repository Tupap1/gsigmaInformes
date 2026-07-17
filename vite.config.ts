import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],

	// Vite options tailored for Tauri development and building
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
		host: false,
		hmr: true,
		watch: {
			// Tell Vite to ignore the src-tauri directory to prevent 
			// unnecessary restarts when Tauri-related Rust files change
			ignored: ["**/src-tauri/**"],
		},
	},
	envPrefix: ["VITE_", "TAURI_ENV_"]
});
