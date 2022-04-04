<script lang="ts">
	import { client } from "$lib/client";
	import { LangKey as lk, language as ln } from "$lib/lang";
	import { APIError, Routes } from "$lib/api";

	import Loader from "$components/loader.svelte";
    import TokensTable from "$components/table/tokens.svelte";
	
    import { goto } from "$app/navigation";

	let user = client.user;

	let error: string;

	let tokens: any;

	Routes.Auth.Token.LIST.send({ token: client.token })
		.then((r) => (tokens = r))
		.catch((e) => {
			if (e instanceof APIError) error = e.get_message();
			else error = ln[lk.ERROR_CONNECTION];
		});

	function create() {
		goto("/dash/tokens/create");
	}
</script>

<div class="root fadein">
	<h1 class="title">{ln[lk.TOKENS_TITLE]}</h1>
	{#if !$user.verified}
		<p class="hint short">{ln[lk.HINT_VERIFY]}</p>
	{/if}
	<p class="description">{ln[lk.TOKENS_DESCRIPTION]} {@html ln[lk.BACK_TO_DASHBOARD]}</p>
	{#if error}<p class="error break short">{error}</p>{/if}

    {#if tokens}
        <TokensTable data={tokens} />
    {:else}
        <Loader />
    {/if}

	<button class="create" on:click={create}>{ln[lk.TOKENS_CREATE]}</button>
</div>

<style>
	.root > :global(.loader) {
		width: 50px;
		height: 50px;
	}

    .create {
        margin-top: 15px;
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
