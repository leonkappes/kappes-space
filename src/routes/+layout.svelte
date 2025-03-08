<script lang="ts">
	import Footer from '$lib/Footer.svelte';
	import Header from '$lib/Header.svelte';
	import '../app.css';
	let { children } = $props();
</script>

<div class="flex flex-col h-full">
	<Header />
	<div class="flex-grow">
		{@render children()}
	</div>
	<Footer />
</div>
