<script lang="ts">
	import { onMount } from "svelte";
	import { goto } from "$app/navigation";
	import Loader from "../../components/loader.svelte";
	import { get_login_redirect, Routes } from "../../api";

	let user: any;
	let loading = true;
	let token: string;

	let show_token: any;

	onMount(async () => {
		let params = new URLSearchParams(window.location.search);

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
				<h1>Apps</h1>
				<p>Here you can see and manage all your apps. <a href="/dash">Dashboard</a></p>

				{#if !user.verified}
					<p class="not-verified hint">
						IMPORTANT: Your email is not yet verified! Non verified users may get deleted.
					</p>
				{/if}

				<h2>Not implemented yet</h2>
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
		width: 100%;
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
</style>
