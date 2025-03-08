<script lang="ts">
	import { loadSlim } from '@tsparticles/slim';
	import { onMount } from 'svelte';

	onMount(async () => {
		const module = await import('@tsparticles/engine');
		let tsParticles = module.tsParticles;

		await loadSlim(tsParticles);
		await tsParticles.load({ id: 'particles', url: 'particles.json' });
	});
</script>

<svelte:head>
	<title>Leon Kappes | Home</title>
</svelte:head>

<div class="w-full h-full flex items-center justify-center relative">
	<div class="absolute w-full h-full bg-gray-800 dark:bg-gray-700"></div>
	<div class="absolute w-full h-full" id="particles"></div>
	<div class="flex flex-col items-center justify-center m-auto">
		<h1 class="lg:text-8xl md:text-6xl text-4xl mx-3 text-white z-10">Hi, I'm Leon Kappes</h1>
		<hr class="w-11/12 z-10 opacity-50 lg:mt-3 md:mt-2 text-white" />
		<h2
			class="lg:text-5xl md:text-3xl text-xl text-white z-10 mb-5 lg:mb-16 md:mb-8 mt-2 lg:mt-4 md:mt-4"
		>
			DevOps and System Administration
		</h2>
		<section class="flex justify-center items-center md:text-3xl text-lg">
			<a
				href="mailto:leon@kappes.space"
				target="_blank"
				class="flex z-10 items-center justify-center text-white hover:text-gray-800 border border-white hover:bg-white focus:ring-4 focus:ring-gray-400 rounded-lg px-5 py-2.5 mr-2 mb-2 transition-all"
				role="button"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="md:h-8 md:w-8 h-4 w-4"
					viewBox="0 0 20 20"
					fill="currentColor"
				>
					<path
						fill-rule="evenodd"
						d="M14.243 5.757a6 6 0 10-.986 9.284 1 1 0 111.087 1.678A8 8 0 1118 10a3 3 0 01-4.8 2.401A4 4 0 1114 10a1 1 0 102 0c0-1.537-.586-3.07-1.757-4.243zM12 10a2 2 0 10-4 0 2 2 0 004 0z"
						clip-rule="evenodd"
					/>
				</svg> E-Mail
			</a>
			<a
				href="https://github.com/leonkappes"
				target="_blank"
				class="flex z-10 items-center justify-center text-white hover:text-gray-800 border border-white hover:bg-white focus:ring-4 focus:ring-gray-400 rounded-lg px-5 py-2.5 mr-2 mb-2 transition-all"
				role="button"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="md:h-8 md:w-8 h-4 w-4"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					stroke-width="2"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
					/>
				</svg> Github
			</a>
		</section>
	</div>
</div>
