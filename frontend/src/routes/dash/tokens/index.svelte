<script lang="ts">
	import { client } from "$lib/client";
	import { LangKey as lk, language } from "$lib/lang";
	import T from "$components/translate.svelte";
	import { APIError, Routes } from "$lib/api";
	import Loader from "$components/loader.svelte";
	import Error from "$components/error.svelte";
	import { goto } from "$app/navigation";

	let user = client.user;

	let error: string;

	let tokens: any;

	Routes.Auth.Token.LIST.send({ token: client.token })
		.then((r) => (tokens = r))
		.catch((e) => {
			if (e instanceof APIError) error = e.get_message();
			else error = language[lk.ERROR_CONNECTION];
		});

	function create() {
		goto("/dash/tokens/create");
	}
</script>

<div class="root fadein">
	<h1 class="title"><T k={lk.TOKENS_TITLE} /></h1>
	{#if !$user.verified}
		<p class="hint short"><T k={lk.HINT_VERIFY} /></p>
	{/if}
	<p class="description"><T k={lk.TOKENS_DESCRIPTION} /> <T k={lk.BACK_TO_DASHBOARD} /></p>
	<Error {error} />

	{#if tokens}
		<div class="tokens">
			{#each tokens as token}
				<div class="token card">
					<p class="token-name">{token.name}</p>
					<p class="token-id">{token.id}</p>
				</div>
			{/each}
		</div>
	{:else}
		<Loader />
	{/if}
	<button on:click={create}><T k={lk.TOKENS_CREATE} /></button>
</div>

<style>
	.root > :global(.loader) {
		width: 50px;
		height: 50px;
	}

	.tokens {
		max-width: 500px;
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		align-items: center;
		gap: 10px;
		margin-bottom: 20px;
	}

	.root {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.description {
		margin-bottom: 10px;
		margin-top: 5px;
	}
</style>
