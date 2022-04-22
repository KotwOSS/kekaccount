<script lang="ts">
	import Category from "$components/category.svelte";
	import Link from "$components/link.svelte";

	import { get_login_redirect } from "$lib/api";
	import { client } from "$lib/client";

	import { goto } from "$app/navigation";

	import { fade } from "svelte/transition";
	import { get_store_value } from "svelte/internal";

	let show: boolean = false;

	if (!get_store_value(client.authorized)) goto(get_login_redirect());
	else show = true;
</script>

{#if show}
	<div class="root" in:fade={{ duration: 200 }}>
		<main>
			<div class="side">
				<Category expand={true}>
					<Link href="/dash/" exact slot="title">Dashboard</Link>
					<svelte:fragment slot="sub">
						<Link href="/dash/account/">Account</Link>
						<Link href="/dash/apps/">Apps</Link>
						<Link href="/dash/tokens/">Tokens</Link>
					</svelte:fragment>
				</Category>
			</div>
			<div class="content" in:fade={{ duration: 200 }}>
				<slot />
			</div>
		</main>
	</div>
{/if}

<style>
	main {
		display: flex;
		flex-direction: row;
		text-align: left;
		margin-left: auto;
		margin-right: auto;
		width: 100%;
		height: calc(100vh - var(--nav-height));
		max-width: 1300px;
	}

	.content {
		padding: 40px 32px;
		overflow-y: scroll;
		width: 100%;
		min-height: 100%;
	}

	.side {
		width: 350px;
		overflow-y: scroll;
		padding: 40px 20px;
		height: 100%;
		border-right: 1px solid var(--nav-border);
	}

	@media (max-width: 900px) {
		.side {
			display: none;
		}
	}
</style>
