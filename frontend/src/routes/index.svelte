<script>
	import { LangKey as lk, language as ln } from "$lib/lang";
</script>

<div class="root fadein">
	<h1>{ln[lk.INDEX_TITLE]}</h1>
	<p>{ln[lk.INDEX_DESCRIPTION]}</p>
	<a href="/register">Create your own personal KekAccount</a>
</div>

<style>
	.root {
		padding: 50px 0;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}
</style>
