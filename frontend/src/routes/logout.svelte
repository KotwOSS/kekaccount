<script lang="ts">
    import { Routes } from "../api";

    import { onMount } from "svelte";
    import { goto } from "$app/navigation";


    onMount(async ()=>{
        let params = new URLSearchParams(window.location.search);

        let redirect = params.get("r")??"/login";

        let token = localStorage.getItem("token");
        if(token) {
            localStorage.removeItem("token");
            try {await Routes.Auth.Token.TERMINATE.send({token});} catch (e) {}
        }
        
        goto(redirect);
    });
</script>