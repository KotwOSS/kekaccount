<script lang="ts">
	import { goto } from "$app/navigation";
	import { Routes } from "$lib/api";
	import { client } from "$lib/client";
	import { Tokens } from "$lib/store";
	import { LangKey as lk, language as ln } from "$lib/lang";
	import PermissionSelect from "$components/table/permission/selector.svelte";

	let name: string;
	let permissions: number = 0;

	function create(e) {
		e.preventDefault();

		client.confirm_access({
			callback: async function (identifier) {
				let result = await Routes.Auth.Token.CREATE.send({
					name,
					permissions,
					identifier
				});

				Tokens.expand.update(function (arr) {
					arr[result.id] = true;
					return arr;
				});

				Tokens.tokens.update(function (arr) {
					arr[result.id] = result.token;
					return arr;
				});

				goto("/dash/tokens");
			},
			description: {}
		});
	}
</script>

<form class="inner" on:submit={create}>
	<h1 class="title">{ln[lk.TOKENS_CREATE]}</h1>
	<p class="description">{@html ln[lk.TOKENS_CREATE_DESCRIPTION]}</p>
	<input class="name" type="text" bind:value={name} placeholder={ln[lk.TOKENS_CREATE_NAME]} />
	<PermissionSelect bind:permissions />
	<button class="create">{ln[lk.TOKENS_CREATE_SUBMIT]}</button>
</form>

<style>
	.inner > :global(.loader) {
		width: 50px;
		height: 50px;
	}

	.inner {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.name {
		margin-bottom: 10px;
	}

	.description {
		margin-bottom: 10px;
	}

	.create {
		margin-top: 10px;
	}
</style>
