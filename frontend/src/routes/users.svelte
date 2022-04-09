<script lang="ts">
	import { goto, afterNavigate } from "$app/navigation";
	import Loader from "$components/loader.svelte";
	import UsersTable from "$components/table/users.svelte";
	import { APIError, Routes } from "$lib/api";
	import { LangKey as lk, language as ln } from "$lib/lang";

	let params = new URLSearchParams(window.location.search);
	let query = params.get("q");

	let username: string = query;

	let has_more = false;
	let offset = 0;

	let loading: boolean = false;
	let users: any[] = [];

	let error: string;

	let last_input: number;

	async function show_more() {
		offset += 10;

		await refresh();

		if (users.length % 10 !== 0) has_more = false;
	}

	function search() {
		goto("/users?" + new URLSearchParams({ q: username }).toString(), {
			keepfocus: true,
			noscroll: true
		});

		users = [];
		offset = 0;

		query = username;

		if (query) refresh();

		if (users.length % 10 !== 0) has_more = false;
	}

	async function refresh() {
		loading = true;
		error = null;
		users = users.concat(
			await Routes.Users.SEARCH.send({ name: query, offset })
				.catch(function (e) {
					if (e instanceof APIError) error = e.get_message();
					else error = ln[lk.ERROR_CONNECTION];
				})
				.finally(function () {
					loading = false;
				})
		);
	}

	function tick() {
		if (query === username) return;

		let now = new Date().getTime();
		if (now - last_input > 350) {
			search();
		}
	}

	function on_input(e) {
		last_input = new Date().getTime();
	}

	if (query) search();

	setInterval(tick, 50);
</script>

<form
	class="root fadein"
	on:submit={function (e) {
		e.preventDefault();
		search();
	}}
>
	<h1>{ln[lk.USERS_TITLE]}</h1>
	<p>{ln[lk.USERS_DESCRIPTION]}</p>
	<div class="search">
		<input
			type="text"
			bind:value={username}
			on:input={on_input}
			placeholder={ln[lk.USERS_USERNAME]}
		/>
		<button>{ln[lk.USERS_SEARCH]}</button>
	</div>

	{#if error}
		<p class="error break">{error}</p>
		<button on:click={refresh}>Refresh</button>
	{:else}
		<UsersTable bind:loading bind:data={users} bind:has_more {show_more} />
	{/if}
</form>

<style>
	.root {
		display: flex;
		flex-direction: column;
		padding: 30px 0;
		justify-content: center;
		align-items: center;
		gap: 10px;
	}

	.root > :global(.loader) {
		width: 50px;
		height: 50px;
	}

	.search {
		margin-bottom: 10px;
	}
</style>
