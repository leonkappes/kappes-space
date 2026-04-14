<script lang="ts">
	import { page } from '$app/state';
</script>

<svelte:head>
	<title>{page.status} — Leon Kappes</title>
</svelte:head>

<div class="flex flex-col items-center justify-center h-full gap-4 py-20 text-center">
	<h1 class="text-6xl font-bold text-gray-900">{page.status}</h1>
	<p class="text-xl text-gray-700">{page.error?.message ?? 'Something went wrong.'}</p>
	<a href="/" class="mt-4 px-5 py-2 rounded bg-blue-700 text-white hover:bg-blue-800 transition-colors">
		Go home
	</a>
</div>
