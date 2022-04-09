<script lang="ts">
	import Loader from "$components/loader.svelte";
	import Navbar from "$components/navbar.svelte";
	import Footer from "$components/footer.svelte";
	import * as lang from "$lib/lang";
	import { client } from "$lib/client";
	import { Routes } from "$lib/api";
	import ApiError from "$components/error/api.svelte";

	let loading: boolean = true;
	let api_error: boolean = false;

	async function main() {
		await lang.init();

		Routes.PING.send({})
			.then(function () {
				let token = localStorage.getItem("token");
				if (token)
					client
						.login(token)
						.catch(function () {
							localStorage.removeItem("token");
						})
						.finally(function () {
							loading = false;
						});
				else loading = false;
			})
			.catch(function (e) {
				api_error = true;
			});
	}

	main();

	function onscroll(e) {
		let scroll = e.target.scrollingElement.scrollTop;
		detach_navbar = scroll >= 50;
	}

	let detach_navbar = false;
</script>

<svelte:window on:scroll={onscroll} />

<div id="app" class="fadein">
	{#if api_error}
		<ApiError />
	{:else if loading}
		<Loader />
	{:else}
		<Navbar detach={detach_navbar} />
		<slot />
		<Footer />
	{/if}
</div>

<style>
	#app > :global(.loader) {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 50px;
		height: 50px;
	}

	#app {
		width: 100%;
	}
</style>
