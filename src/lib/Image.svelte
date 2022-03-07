<script lang="ts">
    //import src from '$lib/assets/intro.jpg?format=avif&srcset';
	//import srcWebP from '$lib/assets/intro.jpg?format=webp&srcset';
	//import fallbackSrc from '$lib/assets/intro.jpg';

    export let classStr: string = "w-full object-cover";
    export let srcString: string;

    let src;
    let srcWebP;
    let fallbackSrc;

    import { onMount } from 'svelte';

	onMount(async () => {
		src = (await import(`./assets/${srcString}.jpg?format=avif&srcse`)).default;
		srcWebP = (await import(`./assets/${srcString}.jpg?format=webp&srcse`)).default;
		fallbackSrc = (await import(`./assets/${srcString}.jpg`)).default;
	});

</script>

<picture>
    <source srcset={src} type="image/avif" />
    <source srcset={srcWebP} type="image/webp" />
    <img class={classStr} alt="Could not load" loading="eager" decoding="async" src="{fallbackSrc}"/>
</picture>