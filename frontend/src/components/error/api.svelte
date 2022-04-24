<script lang="ts">
	import { support } from "$lib/config";
	import { language as ln, LangKey as lk } from "$lib/lang";

	const emojis = ["🤕", "🤧", "🤯", "😕", "😬"];

	const emoji = emojis[Math.floor(Math.random() * emojis.length)];
</script>

<div class="content">
	<h1>{emoji}</h1>

	<p class="short">{ln[lk.API_ERROR_DESCRIPTION]}</p>
	<div class="link">
		<a href={support}>{ln[lk.API_ERROR_CONTACT]}</a>
	</div>
</div>

<style>
	.content {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 20px 0;
		width: 100%;
		min-height: 100vh;
	}

	h1 {
		font-size: 100px;
	}

	a {
		margin-top: 10px;
	}

	p {
		text-align: center;
	}
</style>
