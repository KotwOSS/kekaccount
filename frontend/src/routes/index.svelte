<script>
	import T from "$components/translate.svelte";
</script>

<div class="root fadein">
	<h1><T k="index.title" /></h1>
	<p><T k="index.description" /></p>
	<p>
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
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		background: linear-gradient(180deg, rgba(0, 0, 0, 0.8) 0%, rgba(0, 0, 0, 1) 50%), url("/bg.jpg");
		background-size: cover;
	}
</style>
