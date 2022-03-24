<script lang="ts">
    import { goto } from "$app/navigation";
    import Loader from "../../components/loader.svelte";
    import Setting from "../../components/setting.svelte";
    import { APIError, get_login_redirect, Routes } from "../../api";
    import isEqual from "lodash.isequal";
    import { onMount } from "svelte";
    import { default_avatar } from "../../config";

    let user: any;
    let changes: any;
    let loading = true;
    let token: string;

    let update_loading = false;
    let update_error: string;

    let changed: boolean = false;
    $: changed = profile_is_changed(changes, user)

    function profile_is_changed(changes, user) {
        return !isEqual(changes, user);
    }

    onMount(async ()=>{
        token = localStorage.getItem("token");
        if(token) {
            try {
                let info = await Routes.Auth.Token.INFO.send({token});
                console.log(info);
                if(info.token.perms === -1) {
                    user = await Routes.User.INFO.send({token});
                    changes = {...user};
                    loading = false;
                } else goto(get_login_redirect())
            } catch(e) { 
                console.log("ERROR:" + (e as APIError).get_message())
                goto(get_login_redirect()) 
            }
        } else goto(get_login_redirect())
    });

    function hidden_email(email: string) {
        let p = email.split("@");
        return `${p[0].substring(0, 2)}${"*".repeat(p[0].length-2)}@${p[1]}`;
    }

    function username_changed(e) {
        changes.name = e.target.value;
    }

    async function update() {
        update_loading = true;
                    
        let update = Object.entries(changes).filter(e=>e[1]!=user[e[0]]).reduce((m, e)=>(m[e[0]]=e[1], m), {});

        try {
            await Routes.User.UPDATE.send({ token, changes: update });
            user = {...changes};
        } catch(e) {
            if(e instanceof APIError) {
                update_error = e.get_message();
            } else update_error = "Connection issues"
        }

        update_loading = false;
    }
</script>

<div class="root">
    <div class="inner">
        {#if loading}
        <Loader />
        {:else}
        <main class="blend-in">
            <h1>Account</h1>
            <p>Here you can manage your account. <a href="/dash">Dashboard</a></p>

            {#if !user.verified}
                <p class="not-verified hint">IMPORTANT: Your email is not yet verified! Non verified users may get deleted.</p>
            {/if}

            <div class="profile border">
                <div class="public">
                    <img src={user.avatar==""?default_avatar:user.avatar} alt="Loading avatar..." class="avatar">
                    <div>
                        <Setting on:input={username_changed} clz="username" content={changes.name} placeholder="Username"/>
                        <!-- <h2 class="username break">{user.name}</h2> -->
                        <p class="id break">{user.id}</p>
                    </div>
                </div>
                
                {#if update_error}
                <p class="error break">{update_error}</p>
                {/if}

                <div class="update">    
                    <button class:active={update_loading} disabled={!changed || update_loading} on:click={update}>
                        {#if update_loading}
                            <Loader />
                        {:else}
                            Update
                        {/if}
                    </button>
                    <button disabled={!changed || update_loading} on:click={()=>{
                        changes = {...user};
                    }}>Reset</button>
                </div>
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

    .profile :global(#loading) {
        width: 22px;
        height: 22px;
    }

    .inner {
        padding: 40px 0;
        display: flex;
        align-items: center;
        flex-direction: column;
        width: 100%;
        max-width: 600px;
    }

    .profile {
        margin-top: 20px;
        padding: 10px;
    }

    .profile>.public {
        display: flex;
        align-items: center;
        margin-bottom: 20px;
    }

    .profile>.public>.avatar {
        width: 50px;
        height: 50px;
        margin-right: 10px;
    }

    .profile>.public>div {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    }

    .profile>.public :global(.username) {
        text-align: left;
        width: 100%;
    }

    main {
        display: flex;
        align-items: center;
        flex-direction: column;
    }

    .update {
        display: flex;
        align-items: center;
        justify-content: center;
        margin-top: 10px;
    }

    .update :nth-child(1) {
        margin-right: 10px;
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

    .not-verified {
        margin: 0 10px;
        margin-top: 10px;
    }

    p {
        margin: 0 10px;
    }
</style>