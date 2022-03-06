<script>
	import Footer from '$lib/Footer.svelte';

	import { onMount } from 'svelte';
	// @ts-ignore
	import src from '$lib/assets/intro.jpg?format=avif&srcset';
	import srcWebP from '$lib/assets/intro.jpg?format=webp&srcset';
	import fallbackSrc from '$lib/assets/intro.jpg';

	onMount(async () => {
		const module = await import('tsparticles');

		let tsParticles = module.tsParticles;

		// @ts-ignore
		tsParticles.load('particles', particlesConfig);
	});

	let particlesConfig = {
		fullScreen: {
			enable: false
		},
		particles: {
			number: {
				value: 50,
				density: {
					enable: true,
					value_area: 800
				}
			},
			color: {
				value: '#ffffff'
			},
			shape: {
				type: 'polygon',
				stroke: {
					width: 0,
					color: '#000000'
				},
				polygon: {
					nb_sides: 3
				},
				image: {
					src: 'img/github.svg',
					width: 100,
					height: 100
				}
			},
			opacity: {
				value: 1,
				random: true,
				anim: {
					enable: true,
					speed: 1,
					opacity_min: 0,
					sync: false
				}
			},
			size: {
				value: 3,
				random: true,
				anim: {
					enable: false,
					speed: 4,
					size_min: 0.3,
					sync: false
				}
			},
			line_linked: {
				enable: false,
				distance: 150,
				color: '#ffffff',
				opacity: 0.4,
				width: 1
			},
			move: {
				enable: true,
				speed: 1,
				direction: 'none',
				random: true,
				straight: false,
				out_mode: 'out',
				bounce: false,
				attract: {
					enable: false,
					rotateX: 600,
					rotateY: 600
				}
			}
		},
		interactivity: {
			detect_on: 'canvas',
			events: {
				onhover: {
					enable: false,
					mode: 'grab'
				},
				onclick: {
					enable: false,
					mode: 'repulse'
				},
				resize: true
			},
			modes: {
				grab: {
					distance: 80,
					line_linked: {
						opacity: 1
					}
				},
				bubble: {
					distance: 250,
					size: 0,
					duration: 2,
					opacity: 0,
					speed: 3
				},
				repulse: {
					distance: 400,
					duration: 0.4
				},
				push: {
					particles_nb: 4
				},
				remove: {
					particles_nb: 2
				}
			}
		},
		retina_detect: true
	};
</script>

<svelte:head>
	<title>Leon Kappes | Home</title>
</svelte:head>

<div class="w-full h-full flex items-center justify-center relative">
	<div class="absolute w-full h-full">
		<picture>
			<source srcset={src} type="image/avif" />
			<source srcset={srcWebP} type="image/webp" />
			<img class="h-full w-full" alt="BG" loading="eager" decoding="async" src="{fallbackSrc}"/>
		</picture>
	</div>
	<div class="absolute w-full h-full" id="particles" />
	<div class="flex flex-col items-center justify-center m-auto">
		<h1 class="lg:text-9xl md:text-6xl text-4xl mx-3 text-white z-10">Hi, I'm Leon Kappes</h1>
		<hr class="md:w-14 w-6 mb-24 z-10" />
		<section class="flex justify-center items-center md:text-6xl text-lg">
			<a
				href="mailto:leon@kappes.space"
				target="_blank"
				class="flex z-10 items-center justify-center text-white hover:text-gray-800 border border-white hover:bg-white focus:ring-4 focus:ring-gray-400 rounded-lg px-5 py-2.5 mr-2 mb-2 transition-all"
				role="button"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-10 w-10"
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
					class="h-10 w-10 m-0"
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
