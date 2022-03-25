<script lang="ts">
    import { goto } from '$app/navigation';
    import Loader from '../components/loader.svelte';
    import { onMount } from "svelte";
    import { Routes, hash_password, get_identifier, APIError } from "../api";

    let uoe;
    let password;

    let username: string;
    let creds: boolean = false;

    let error: undefined|string;

    let success: boolean;

    let token: string;
    let valid: boolean = false;
    let user: any;

    let submit_loading: boolean = true;

    let params: URLSearchParams;
    let redirect: string;

    let force: boolean = false;

    onMount(async ()=>{
        params = new URLSearchParams(window.location.search);
        redirect = params.get("r")??"/dash";

        username = localStorage.getItem("uoe");
        creds = localStorage.getItem("creds")!==null;

        force = params.get("force")==="true";

        token = localStorage.getItem("token");
        if(token) {
            try {
                let info = await Routes.Auth.Token.INFO.send({token});
                console.log(info);
                if(info.token.perms === -1) {
                    valid = true;
                    user = await Routes.User.INFO.send({token});
                }
            } catch(e: any) {}
        }

        submit_loading = false;
    });

    async function login(e) {
        submit_loading = true;
        e.preventDefault();
 
        let hashed_password = hash_password(password.value);
        let identifier = get_identifier(uoe.value, hashed_password);

        success = true;
        if(!valid || force) await regenerate_token(identifier);

        if(success) goto(redirect);
        else submit_loading = false;
    }

    async function regenerate_token(identifier: any) {
        console.log("regenerate " + valid);
        localStorage.removeItem("token");
        try {
            let response = await Routes.Auth.Token.CREATE.send({
                name: "login", permissions: -1, identifier
            });
            localStorage.setItem("token", response.token);
            success = true;
        } catch(e: any) {
            success = false;
            if(e instanceof APIError) {
                error = e.get_message();
            } else error = "Connetion issues";
        }
    }

    function confirm_access() {
        let hashed_password = hash_password(password.value);
        localStorage.setItem("uoe", uoe.value);
        localStorage.setItem("password", hashed_password);

        localStorage.removeItem("creds");

        goto(redirect);
    }
</script>

<div class="root">
    <form on:submit={login} class="inner">
        <main class="blend-in">
            {#if creds}
            <h1>Confirm access</h1>
            <input bind:this={uoe} value={username??""} disabled={username!==null} type="text" placeholder="username or email">
            <input bind:this={password} type="password" placeholder="password">
            <button on:click={confirm_access}>Confirm</button>
            {:else}
            <h1>Login</h1>
            <input bind:this={uoe} type="text" placeholder="username or email">
            <input bind:this={password} type="password" placeholder="password">
            {#if error}
                <p class="error break">{error}</p>
            {/if}
            <p class="register">Don't have an account? <a href="/register">Register</a></p>
            {#if user && !force}
            <p class="hint">There already exists a valid token for the user {user.name}! A new token wont be generated! <a href="/logout">Logout</a></p>
            {/if}
            <button class:active={submit_loading} disabled={submit_loading}>
            {#if submit_loading}
                <Loader />
            {:else}
                Login
            {/if}
            </button>
            {/if}
        </main>
    </form>    
</div>

<style>
    @import url("../themes/default.css");

    .register {
        margin-top: 5px;
        margin-bottom: 5px;
    }

    .root :global(#loading) {
        width: 22px;
        height: 22px;
    }

    .root {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    h1 {
        margin-bottom: 10px;
    }

    input {
        margin-bottom: 5px;
        text-align: center;
    }

    button {
        margin-top: 10px;
    }

    form {
        overflow: hidden;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 100%;
        max-width: 450px;
        padding: 30px 10px;
        
    }

    main {
        overflow: hidden;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 100%;
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

    .error {
        animation: error-blend-in 0.3s ease forwards;
        text-align: center;
    }

    .hint {
        margin-bottom: 5px;
    }
</style>