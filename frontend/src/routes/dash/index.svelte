<script>
	import { client } from "$lib/client";
	import { LangKey as lk, language as ln } from "$lib/lang";

	let user = client.user;

	console.log($user);

	const categories = [
		{
			title: ln[lk.ACCOUNT_TITLE],
			description: ln[lk.ACCOUNT_DESCRIPTION],
			link: "/dash/account"
		},
		{
			title: ln[lk.APPS_TITLE],
			description: ln[lk.APPS_DESCRIPTION],
			link: "/dash/apps"
		},
		{
			title: ln[lk.TOKENS_TITLE],
			description: ln[lk.TOKENS_DESCRIPTION],
			link: "/dash/tokens"
		}
	];
</script>

<h1 class="title">{ln[lk.DASHBOARD_TITLE]}</h1>
{#if !$user.verified}
	<p class="hint short">{ln[lk.HINT_VERIFY]}</p>
{/if}

<div class="categories">
	{#each categories as category}
		<a class="card category" href={category.link}>
			<h2>{category.title}</h2>
			<p>{category.description}</p>
		</a>
	{/each}
</div>

<style>
	.categories {
		margin-top: 20px;
		display: flex;
		flex-direction: column;
		gap: 15px;
	}

	.category:hover {
		border-bottom-width: 1px;
	}
</style>
