<script lang="ts">
    import { goto } from "$app/navigation";
    import Loader from "../../components/loader.svelte";
    import Setting from "../../components/setting.svelte";
    import { APIError, get_login_redirect, Routes } from "../../api";
    import { onMount } from "svelte";
    import { default_avatar } from "../../config";

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

    function hidden_email(email: string) {
        let p = email.split("@");
        return `${p[0].substring(0, 2)}${"*".repeat(p[0].length-2)}@${p[1]}`;
    }
</script>

<div class="root">
    <div class="inner">
        {#if loading}
        <Loader />
        {:else}
        <main class="blend-in">
            <h1>Dashboard</h1>
            <p>This is the dashboard. Here you can manage your account!</p>
            <div class="profile border">
                <div class="public">
                    <img src={user.avatar==""?default_avatar:user.avatar} alt="Loading avatar..." class="avatar">
                    <div>
                        <h2 class="username break">{user.name}</h2>
                        <p class="id break">{user.id}</p>
                    </div>
                </div>
                
                
                <Setting 
                    test_content={(content)=>content==="123"?"kekw":undefined}
                    get_content={(hidden)=>hidden?hidden_email(user.email):user.email} 
                    allow_hide={true}/>

                <Setting 
                    test_content={(content)=>content==="123"?"kekw":undefined}
                    get_content={(hidden)=>hidden?hidden_email(user.email):user.email} 
                    allow_hide={false}/>
                <!-- <p class="email">{user.email}</p> -->
                
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
        justify-content: center;
        align-items: flex-start;
    }

    main {
        display: flex;
        align-items: center;
        flex-direction: column;
    }
</style>