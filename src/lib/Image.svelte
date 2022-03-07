<script lang="ts">
    import src from '$lib/assets/intro.jpg?format=avif&srcset';
	import srcWebP from '$lib/assets/intro.jpg?format=webp&srcset';
	import fallbackSrc from '$lib/assets/intro.jpg';
</script>

<picture>
    <source srcset={src} type="image/avif" />
    <source srcset={srcWebP} type="image/webp" />
    <img class="h-full w-full" alt="BG" loading="eager" decoding="async" src="{fallbackSrc}"/>
</picture>