<script lang="ts">
    import { goto } from "$app/navigation";
    import Loader from "../../components/loader.svelte";
    import { APIError, get_login_redirect, Routes } from "../../api";
    import { onMount } from "svelte";

    let user: any;
    let loading = true;
    let token: string;

    let tokens: any[] = [];
    let tokens_loading: boolean = false;
    let tokens_error: string;

    onMount(async ()=>{
        token = localStorage.getItem("token");
        if(token) {
            try {
                let info = await Routes.Auth.Token.INFO.send({token});
                console.log(info);
                if(info.token.perms === -1) {
                    user = await Routes.User.INFO.send({token});
                    loading = false;

                    refresh_tokens();
                }
            } catch(e) { }
        }

        if(loading) goto(get_login_redirect());
    });

    async function refresh_tokens() {
        tokens_loading = true;
        try {
            tokens = await Routes.Auth.Token.LIST.send({token});
        } catch(e) { 
            if(e instanceof APIError) {
                tokens_error = e.get_message();
            } else tokens_error = "Connection issues";
        }
        tokens_loading = false;
    }
</script>

<div class="root">
    <div class="inner">
        {#if loading}
        <Loader />
        {:else}
        <main class="blend-in">
            <h1>Tokens</h1>
            <p>Here you can see and manage all your tokens.</p>

            {#if !user.verified}
                <p class="not-verified hint">IMPORTANT: Your email is not yet verified! Non verified users may get deleted.</p>
            {/if}

            {#if tokens_loading}
            <Loader />
            {:else}
            <div class="tokens blend-in">
                {#each tokens as tk}
                    <div class:active={tk.active} class="token border hhigh">
                        <h2>{tk.name}</h2>
                        <p>{tk.id}</p>
                    </div>
                {/each}
            </div>
            {/if}

            {#if tokens_error}
                <p class="error break">{tokens_error}</p>
            {/if}

            <button on:click={refresh_tokens} class="refresh" class:active={tokens_loading} disabled={tokens_loading}>
                {#if tokens_loading}
                    <Loader />
                {:else}
                    Refresh
                {/if}
            </button>
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

    .tokens {
        margin: 10px 0;
        margin-bottom: 10px;
        display: flex;
        flex-direction: column;

    }

    .token {
        padding: 10px;
        cursor: pointer;
        transition: all 0.3s ease;
        margin-top: 8px;
    }
</style>