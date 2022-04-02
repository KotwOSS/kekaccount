<script>
	import T from "$components/translate.svelte";

	import { LangKey as lk } from "$lib/lang";
</script>

<div class="root fadein">
	<h1><T k={lk.INDEX_TITLE} /></h1>
	<p><T k={lk.INDEX_DESCRIPTION} /></p>
	<p class="short">
		Lorem ipsum dolor sit amet consectetur adipisicing elit. Saepe ad dolor fuga distinctio facilis
		tempore praesentium molestias harum eligendi, esse inventore, molestiae quos ut enim in fugit
		totam minima numquam. Aliquid repudiandae assumenda quia, itaque rerum voluptatum ducimus quam
		natus obcaecati nemo dolores dolor minima voluptates iure harum id dicta eveniet. Illum odio,
		minus mollitia qui vero libero? Voluptate, fugit? Quod ex nisi laborum veritatis sunt non
		doloremque, vero amet soluta eos porro voluptas officiis voluptatem corrupti iure eligendi
		excepturi, distinctio accusamus, vel nobis fugit nihil adipisci natus! Ad, molestias? Odit
		aperiam accusantium, velit eveniet vero nesciunt dolorum impedit repudiandae nobis alias quia,
		non quisquam earum cumque adipisci dolores! Accusamus quis omnis eos facilis nostrum natus odio
		velit eaque quia. Optio odit ducimus, assumenda sapiente repellendus iste, magni minima illo
		esse fuga neque quibusdam quos. Optio dolor voluptatibus eligendi deleniti reprehenderit vero
		magni porro praesentium! Consectetur alias culpa aut facilis.
	</p>

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
