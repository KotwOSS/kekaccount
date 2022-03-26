<script lang="ts">
	import { goto } from "$app/navigation";
	import Loader from "../../components/loader.svelte";
	import { get_login_redirect, Routes } from "../../api";
	import { onMount } from "svelte";

	let user: any;
	let loading = true;
	let token: string;

	onMount(async () => {
		token = localStorage.getItem("token");
		if (token) {
			try {
				let info = await Routes.Auth.Token.INFO.send({ token });
				console.log(info);
				if (info.token.perms === -1) {
					user = await Routes.User.INFO.send({ token });
					loading = false;
				}
			} catch (e) {}
		}

		if (loading) goto(get_login_redirect());
	});
</script>

<div class="root">
	<div class="inner">
		{#if loading}
			<Loader />
		{:else}
			<main class="blend-in">
				<h1>Dashboard</h1>
				<p>This is the dashboard. Here you can manage your account!</p>

				{#if !user.verified}
					<p class="not-verified hint">
						IMPORTANT: Your email is not yet verified! Non verified users may get deleted.
					</p>
				{/if}

				<div class="categories">
					<a href="/dash/account" class="border category hhigh">
						<h1>Account</h1>
						<p>Manage private and public account details</p>
					</a>

					<a href="/dash/tokens" class="border category hhigh">
						<h1>Tokens</h1>
						<p>Manage tokens</p>
					</a>
				</div>
			</main>
		{/if}
	</div>
</div>

<style>
	@import url("../../themes/default.css");

	.root :global(#loading) {
		max-width: 50px;
		max-height: 50px;
	}

	.root {
		width: 100%;
		height: 100%;
		padding: 10px 0;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.inner {
		padding: 40px 0;
		display: flex;
		align-items: center;
		flex-direction: column;
		width: 100%;
		max-width: 600px;
	}

	main {
		display: flex;
		align-items: center;
		flex-direction: column;
	}

	@keyframes error-blend-in {
		0% {
			opacity: 0;
			height: 0;
		}
		100% {
			opacity: 1;
			height: 20px;
			margin-top: 8px;
			margin-bottom: 5px;
		}
	}

	.not-verified {
		margin: 0 10px;
		margin-top: 10px;
	}

	p {
		margin: 0 10px;
	}

	.category {
		padding: 10px 10px;
		text-decoration: none;
		transition: 0.3s ease;
		margin-top: 10px;
	}

	.categories {
		margin-top: 5px;
		display: flex;
		flex-direction: column;
	}
</style>
