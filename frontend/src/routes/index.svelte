<script>
	import { goto } from "$app/navigation";
	import { is_authorized } from "../api";
	import { onMount } from "svelte";

	onMount(async () => {
		if (await is_authorized()) goto("/dash");
		else goto("/login");
	});
</script>

<div class="root" />

<style>
	@import url("../themes/default.css");

	.root {
		height: 100%;
		width: 100%;
	}
</style>
