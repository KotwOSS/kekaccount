<script lang="ts">
	import { client } from "$lib/client";
	import { LangKey as lk, language } from "$lib/lang";
	import T from "$components/translate.svelte";
    import { goto } from "$app/navigation";

	let user = client.user;

    function delete_account() {
        client.confirm_access({
            callback: function(identifier) {
                goto("/dash/account");
                console.log(identifier);
            }, 
            description: {
                warning: language[lk.ACCOUNT_DELETE_CONFIRM]
            },
            type_to_confirm: `delete/${$user.name}`
        });
    }
</script>

<div class="root fadein">
	<h1 class="title"><T k={lk.ACCOUNT_TITLE} /></h1>
	{#if !$user.verified}
		<p class="hint short"><T k={lk.HINT_VERIFY} /></p>
	{/if}
	<p class="description"><T k={lk.ACCOUNT_DESCRIPTION} /> <T k={lk.BACK_TO_DASHBOARD} /></p>
	<p>{$user.name}</p>
	<p class="break">{$user.id}</p>
	<p>{$user.email}</p>

    <button on:click={delete_account}><T k={lk.ACCOUNT_DELETE} /></button>
</div>

<style>
	.root {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.description {
		margin-bottom: 10px;
		margin-top: 5px;
	}
</style>
