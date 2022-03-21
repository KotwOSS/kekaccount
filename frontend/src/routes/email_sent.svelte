<script>
    import { goto } from '$app/navigation';

    function redirect_login(e) {
        e.preventDefault();
        goto("/login");
    }
</script>

<main class="root">
    <form class="inner">
        <h1>Verification email sent</h1>
        <p>Please check your inbox including the spam folder</p>
        <button on:click={redirect_login}>Login</button>
    </form>
</main>


<style>
    @import url("../themes/default.css");

    main {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    h1 {
        margin-bottom: 5px;
    }

    p {
        margin-bottom: 15px;
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
</style>