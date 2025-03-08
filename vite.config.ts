import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit(), tailwindcss()],
	ssr: {
		noExternal: ['@tsparticles/slim', '@tsparticles/engine'] // add all tsparticles libraries here, they're not made for SSR, they're client only
	}
});
