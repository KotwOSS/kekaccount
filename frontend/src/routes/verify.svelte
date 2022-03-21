<script>
    import { goto } from "$app/navigation";
    import { APIError, get_identifier, get_login_redirect, Routes } from "../api";
    import { onMount } from "svelte";

    let error;

    onMount(()=>{
        let params = new URLSearchParams(window.location.search);
        let id = params.get("id");

        let uoe = localStorage.getItem("uoe");
        localStorage.removeItem("uoe");
        let password = localStorage.getItem("password");
        localStorage.removeItem("password");

        if(id) {
            if(uoe && password) {
                let identifier = get_identifier(uoe, password);
                Routes.Auth.VERIFY.send({
                    id,
                    identifier
                })
                .then(r=>goto("/dash"))
                .catch(e=>{
                    if(e instanceof APIError) {
                        console.log(e.get_message());
                        error = e.get_message();
                    }
                });
            } else {
                localStorage.setItem("creds", "true");
                goto(get_login_redirect());
            }
        } else goto("/dash");
    });
</script>

{#if error}
<h2>Error while verifying</h2>
<p class="error">{error}</p>
<a href="/dash">Dashboard</a>
{/if}

<style>
    @import url("../themes/default.css");

    a {
        text-decoration: none;
        color: aqua;
    }

    .error {
        color: rgb(255, 62, 62); 
        margin-bottom: 10px;
        word-wrap: break-word;
        word-break: break-all;
        line-break: strict;
    }
</style>