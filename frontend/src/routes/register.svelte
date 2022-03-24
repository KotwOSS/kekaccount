<script lang="ts">
    import { goto } from '$app/navigation';
    import Loader from '../components/loader.svelte';
    import { APIError, hash_password, Routes } from "../api";

    let username;
    let email;
    let password;
    let password_repeat;

    let error;

    let submit_loading;

    function register(e) {
        submit_loading = true;

        e.preventDefault();

        if(password.value == password_repeat.value) {
            let hashed_password = hash_password(password.value);

            console.log(hashed_password);

            Routes.Auth.REGISTER.send({
                username: username.value, 
                email: email.value, 
                password: hashed_password,
                avatar: ""
            }).then(()=>goto("/email_sent"))
            .catch(e=>{
                submit_loading = false;
                if(e instanceof APIError) {
                    error = e.get_message();
                } else error = "Connection issues";
            })
        } else {
            error = "Passwords don't match";
            submit_loading = false;
        }
    }

</script>

<div class="root">
    <form on:submit={register} class="inner">
        <main class="blend-in">
            <h1>Register</h1>
            <input bind:this={username} type="text" placeholder="username">
            <input bind:this={email} type="text" placeholder="email">
            <input bind:this={password} type="password" placeholder="password">
            <input bind:this={password_repeat} type="password" placeholder="repeat password">
            {#if error}
                <p class="error break">{error}</p>
            {/if}
            <p class="login">Already have an account? <a href="/login">Login</a></p>
            <button class:active={submit_loading} disabled={submit_loading}>
            {#if submit_loading}
                <Loader />
            {:else}
                Register
            {/if}
            </button>
        </main>
    </form>    
</div>

<style>
    @import url("../themes/default.css");

    .login {
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
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 100%;
        max-width: 450px;
        padding: 30px 10px;
        
    }

    main {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 100%;
    }

    @keyframes error-blend-in {
        0% { 
            height: 0;
            opacity: 0;
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
</style>