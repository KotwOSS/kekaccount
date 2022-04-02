<script lang="ts">
	import { goto } from "$app/navigation";
	import { get_login_redirect } from "$lib/api";

	import { client } from "$lib/client";

	import { get_store_value } from "svelte/internal";

	let show: boolean = false;

	if (!get_store_value(client.authorized)) goto(get_login_redirect());
	else show = true;
</script>

{#if show}<slot />{/if}
