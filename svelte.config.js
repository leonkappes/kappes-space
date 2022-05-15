import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';
import path from "path";
import { chunkSplitPlugin } from 'vite-plugin-chunk-split';


/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter({
			precompress: true
		}),
		vite: {
			plugins: [
				chunkSplitPlugin(),
			]
		},
		prerender: {
			default: true,
			crawl: true,
		}
	}
};

export default config;
