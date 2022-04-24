<script lang="ts">
	import { LangKey as lk, language as ln } from "$lib/lang";

	import { fade } from "svelte/transition";
</script>

<div class="root" in:fade={{ duration: 200 }}>
	<h1 class="title">{ln[lk.INDEX_TITLE]}</h1>
	<p class="description">{ln[lk.INDEX_DESCRIPTION]}</p>
	<div class="link">
		<a href="/register">Create your own personal KekAccount</a>
	</div>
</div>

<style>
	.root {
		padding: 50px 0;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	p {
		margin-bottom: 15px;
	}
</style>
