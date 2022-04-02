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

	let apps: any;

	Routes.Apps.LIST.send({ token: client.token })
		.then((r) => (apps = r))
		.catch((e) => {
			if (e instanceof APIError) error = e.get_message();
			else error = language[lk.ERROR_CONNECTION];
		});

	function create() {
		goto("/dash/apps/create");
	}
</script>

<div class="root fadein">
	<h1 class="title"><T k={lk.APPS_TITLE} /></h1>
	{#if !$user.verified}
		<p class="hint short"><T k={lk.HINT_VERIFY} /></p>
	{/if}
	<p class="description"><T k={lk.APPS_DESCRIPTION} /> <T k={lk.BACK_TO_DASHBOARD} /></p>
	<Error {error} />

	{#if apps}
		<div class="apps">
			{#each apps as app}
				<div class="app card">
					<p class="app-name">{app.name}</p>
					<p class="app-id">{app.id}</p>
				</div>
			{/each}
		</div>
	{:else}
		<Loader />
	{/if}
	<button on:click={create}><T k={lk.APPS_CREATE} /></button>
</div>

<style>
	.root > :global(.loader) {
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
