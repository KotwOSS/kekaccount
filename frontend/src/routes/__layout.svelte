<script lang="ts">
	import Loader from "$components/loader.svelte";
	import Navbar from "$components/navbar.svelte";
	import Footer from "$components/footer.svelte";
	import * as lang from "$lib/lang";
	import { client } from "$lib/client";
	import { Routes } from "$lib/api";
	import ApiError from "$components/error/api.svelte";

	let loading: boolean = true;
	let api_error: boolean = false;

	async function main() {
		await lang.init();

		Routes.PING.send({})
			.then(function () {
				let token = localStorage.getItem("token");
				if (token)
					client
						.login(token)
						.catch(function () {
							localStorage.removeItem("token");
						})
						.finally(function () {
							loading = false;
						});
				else loading = false;
			})
			.catch(function (e) {
				api_error = true;
			});
	}

	main();
</script>

<div id="app" class="fadein">
    <div class="wrapper">
        <main>
            {#if api_error}
                <ApiError />
            {:else if loading}
                <div class="content loading">
                    <Loader />
                </div>
            {:else}
                <Navbar />
                <div class="content">
                    <slot />
                </div>
                <Footer />
            {/if}
        </main>
    </div>
</div>

<style>
    .loading {
        display: flex;
        align-items: center; 
        justify-content: center;
        width: 100%;
        height: 100vh;
    }
    
	.loading > :global(.loader) {
		width: 50px;
		height: 50px;
	}

    main {
        min-height: 100%;
    }

    .wrapper {
        overflow: scroll;
        width: 100%;
        height: 100%;
    }
    
	#app {
		width: 100%;
        height: 100vh;
        overflow: hidden;
	}
</style>
