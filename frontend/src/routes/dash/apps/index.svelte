<script lang="ts">
	import { client } from "$lib/client";
	import { LangKey as lk, language as ln } from "$lib/lang";
	import { APIError, Routes } from "$lib/api";

	import Loader from "$components/loader.svelte";

	import { goto } from "$app/navigation";

	let user = client.user;

	let error: string;

	let apps: any;

	Routes.Apps.LIST.send({ token: client.token })
		.then((r) => (apps = r))
		.catch((e) => {
			if (e instanceof APIError) error = e.get_message();
			else error = ln[lk.ERROR_CONNECTION];
		});

	function create() {
		goto("/dash/apps/create");
	}
</script>

<h1 class="title">{ln[lk.APPS_TITLE]}</h1>
{#if !$user.verified}
	<p class="hint short">{ln[lk.HINT_VERIFY]}</p>
{/if}
<p class="description">{ln[lk.APPS_DESCRIPTION]} {@html ln[lk.BACK_TO_DASHBOARD]}</p>

{#if error}
	<p class="error break short">{error}</p>
{:else if apps}
	<div class="apps">
		{#each apps as app}
			<div class="app card">
				<p class="app-name">{app.name}</p>
				<p class="app-id">{app.id}</p>
			</div>
		{/each}
	</div>
{:else}
	<div class="load">
		<Loader />
	</div>
{/if}

<button class="create" on:click={create}>{ln[lk.APPS_CREATE]}</button>

<style>
	.load > :global(.loader) {
		width: 50px;
		height: 50px;
	}

	.apps {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 10px;
	}

	.description {
		margin-bottom: 10px;
	}

	.create {
		margin-top: 15px;
	}
</style>
