<script lang="ts">
	import { goto } from "$app/navigation";

	import LoadingButton from "$components/button/loading.svelte";

	import { LangKey as lk, language as ln } from "$lib/lang";
	import { APIError, hash_password } from "$lib/api";
	import { client, type Confirm } from "$lib/client";
	import { regex } from "$lib/checker";

	import { fade } from "svelte/transition";

	let loading: boolean = false;
	let error: string;

	let user = client.user;
	let authorized = client.authorized;

	let confirm: Confirm = client.confirm;
	console.log(confirm);

	let type_to_confirm: string;

	let disabled: boolean = false;
	$: disabled = confirm && confirm.type_to_confirm && confirm.type_to_confirm !== type_to_confirm;

	let uoe: string = $user ? $user.name : null;
	let password: string;

	async function submit(e) {
		e.preventDefault();

		loading = true;

		let hashed_password = hash_password(password);

		let identifier = regex.EMAIL.test(uoe)
			? { email: uoe, password: hashed_password }
			: { username: uoe, password: hashed_password };

		try {
			await confirm.callback(identifier);
		} catch (e) {
			if (e instanceof APIError) {
				if (e.status === 401) error = ln[lk.ERROR_CREDENTIALS];
				else error = e.get_message();
			} else if (e instanceof TypeError) error = ln[lk.ERROR_CONNECTION];
			else error = e.message;
			loading = false;
		}
	}

	if (!confirm) goto("/dash");
</script>

<div class="root" in:fade={{ duration: 200 }}>
	<form class="card" on:submit={submit}>
		{#if confirm}
			<h1 class="title">{ln[lk.CONFIRM_TITLE]}</h1>
			<p class="description">{ln[lk.CONFIRM_DESCRIPTION]}</p>
			{#if error}<p class="error break short">{error}</p>{/if}
			<input
				disabled={$authorized}
				bind:value={uoe}
				type="text"
				id="uoe"
				placeholder={ln[lk.LOGIN_UOE]}
			/>
			<input
				bind:value={password}
				type="password"
				id="password"
				placeholder={ln[lk.LOGIN_PASSWORD]}
			/>

			{#if confirm.description.hint}
				<p class="hint short">{confirm.description.hint}</p>
			{:else if confirm.description.warning}
				<p class="warning short">{confirm.description.warning}</p>
			{/if}

			{#if confirm.type_to_confirm}
				<div class="type_to_confirm">
					<p class="short">
						<span class="hint">{ln[lk.CONFIRM_TYPE_TO_CONFIRM]}</span>
						{confirm.type_to_confirm}
					</p>
					<input bind:value={type_to_confirm} type="text" />
				</div>
			{/if}

			<div class="spacer" />

			<LoadingButton bind:loading bind:disabled>{ln[lk.CONFIRM_SUBMIT]}</LoadingButton>
		{/if}
	</form>
</div>

<style>
	.type_to_confirm {
		margin-top: 10px;
		margin-bottom: 10px;
	}

	.type_to_confirm > p {
		margin-bottom: 10px;
	}

	.card {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.description {
		margin-bottom: 5px;
	}

	#uoe {
		margin-top: 10px;
		margin-bottom: 10px;
	}

	#password {
		margin-bottom: 20px;
	}

	.spacer {
		margin-bottom: 10px;
	}

	.root {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.root > :global(.error) {
		margin-top: 5px;
	}
</style>
