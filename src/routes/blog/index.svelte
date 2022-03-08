<script lang="ts" context="module">
	import type { Load } from '@sveltejs/kit';

	export const load: Load = async ({ fetch }) => {
		const res = await fetch('blog/posts.json');
		const json: PostResponse = await res.json();

        console.log(json)
        console.log(json.data)

		return { props: { postData: json } };
	};
</script>

<script lang="ts">
	import type { Post } from '$lib/types';
    import type { PostResponse } from '$lib/types/post';

	export let postData: PostResponse;

    let posts = postData.data;
</script>

<div>
	{#each posts as post}
		<h1>{post.attributes.title}</h1>
	{/each}
</div>
