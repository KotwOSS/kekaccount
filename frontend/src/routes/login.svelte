<script lang="ts">
    import { goto } from '$app/navigation';
    import Loader from '../components/loader.svelte';
    import { onMount } from "svelte";
    import { Routes, hash_password, get_identifier, APIError } from "../api";

    let uoe;
    let password;

    let error: undefined|string;

    let success: boolean;

    let submit_loading: boolean;

    let params: URLSearchParams;

    onMount(()=>{
        params = new URLSearchParams(window.location.search);

        localStorage.removeItem("uoe");
        localStorage.removeItem("password");
    });

    async function login(e) {
        submit_loading = true;
        e.preventDefault();
 
        let hashed_password = hash_password(password.value);
        let identifier = get_identifier(uoe.value, hashed_password);

        let redirect = params.get("r")??"/dash";

        if(localStorage.getItem("creds")) {
            localStorage.removeItem("creds");
            localStorage.setItem("uoe", uoe.value);
            localStorage.setItem("password", hashed_password);
        }

        let token = localStorage.getItem("token");

        if(token) {
            try {
                let info = await Routes.Auth.Token.INFO.send({token});
                if(info.token.permission === -1) await regenerate_token(identifier);
                else success = true;
            } catch(e: any) {
                await regenerate_token(identifier);
            }
        } else await regenerate_token(identifier);

        submit_loading = false;

        if(success) goto(redirect);
    }

    async function regenerate_token(identifier: any) {
        localStorage.removeItem("token");
        try {
            let response = await Routes.Auth.Token.CREATE.send({
                name: "login", permissions: -1, identifier
            });
            localStorage.setItem("token", response.token);
            success = true;
        } catch(e: any) {
            if(e instanceof APIError) {
                success = false;
                error = e.get_message();
            }
        }
    }
</script>

<div class="root">
    <form on:submit={login} class="inner">
        <main class="blend-in">
            <h1>Login</h1>
            <input bind:this={uoe} type="text" placeholder="username or email">
            <input bind:this={password} type="password" placeholder="password">
            {#if error}
                <p class="error">{error}</p>
            {/if}
            <p class="register">Don't have an account? <a href="/register">Register</a></p>
            <button class={submit_loading?"active":""} disabled={submit_loading}>
            {#if submit_loading}
                <Loader />
            {:else}
                Login
            {/if}
            </button>
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
        color: rgb(255, 42, 42);
        animation: error-blend-in 0.3s ease forwards;
        word-wrap: break-word;
        word-break: break-all;
        line-break: strict;
        text-align: center;
    }
</style>