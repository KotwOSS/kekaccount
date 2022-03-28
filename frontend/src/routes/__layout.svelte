<script lang="ts">
	import Loader from "$components/loader.svelte";
	import Navbar from "$components/navbar.svelte";
	import Footer from "$components/footer.svelte";
	import lang from "$lib/lang";

	let loading: boolean = true;

	async function main() {
		await lang.init();
		loading = false;
	}

	main();

	function onscroll(e) {
		let scroll = e.target.scrollingElement.scrollTop;
		detach_navbar = scroll >= 50;
	}

	let detach_navbar = false;
</script>

<svelte:window on:scroll={onscroll}/>

<div id="app" class="fadein">
	{#if loading}
		<Loader />
	{:else}
    <!-- <div class="inner"> -->
        <Navbar detach={detach_navbar} />
		<slot />
		<Footer />
    <!-- </div>	 -->
	{/if}
</div>

<style>
	#app > :global(.loader) {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 50px;
		height: 50px;
	}

	#app {
        width: 100%;
	}
</style>
