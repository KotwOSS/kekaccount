<script lang="ts">
	import Loader from "$components/loader.svelte";
	import { default_avatar } from "$lib/config";

	export let data: any[];
	export let has_more: boolean;
	export let show_more: () => void;

	export let loading: boolean = false;

	let data_length = data.length;
</script>

<div class="table fadein">
	{#each data as user, i}
		<div class="user" class:first={i === 0} class:last={i === data_length - 1}>
			<div class="inner">
				<img class="avatar" src={user.avatar === "" ? default_avatar : user.avatar} alt="" />
				<div class="wrapper">
					<p>{user.name}</p>
					<p class="id">{user.id}</p>
				</div>
			</div>
		</div>
	{/each}

	{#if loading}
		<Loader />
	{:else if has_more}
		<button on:click={show_more}>Show more</button>
	{/if}
</div>

<style>
	.table {
		display: flex;
		align-items: center;
		flex-direction: column;
	}

	.table > :global(.loader) {
		width: 30px;
		height: 30px;
	}

	@keyframes slow {
		from {
			height: 0;
		}
	}

	.inner {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
		padding: 5px 15px;
		margin-bottom: 10px;
	}

	.user.last .inner {
		border-bottom: none;
	}

	.user.last {
		border-bottom: none;
	}

	.wrapper {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		row-gap: 10px;
	}

	.avatar {
		border-radius: 90px;
		width: 60px;
		height: 60px;
		margin-right: 10px;
	}
</style>
