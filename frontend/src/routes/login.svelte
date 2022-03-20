<script lang="ts">
    import CryptoJS from "crypto-js";
    import { goto } from '$app/navigation';
    import { api_base } from "../config";
    import { onMount } from "svelte";

    let username_or_email;
    let password;

    let error;

    let submit_loading;

    let verify_id

    onMount(()=>{
        let params = new URLSearchParams(window.location.search);

        verify_id = params.get("verify");

        if(!verify_id && localStorage.getItem("token")) goto("/dash");
    });

    function isEmail(email) {
        const email_regex = /^\w+[\+\.\w-]*@([\w-]+\.)*\w+[\w-]*\.([a-z]{2,18}|\d+)$/g;
        return email_regex.test(email);
    }

    function login(e) {
        submit_loading = true;

        e.preventDefault();

        let hashed_password = CryptoJS.SHA512(password.value).toString();

        let uoe = username_or_email.value;
        let acc_identifier = isEmail(uoe)?{"email": uoe}:{"username": uoe};


        console.log(hashed_password);

        if(verify_id) {
            fetch(`${api_base}/auth/verify`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    "password": hashed_password,
                    "id": verify_id,
                    ...acc_identifier
                })
            }).then(r=>{
                if(r.status === 200) {
                    submit_loading = false;
                } else r.json().then(j=>{
                    error = j.message;
                    submit_loading = false;
                });
            })
            .catch(e=>{
                error = "Connection errors! Please contact an admin";
                submit_loading = false;
            });
        }

        if(!localStorage.getItem("token")) fetch(`${api_base}/auth/token/create`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "password": hashed_password,
                "name": "login",
                "permissions": -1,
                ...acc_identifier
            })
        }).then(r=>{
            if(r.status === 200) {
                r.json().then(j=>{
                    submit_loading = false;
                    localStorage.setItem("token", j.token);
                    goto("/dash");
                })
            } else r.json().then(j=>{
                error = j.message;
                submit_loading = false;
            });
        })
        .catch(e=>{
            error = "Connection errors! Please contact an admin";
            submit_loading = false;
        });
    }
</script>

<main class="root">
    <form on:submit={login} class="inner">
        <h1>Login</h1>
        {#if verify_id}
        <p>This will verify your account!</p>
        {/if}
        <input bind:this={username_or_email} type="text" placeholder="username or email">
        <input bind:this={password} type="password" placeholder="password">
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
            Login
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

    p {
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

    @keyframes error-blend-in {
        0% { 
            opacity: 0;
        }
        100% { 
            opacity: 1;
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