<script>
	import { client } from "$lib/client";
	import { LangKey as lk } from "$lib/lang";
	import T from "$components/translate.svelte";

	let user = client.user;

	console.log($user);
</script>

<div class="root fadein">
	<h1 class="title"><T k={lk.ACCOUNT_TITLE} /></h1>
	{#if !$user.verified}
		<p class="hint short"><T k={lk.HINT_VERIFY} /></p>
	{/if}
	<p class="description"><T k={lk.ACCOUNT_DESCRIPTION} /> <T k={lk.BACK_TO_DASHBOARD} /></p>
	<p>{$user.name}</p>
	<p class="break">{$user.id}</p>
	<p>{$user.email}</p>
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

	.description {
		margin-bottom: 10px;
		margin-top: 5px;
	}
</style>
