<script lang="ts">
    import { goto } from "$app/navigation";
    import Loader from "../../components/loader.svelte"
    import { APIError, get_login_redirect, Routes } from "../../api";
    import { onMount } from "svelte";

    let user;

    let loading = true;

    onMount(async ()=>{
        let token = localStorage.getItem("token")
        if(token) {
            try {
                let info = await Routes.Auth.Token.INFO.send({token});
                console.log(info);
                if(info.token.perms === -1) {
                    user = await Routes.User.INFO.send({token});
                    loading = false;
                } else goto(get_login_redirect())
            } catch(e) { 
                console.log("ERROR:" + (e as APIError).get_message())
                goto(get_login_redirect()) 
            }
        } else goto(get_login_redirect())
    });
</script>

<div class="root">
    <div class="inner">
        {#if loading}
        <Loader />
        {:else}
        <main class="blend-in">
            <h1>Dashboard</h1>
            <p>This is the dashboard. Here you can manage your account!</p>
            <div class="profile">
                <p class="username">{user.name}</p>
                <p class="email">{user.email}</p>
                <p class="id">{user.id}</p>
                {#if !user.verified}
                <p class="not-verified">Your email is not yet verified! Non verified users may get deleted.</p>
                {/if}
            </div>
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
        max-width: 1000px;
    }

    main {
        display: flex;
        align-items: center;
        flex-direction: column;
    }
</style>