<script lang="ts">
    import CryptoJS from "crypto-js";
    import { goto } from '$app/navigation';
    import { api_base } from "../config";

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
            let hashed_password = CryptoJS.SHA512(password.value).toString();

            console.log(hashed_password);

            fetch(`${api_base}/auth/register`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    "username": username.value,
                    "email": email.value,
                    "password": hashed_password
                })
            }).then(r=>{
                if(r.status === 200) {
                    r.json().then(j=>goto("/email_sent"));
                } else r.json().then(j=>{
                    error = j.message;
                    submit_loading = false;
                });
            })
            .catch(e=>{
                error = "Connection errors! Please contact an admin";
                submit_loading = false;
            });
        } else {
            error = "Passwords don't match";
            submit_loading = false;
        }
    }

</script>

<main class="root">
    <form on:submit={register} class="inner">
        <h1>Register</h1>
        <input bind:this={username} type="text" placeholder="username">
        <input bind:this={email} type="text" placeholder="email">
        <input bind:this={password} type="password" placeholder="password">
        <input bind:this={password_repeat} type="password" placeholder="repeat password">
        {#if error}
            <p class="error">{error}</p>
        {/if}
        <button class={submit_loading?"active":""} disabled={submit_loading}>
        {#if submit_loading}
            <div id="loading">
                <svg viewBox="0 0 100 100">
                    <defs>
                        <filter id="shadow">
                        <feDropShadow dx="0" dy="0" stdDeviation="1.5" 
                            flood-color="#ffffff"/>
                        </filter>
                    </defs>
                    <circle id="spinner" style="fill:transparent;stroke:#ffffff;stroke-width: 7px;stroke-linecap: round;filter:url(#shadow);" cx="50" cy="50" r="45"/>
                </svg>
            </div>
        {:else}
            Register
        {/if}
        </button>
    </form>    
</main>

<style>
    @import url("../themes/default.css");


    #loading {
        width: 22px;
        height: 22px;
    }

    @keyframes animation {
        0% {
            stroke-dasharray: 1 98;
            stroke-dashoffset: -105;
        }
        50% {
            stroke-dasharray: 80 10;
            stroke-dashoffset: -160;
        }
        100% {
            stroke-dasharray: 1 98;
            stroke-dashoffset: -300;
        }
    }

    #spinner {
        transform-origin: center;
        animation-name: animation;
        animation-duration: 1.2s;
        animation-timing-function: cubic-bezier;
        animation-iteration-count: infinite;
    }

    main {
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
        color: rgb(255, 42, 42);
        animation: error-blend-in 0.3s ease forwards;
    }
</style>