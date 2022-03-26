<script lang="ts">
	import { Permissions } from "../api";

	import { onMount } from "svelte";
	import { goto } from "$app/navigation";
	import { perm_docs } from "../config";

	export let perms: number;

	let perms_mapped = Permissions.PERMISSIONS.filter((p) => (perms & p.bitmask) !== 0);

	onMount(() => {});
</script>

<div class="perms">
	<h3>Permissions</h3>
	{#each perms_mapped as perm}
		<a target="_blank" href={perm_docs} class="perm border hhigh">
			<header>
				<p>{perm.id}:</p>
				<h4>{perm.name}</h4>
			</header>
			<p>{perm.description}</p>
		</a>
	{/each}
</div>

<style>
	.perms {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.perm > header {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.perm > header > h4 {
		margin-left: 5px;
	}

	.perm {
		text-decoration: none;
		padding: 5px;
		margin-top: 5px;
		width: 100%;
		transition: all 0.3s ease;
	}
</style>
