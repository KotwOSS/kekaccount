<script lang="ts">
	import { client } from "$lib/client";
	import { LangKey as lk, language as ln } from "$lib/lang";
	import { Routes } from "$lib/api";

	import { goto } from "$app/navigation";

	let user = client.user;

	function delete_account() {
		client.confirm_access({
			callback: async function (identifier) {
				await Routes.User.DELETE.send({ identifier });
				client.logout();
				localStorage.removeItem("token");
				goto("/");
			},
			description: {
				warning: ln[lk.ACCOUNT_DELETE_CONFIRM]
			},
			type_to_confirm: `delete/${$user.name}`
		});
	}
</script>

<div class="root fadein">
	<h1 class="title">{ln[lk.ACCOUNT_TITLE]}</h1>
	{#if !$user.verified}
		<p class="hint short">{ln[lk.HINT_VERIFY]}</p>
	{/if}
	<p class="description">{ln[lk.ACCOUNT_DESCRIPTION]} {@html ln[lk.BACK_TO_DASHBOARD]}</p>
	<p>{$user.name}</p>
	<p class="break">{$user.id}</p>
	<p>{$user.email}</p>

	<button on:click={delete_account}>{ln[lk.ACCOUNT_DELETE]}</button>
</div>

<style>
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
