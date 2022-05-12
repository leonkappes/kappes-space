<script lang="ts">
	export let href: string = "";
	export let image: boolean = false;
	export let title: string;
	export let description: string;

    let target = href.startsWith("http://") || href.startsWith("https://") ? '_blank' : '_self';
</script>

<a
    rel="noopener noreferrer"
	{href}
    {target}
	class="flex flex-col items-center m-2 bg-white rounded-lg border shadow-md md:flex-row hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-900"
>
	{#if image}
		<slot></slot>
	{/if}
	<div class="flex flex-col justify-between p-4 leading-normal w-full">
		<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
			{title}
		</h5>
		<p class="mb-3 font-normal text-gray-700 dark:text-gray-400">
			{description}
		</p>
	</div>
</a>
