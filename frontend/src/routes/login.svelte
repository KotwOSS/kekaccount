<script lang="ts">
	import { goto } from "$app/navigation";

	import LoadingButton from "$components/button/loading.svelte";

	import { LangKey as lk, language as ln } from "$lib/lang";
	import { APIError, hash_password, Routes } from "$lib/api";
	import { client } from "$lib/client";
	import { regex } from "$lib/checker";

	let loading: boolean = false;
	let error: string;

	let uoe: string;
	let password: string;

	let params: URLSearchParams = new URLSearchParams(window.location.search);
	let redirect: string = params.get("redirect") || "/dash";

	async function purge_old_token() {
		let token = localStorage.getItem("token");
		if (token) {
			await Routes.Auth.Token.TERMINATE.send({ token }).finally(() => {
				localStorage.removeItem("token");
			});
		}
	}

	function submit(e) {
		e.preventDefault();

		loading = true;

		let hashed_password = hash_password(password);

		let identifier = regex.EMAIL.test(uoe)
			? { email: uoe, password: hashed_password }
			: { username: uoe, password: hashed_password };

		Routes.Auth.Token.CREATE.send({
			name: "login",
			permissions: -1,
			identifier
		})
			.then(async function (r) {
				let token = r.token;

				await purge_old_token();
				await client.login(token);

				console.log(client);

				localStorage.setItem("token", token);

				goto(redirect);
			})
			.catch((e) => {
				if (e instanceof APIError) {
					if (e.status === 401) error = ln[lk.ERROR_CREDENTIALS];
					else error = e.get_message();
				} else error = ln[lk.ERROR_CONNECTION];

				loading = false;
			});
	}
</script>

<div class="root fadein">
	<form class="card" on:submit={submit}>
		<h1 class="title">{ln[lk.LOGIN_TITLE]}</h1>
		<p class="description">{ln[lk.LOGIN_DESCRIPTION]}</p>
		{#if error}<p class="error break short">{error}</p>{/if}
		<input bind:value={uoe} type="text" id="uoe" placeholder={ln[lk.LOGIN_UOE]} />
		<input
			bind:value={password}
			type="password"
			id="password"
			placeholder={ln[lk.LOGIN_PASSWORD]}
		/>

		<p class="register">{@html ln[lk.LOGIN_REGISTER]}</p>

		<LoadingButton bind:loading>{ln[lk.LOGIN_SUBMIT]}</LoadingButton>
	</form>
</div>

<style>
	.card {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.title {
		margin-bottom: 5px;
	}

	.description {
		margin-bottom: 5px;
	}

	.register {
		margin-bottom: 15px;
	}

	#uoe {
		margin-top: 10px;
		margin-bottom: 10px;
	}

	#password {
		margin-bottom: 20px;
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
