<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import Loader from "../../components/loader.svelte";
    import Perms from "../../components/perms.svelte";
    import { APIError, get_identifier, get_login_redirect, Routes } from "../../api";

    let user: any;
    let loading = true;
    let token: string;

    let tokens: any[] = [];
    let tokens_loading: boolean = false;
    let tokens_error: string;

    let show_token: any;

    onMount(async ()=>{
        let delete_token = localStorage.getItem("delete_token");
        if(delete_token) {
            localStorage.removeItem("delete_token");

            let uoe = localStorage.getItem("uoe");
            let password = localStorage.getItem("password");
            if(uoe && password) {
                let identifier = get_identifier(uoe, password);
                try {
                    await Routes.Auth.Token.DELETE.send({id: delete_token, identifier});
                } catch(e) {
                    if(e instanceof APIError) {
                        tokens_error = e.get_message();
                    } else tokens_error = "Connection issues";
                }
                
            }
        }

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
            <p>Here you can see and manage all your tokens. <a href="/dash">Dashboard</a></p>

            {#if !user.verified}
                <p class="not-verified hint">IMPORTANT: Your email is not yet verified! Non verified users may get deleted.</p>
            {/if}

            {#if tokens_loading}
            <Loader />
            {:else}
            {#if show_token}
            <div class="show_token blend-in border">
                {#if show_token.active}
                <p class="hint">This is the token youre currently logged in with</p>
                {/if}

                <h2>{show_token.name}</h2>
                <p class="id">{show_token.id}</p>
                <Perms perms={show_token.perms} />

                <div class="actions">
                    <button class="delete" on:click={()=>{
                        localStorage.setItem("creds", "true");
                        localStorage.setItem("uoe", user.name);
                        localStorage.setItem("delete_token", show_token.id);
                        goto(get_login_redirect());
                    }}>Delete</button>
                    <button class="back" on:click={()=>show_token = undefined}>Back</button>
                </div>
            </div>
            {:else}
            <div class="tokens blend-in">
                {#each tokens as tk}
                    <div on:click={()=>{
                        show_token = tk;
                    }} class:active={tk.active} class="token border hhigh">
                        <h2>{tk.name}</h2>
                        <p>{tk.id}</p>
                    </div>
                {/each}
            </div>
            {/if}
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

    .refresh {
        margin-top: 15px;
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

    .show_token {
        padding: 10px;
        transition: all 0.3s ease;
        margin: 20px 0;
        width: 98%;
    }

    .show_token>.id {
        margin-top: 5px;
        margin-bottom: 10px;
    }

    .show_token>.hint {
        margin-bottom: 10px;
    }

    .show_token>.actions {
        display: flex;
        justify-content: center;
        align-items: center;
        margin-top: 20px;
    }

    .show_token>.actions>.delete {
        margin-right: 10px;
    }
</style>