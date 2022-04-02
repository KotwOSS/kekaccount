<script>
	import { client } from "$lib/client";
	import { LangKey as lk, language } from "$lib/lang";
	import T from "$components/translate.svelte";

	let user = client.user;

	console.log($user);

	const categories = [
		{
			title: language[lk.ACCOUNT_TITLE],
			description: language[lk.ACCOUNT_DESCRIPTION],
			link: "/dash/account"
		},
		{
			title: language[lk.APPS_TITLE],
			description: language[lk.APPS_DESCRIPTION],
			link: "/dash/apps"
		},
		{
			title: language[lk.TOKENS_TITLE],
			description: language[lk.TOKENS_DESCRIPTION],
			link: "/dash/tokens"
		}
	];
</script>

<div class="root fadein">
	<h1 class="title"><T k={lk.DASHBOARD_TITLE} /></h1>
	{#if !$user.verified}
		<p class="hint short"><T k={lk.HINT_VERIFY} /></p>
	{/if}

	<div class="categories">
		{#each categories as category}
			<a class="category card" href={category.link}>
				<h2>{category.title}</h2>
				<p>{category.description}</p>
			</a>
		{/each}
	</div>
</div>

<style>
	.categories {
		margin-top: 20px;
		display: flex;
		flex-direction: column;
		gap: 15px;
		justify-content: center;
		align-items: center;
	}

	.root {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		background: linear-gradient(180deg, rgba(0, 0, 0, 0.8) 0%, rgba(0, 0, 0, 1) 50%), url("/bg.jpg");
		background-size: cover;
	}
</style>
